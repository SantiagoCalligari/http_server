use std::io;
use std::str;
use tokio::net::{TcpListener, TcpStream};
mod http_request;
use crate::http_request::HttpRequest;

async fn handle_request(raw_request: &str, stream: &TcpStream) {
    let request: HttpRequest = http_request::parse_request(raw_request).await;
    stream.writable().await.unwrap();
    stream
        .try_write(b"HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length:")
        .unwrap();

    //12\n\nHello World!\n\0",
    let response = match &request.method.unwrap()[..] {
        "GET" => b"21\n\nThis is a GET request\n\0 ",
        "POST" => b"22\n\nThis is a POST request\n\0",
        "PUT" => b"21\n\nThis is a PUT request\n\0 ",
        "HEAD" => b"22\n\nThis is a HEAD request\n\0",
        _ => b"22\n\nThis is nt implemented\n\0",
    };

    stream.writable().await.unwrap();
    stream.try_write(response).unwrap();
}

async fn handle_connection(stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    loop {
        stream.readable().await?;
        match stream.try_read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => handle_request(str::from_utf8(&buffer).unwrap(), &stream).await,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
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
        handle_connection(socket).await.unwrap_or_else(|e| {
            println!("Error: {}", e);
        });
    }
}
