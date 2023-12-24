use crate::http_request::HttpRequest;
use std::fs;
use std::io::Result;
use std::path::Path;
use tokio::net::TcpStream;

async fn write_file(stream: &TcpStream, path: &str) {
    println!("{}", path);
    let path = Path::new(path);
    if path.exists() {
        stream.writable().await.unwrap();
        stream
            .try_write(b"HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length:")
            .unwrap();
    } else {
        stream.writable().await.unwrap();
        stream
            .try_write(b"HTTP/1.1 404 Not Found\nContent-Type: text/plain\nContent-Length:10\n\nNot Found\n")
            .unwrap();
    }
}

pub async fn get_reply(stream: &TcpStream, request: HttpRequest<'_>) {
    write_file(&stream, request.path).await;
}

pub async fn post_reply(stream: &TcpStream, _request: HttpRequest<'_>) {
    stream
        .try_write(b"HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length:")
        .unwrap();
    stream.writable().await.unwrap();
    stream.try_write(b"12\n\nHello PUT \n").unwrap();
}
pub async fn head_reply(_stream: &TcpStream, _request: HttpRequest<'_>) {
    todo!();
}
pub async fn put_reply(_stream: &TcpStream, _request: HttpRequest<'_>) {
    todo!();
}
pub async fn reply_error(_stream: &TcpStream, _request: HttpRequest<'_>) {
    todo!();
}
