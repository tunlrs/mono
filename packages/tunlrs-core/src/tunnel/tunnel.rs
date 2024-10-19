use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

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
        tokio::spawn(async move {
            let mut buf = [0; 4096];

            loop {
                let n = stream
                    .read(&mut buf)
                    .await
                    .expect("Had an error in reading!");

                if n == 0 {
                    return;
                }

                println!("Got message: {:?} @ {:?}", &buf[0..n], socket);

                if let Err(e) = stream.write_all(&buf[0..n]).await {
                    eprintln!("The error was: {:?}", e);
                }
            }
        });
    }
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
