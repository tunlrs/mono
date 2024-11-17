use ::std::net::SocketAddr;
use ::tokio::net::{TcpSocket, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

/* verify procedure for writing for server machine. */
type function_callback = Box<dyn Fn(&TunnelConnection) + Send + Sync>;

pub struct TunnelConnection {
    client_stream: TcpStream,
    client_socket: SocketAddr,
    server_address: String,
    server_port: u16,
    server_stream: Option<TcpStream>,
    server_socket: Option<SocketAddr>,
    on_connect: Option<function_callback>,
    on_client_read: Option<function_callback>,
    on_server_request: Option<function_callback>,
    on_server_response: Option<function_callback>,
    on_client_write: Option<function_callback>,
    on_disconnect: Option<function_callback>,
    active: bool,
    timeout: u32,
    has_raised_error: bool,
    error_message: String,
    /*
    Other stuff:
    - shutdown signal
    - TLS/SSL
    - encryption(?)
    */
}

/*
    expose
    TunnelManager
        - values()
        - ready, waiting for server
        - vec[vec[OBJ, OBJ], 1: WAITIN]
        - TunnelConnection
*/

impl TunnelConnection {
    pub fn new(
        client_stream: TcpStream,
        client_socket: SocketAddr,
        server_address: String,
        server_port: u16,
    ) -> Self {
        TunnelConnection {
            client_stream,
            client_socket,
            server_address,
            server_port,
            server_stream: None,
            server_socket: None,
            on_connect: None,
            on_client_read: None,
            on_server_request: None,
            on_server_response: None,
            on_client_write: None,
            on_disconnect: None,
            active: false,
            timeout: 1000,
            has_raised_error: false,
            error_message: "".to_string(),
            /* find out a better method for
            containing diagnostics data - maybe another
            struct? */
        }
    }

    fn consume_callback_function(&mut self, func: Option<function_callback>) -> () {
        match func {
            Some(x) => x(&self),
            None => (),
        }
    }

    pub fn set_on_connect(&mut self) {}
    pub fn set_on_client_read(&mut self) {}
    pub fn set_on_server_request(&mut self) {}
    pub fn set_on_server_response(&mut self) {}
    pub fn set_on_client_write(&mut self) {}
    pub fn set_on_disconnect(&mut self) {}

    pub async fn connect(&mut self) {
        if self.has_raised_error {
            return;
        }

        /* If we fail to get a connection with the server, then
        reading from the client's TCP pipe is useless - so we attempt
        to get a connection with the server first. */
        let connection_stream =
            TcpStream::connect(format!("{}:{}", self.server_address, self.server_port)).await;
        if let Ok(connection) = connection_stream {
            self.server_stream = Some(connection);
            println!(
                "OK: Connected @ {}",
                format!("{}:{}", self.server_address, self.server_port)
            );
        } else {
            self.has_raised_error = true;
            println!(
                "ERROR: Server refused connection - {}:{}.\nThis connection will be refuted.",
                self.server_address, self.server_port
            );
            return;
        }
        let callback = self.on_connect.take();
        self.consume_callback_function(callback);
    }

    pub async fn relay_to_server(&mut self) {
        if self.has_raised_error {
            return;
        }

        /* === READ FROM CLIENT === */

        let mut client_req_buf = [0 as u8; 4096];
        let n_bytes_read = self
            .client_stream
            .read(&mut client_req_buf)
            .await
            .expect("");

        if n_bytes_read == 0 {
            println!("Empty pipe from client!");
            return;
        }

        let client_read_callback = self.on_client_read.take();
        self.consume_callback_function(client_read_callback);

        /* === FORWARD TO HOST === */

        let server_stream = Option::expect(self.server_stream.as_mut(), "");
        let (_, mut write_head) = server_stream.split();
        print!("Writing: {:?}\n", &client_req_buf[0..n_bytes_read]);
        let nb = write_head
            .write(&client_req_buf[0..n_bytes_read])
            .await
            .unwrap();
        println!("Bytes written: {:?}", nb);

        let server_req_callback = self.on_server_request.take();
        self.consume_callback_function(server_req_callback);

        self.reply_to_client().await;
    }

    pub async fn reply_to_client(&mut self) {
        /* === READ FROM HOST === */

        let server_stream = Option::expect(self.server_stream.as_mut(), "");
        let (mut read_head, _) = server_stream.split();

        let mut server_res_buf = [0 as u8; 4096];
        let n_bytes_read = read_head.read(&mut server_res_buf).await.unwrap();

        if n_bytes_read == 0 {
            println!("Empty pipe from server!");
            return;
        }
        println!("Read from server: {:?}", &server_res_buf[0..n_bytes_read]);

        let server_res_callback = self.on_server_response.take();
        self.consume_callback_function(server_res_callback);

        /* === REPLY TO CLIENT === */

        let client_stream = &mut self.client_stream;
        let (_, mut client_write_head) = client_stream.split();

        let n_bytes_written = client_write_head
            .write(&server_res_buf[0..n_bytes_read])
            .await
            .unwrap();
        let callback = self.on_client_write.take();
        self.consume_callback_function(callback);
    }

    pub async fn disconnect(&mut self) {
        let callback = self.on_disconnect.take();
        self.consume_callback_function(callback);
        /* do stuff to disconnect */
    }
}

/* @todo: unit testing of the TunnelConnection */
