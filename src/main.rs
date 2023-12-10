use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};
use std::str;

fn handle_connection(mut stream: &TcpStream)  {
    let mut streamOpen = true;
    let mut buffer = [0; 512];
    stream.write(b"HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length: 12\n\nHello World!");
    println!("You are connected");
    while stream.read(&mut buffer).unwrap() != 0 {
        let buffer = str::from_utf8(&buffer).unwrap();
        println!("{}",buffer);
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.0:8088")?;
    for stream in listener.incoming() {
      match stream {
          Ok(stream) => {
              handle_connection(&stream);
          }
          Err(e) => {
              eprintln!("An error has ocurred: {}", e);
          }
      }
    }
    Ok(())
}
