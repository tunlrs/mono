use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;
use tokio::net::TcpStream;

use crate::tunnel::tunnelconnection::TunnelConnection;

const LISTEN_PORT: u16 = 5000;
const REPLY_PORT: u16 = 6000;

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
    let listener = TcpListener::bind("127.0.0.1:5000").await?;
    loop {
        let (mut stream, socket) = listener.accept().await?;
        let mut tunnel_connection_object =
            TunnelConnection::new(stream, socket, "127.0.0.1".to_string(), 5050);
        tokio::spawn(async move {
            tunnel_connection_object.connect().await;
            tunnel_connection_object.relay_to_server().await;
        });
    }
}

async fn tunnel_to_machine(msg: &[u8]) -> Result<(), Box<dyn std::error::Error>> {
    let mut tunnel_to_machine = TcpStream::connect("127.0.0.1:54180")
        .await
        .expect("TCP connection refused by server!");
    let message = msg;

    tunnel_to_machine.write(message).await.unwrap();
    println!("Wrote to tunnel!");
    Ok(())
}

// #[cfg(test)]
// mod test_listener {
//     use super::*;
//     #[test]
//     fn test_init_listener() {
//         let res = init_listener();
//         assert_eq!(res, ());
//     }
// }
