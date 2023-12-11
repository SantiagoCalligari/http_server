use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt, AsyncReadExt};
use std::str;
use std::io;

async fn handle_connection(mut stream: TcpStream) -> std::io::Result<()>{
    let mut buffer = [0; 1024];
    stream.writable().await?;
    stream.try_write(b"HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length: 12\n\nHello World!")?;
    loop {
        stream.readable().await?;
        match stream.try_read(&mut buffer) {
            Ok(0) => break,
            Ok(n) => {
                println!("{:?}",n);
                println!("{}", str::from_utf8(&buffer).unwrap());
            }
            Err(ref e ) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("192.168.100.224:6969").await.unwrap();
    loop {
        let (socket, _) = listener.accept().await.unwrap();
        handle_connection(socket).await.unwrap_or_else( |e| {
            println!("Error: {}", e);
        });
    }
}
