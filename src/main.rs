use std::net::{TcpListener, TcpStream};

fn handle_connection(stream: &TcpStream) {
    let mut buffer = [0, 512];
    stream.write("HTTP/1.1 200 OK\nContent-Type: text/plain\nContent-Length: 12\n\nHello World!");
    println!("You are connected");
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.0:8088")?;
    for stream in listener.incoming() {
      match stream {
          Ok(stream) => {
              let mut stream = stream;
              handle_connection(&stream);
          }
          Err(e) => {
              eprintln!("An error has ocurred: {}", e);
          }
      }
    }
    Ok(())
}
