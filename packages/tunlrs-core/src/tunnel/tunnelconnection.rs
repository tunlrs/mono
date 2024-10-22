use ::core::net::SocketAddr;
use ::tokio::net::{TcpSocket, TcpStream};

/* verify procedure for writing for server machine. */
type function_callback = Box<dyn Fn() + Send + Sync>;

pub struct TunnelConnection {
    client_stream: TcpStream,
    client_socket: SocketAddr,
    server_stream: TcpStream,
    server_socket: SocketAddr,
    on_connect: Option<function_callback>,
    on_client_read: Option<function_callback>,
    on_server_request: Option<function_callback>,
    on_server_response: Option<function_callback>,
    on_client_write: Option<function_callback>,
    on_disconnect: Option<function_callback>,
    active: bool,
    timeout: u32,
    /*
    Other stuff:
    - shutdown signal
    - TLS/SSL
    - encryption(?)
    */
}

impl TunnelConnection {
    pub fn new(
        client_stream: TcpStream,
        client_socket: SocketAddr,
        server_stream: TcpStream,
        server_socket: SocketAddr,
    ) -> Self {
        TunnelConnection {
            client_stream,
            client_socket,
            server_stream,
            server_socket,
            on_connect: None,
            on_client_read: None,
            on_server_request: None,
            on_server_response: None,
            on_client_write: None,
            on_disconnect: None,
            active: false,
            timeout: 1000,
        }
    }

    fn consume_callback_function(&mut self, func: Option<function_callback>) -> () {
        match func {
            Some(x) => x(),
            None => (),
        }
    }

    pub fn set_on_connect(&mut self) {}
    pub fn set_on_client_read(&mut self) {}
    pub fn set_on_server_request(&mut self) {}
    pub fn set_on_server_response(&mut self) {}
    pub fn set_on_client_write(&mut self) {}
    pub fn set_on_disconnect(&mut self) {}

    pub fn connect(&mut self) {
        let callback = self.on_connect.take();
        self.consume_callback_function(callback);
        /* do other connect stuff */
    }
    pub fn relay_to_server(&mut self) {
        let mut callback = self.on_client_read.take();
        self.consume_callback_function(callback);
        /* do other relay_to_server stuff */

        callback = self.on_server_request.take();
        self.consume_callback_function(callback);
        /* do stuff in between request and response */
        callback = self.on_server_response.take();
        self.consume_callback_function(callback);
        /* do stuff after getting client request */
    }
    pub fn reply_to_client(&mut self) {
        let callback = self.on_client_write.take();
        self.consume_callback_function(callback);
        /* write to client */
    }

    pub fn disconnect(&mut self) {
        let callback = self.on_disconnect.take();
        self.consume_callback_function(callback);
        /* do stuff to disconnect */
    }
}

/* @todo: unit testing of the TunnelConnection */
