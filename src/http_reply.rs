use crate::http_request::HttpRequest;
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use tokio::net::TcpStream;

async fn write_to_stream(stream: &TcpStream, msg: &[u8]) {
    println!("{}", std::str::from_utf8(msg).unwrap());
    stream.writable().await.unwrap();
    stream.try_write(msg).unwrap();
}

async fn write_file(stream: &TcpStream, path: &str) {
    let path = Path::new(path);
    let display = path.display();
    println!("{}", display);
    let extension = path.extension().unwrap().to_str().unwrap();
    let mut file = File::open(path).unwrap();
    let size = file.metadata().unwrap().len();

    match extension {
        "html" => {
            write_to_stream(
                &stream,
                b"HTTP/1.1 200 OK\nContent-Type: text/html; charset=UTF-8\nContent-Length: ",
            )
            .await
        }

        "css" => {
            write_to_stream(
                &stream,
                b"HTTP/1.1 200 OK\nContent-Type: text/css; charset=UTF-8\nContent-Length: ",
            )
            .await
        }

        _ => {
            write_to_stream(
                &stream,
                b"HTTP/1.1 200 OK\nContent-Type: text/plain; charset=UTF-8\nContent-Length: ",
            )
            .await
        }
    }
    let mut file_lines = String::new();
    match file.read_to_string(&mut file_lines) {
        Err(why) => println!("Couldn't read {}: {}", display, why),
        Ok(_) => {
            write_to_stream(&stream, size.to_string().as_bytes()).await;
            write_to_stream(&stream, b"\n\n").await;
            write_to_stream(&stream, file_lines.as_bytes()).await;
        }
    };
}

pub async fn get_reply(stream: &TcpStream, request: HttpRequest<'_>) {
    let path = Path::new(request.path);
    if path.exists() {
        write_file(&stream, request.path).await;
    } else {
        write_to_stream(
            &stream,
            b"HTTP/1.1 404 Not Found\nContent-Type: text/plain\nContent-Length: 10\n\nNot Found\n",
        )
        .await;
    }
}

pub async fn post_reply(stream: &TcpStream, _request: HttpRequest<'_>) {
    todo!();
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
