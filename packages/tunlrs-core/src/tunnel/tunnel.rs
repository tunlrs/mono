use tokio::net::TcpListener;

use crate::tunnel::tunnelconnection::TunnelConnection;

const LISTEN_PORT: u16 = 5000;

pub fn init_tunnel() -> () {
    println!("Accessing the tunnel!");
    match tunnel_loop() {
        Ok(x) => x,
        Err(_) => (),
    }
    println!("Tunnel ready!");
}

#[tokio::main]
async fn tunnel_loop() -> Result<(), Box<dyn std::error::Error>> {
    let listener = TcpListener::bind(format!("127.0.0.1:{}", LISTEN_PORT).as_str()).await?;
    loop {
        let (stream, socket) = listener.accept().await?;
        let mut tunnel_connection_object =
            TunnelConnection::new(stream, socket, "127.0.0.1".to_string(), 5050);
        tokio::spawn(async move {
            tunnel_connection_object.connect().await;
            tunnel_connection_object.relay_to_server().await;
        });
    }
}

