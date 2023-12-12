use std::io;
use std::str;
use tokio::net::{TcpListener, TcpStream};
mod http_request;

async fn handle_request(request: &str) {
    let mut request_lines = request.lines();
    let (method, tail) = http_request::get_method(&request_lines.next().unwrap()).await;
    match method {
        Some(request) => println!("The request is:{:?}, the tail is:{}", request, tail),
        None => println!("The request is not implemented"),
    }
}

async fn handle_connection(stream: TcpStream) -> std::io::Result<()> {
    let mut buffer = [0; 512];
    loop {
        stream.readable().await?;
        match stream.try_read(&mut buffer) {
            Ok(0) => break,
            Ok(_) => handle_request(str::from_utf8(&buffer).unwrap()).await,
            Err(ref e) if e.kind() == io::ErrorKind::WouldBlock => {
                continue;
            }
            Err(e) => {
                return Err(e.into());
            }
        }
        /*
               stream.writable().await?;
               stream.try_write(
                   b"HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length: 12\n\nHello World!\n\0",
               )?;
        */
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
