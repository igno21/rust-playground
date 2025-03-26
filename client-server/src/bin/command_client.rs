use std::{env, error::Error, net::SocketAddr};

use rust_chat::Commands;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    let addr = env::args()
        .nth(1)
        .unwrap_or_else(|| panic!("this program requires at least one argument"));

    let addr = addr.parse::<SocketAddr>()?;

    let mut stream = TcpStream::connect(&addr).await?;

    let command = Commands::Test {
        value: "Test".to_string(),
    };
    let bytes = serde_json::to_vec(&command)?;
    stream.write_all(&bytes).await?;

    let mut buf = [0; 1024];
    let n = stream.read(&mut buf).await?;

    let response: Commands = serde_json::from_slice(&buf[..n]).unwrap();
    println!("repsonse: {:?}", response);
    Ok(())
}
