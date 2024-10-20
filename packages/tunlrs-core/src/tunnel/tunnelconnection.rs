use ::core::net::SocketAddr;
use ::tokio::net::{TcpSocket, TcpStream};

/* verify procedure for writing for server machine. */
pub struct TunnelConnection {
    client_stream: TcpStream,
    client_socket: SocketAddr,
    server_stream: TcpStream,
    server_socket: SocketAddr,
    on_connect: Option<Box<dyn Fn() + Send + Sync>>,
    on_client_read: Option<Box<dyn Fn() + Send + Sync>>,
    on_server_request: Option<Box<dyn Fn() + Send + Sync>>,
    on_server_response: Option<Box<dyn Fn() + Send + Sync>>,
    on_client_write: Option<Box<dyn Fn() + Send + Sync>>,
    on_disconnect: Option<Box<dyn Fn() + Send + Sync>>,
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

    pub fn set_on_connect(&mut self) {}
    pub fn set_on_client_read(&mut self) {}
    pub fn set_on_server_request(&mut self) {}
    pub fn set_on_server_response(&mut self) {}
    pub fn set_on_client_write(&mut self) {}
    pub fn set_on_disconnect(&mut self) {}

    pub fn connect() {}
    pub fn relay_to_server() {}
    pub fn reply_to_client() {}
    pub fn disconnect() {}
}

/* @todo: unit testing of the TunnelConnection */
