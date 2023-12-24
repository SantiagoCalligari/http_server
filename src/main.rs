use std::io;
use std::str;
use tokio::net::{TcpListener, TcpStream};
mod http_reply;
mod http_request;
use crate::http_request::{parse_request, HttpRequest};

async fn handle_request(raw_request: &str, stream: &TcpStream) {
    let request: HttpRequest = parse_request(raw_request).await;
    stream.writable().await.unwrap();
    stream
        .try_write(b"HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length:")
        .unwrap();

    //12\n\nHello World!\n\0",
    match request.method {
        "GET" => b"24\n\nThis is a  POST request\n",
        "POST" => b"24\n\nThis is a  POST request\n",
        "PUT" => b"24\n\nThis is a  PUT request \n",
        "HEAD" => b"24\n\nThis is a  HEAD request\n",
        _ => b"24\n\nThis is nt implemented \n",
    };
}

async fn handle_connection(stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    loop {
        stream.readable().await?;
        match stream.try_read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => {
                handle_request(
                    str::from_utf8(&buffer).unwrap().trim_matches(char::from(0)),
                    &stream,
                )
                .await
            }
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
