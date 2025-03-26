use std::error::Error;

use rust_chat::Commands;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;

    loop {
        let (stream, _) = listener.accept().await?;
        tokio::spawn(async move {
            if let Err(e) = handle_connection(stream).await {
                eprintln!("Error handling connection {}", e);
            }
        });
    }
}

async fn handle_connection(mut stream: TcpStream) -> Result<(), Box<dyn Error>> {
    let mut buf = [0; 1024];

    loop {
        let n = stream.read(&mut buf).await?;
        if n == 0 {
            return Ok(());
        }
        println!("server received {:?}", &buf[..n]);

        let command: Commands = match serde_json::from_slice(&buf[..n]) {
            Ok(cmd) => cmd,
            Err(e) => {
                eprintln!("error deserializing {}", e);
                return Err(Box::new(e));
            }
        };

        println!("server received {:?}", command);

        let response: Commands = Commands::Test {
            value: "success".to_string(),
        };
        let bytes = serde_json::to_vec(&response).unwrap();

        stream.write_all(&bytes).await?;
    }
}
