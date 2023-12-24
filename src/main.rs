use std::io;
use std::str;
use tokio::net::{TcpListener, TcpStream};
mod http_reply;
mod http_request;
use crate::http_reply::{get_reply, head_reply, post_reply, put_reply, reply_error};
use crate::http_request::{parse_request, HttpRequest};

async fn handle_request(raw_request: &str, stream: &TcpStream) {
    let request: HttpRequest = parse_request(raw_request).await;
    //12\n\nHello World!\n\0",
    match request.method {
        "GET" => get_reply(&stream, request).await,
        "POST" => post_reply(&stream, request).await,
        "PUT" => put_reply(&stream, request).await,
        "HEAD" => head_reply(&stream, request).await,
        _ => reply_error(&stream, request).await,
    }
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
