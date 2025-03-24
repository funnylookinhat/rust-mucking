use std::io::{self, Read, Write};
use std::net::{TcpListener, TcpStream};

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8000")?;
    println!("Server listening on 127.0.0.1:8000");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr()?);
                if let Err(e) = handle_connection(stream) {
                    println!("Failed to handle connection: {}", e);
                }
            }
            Err(e) => {
                println!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_connection(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer)?;

    let response = "HTTP/1.1 200 OK\r\n\
                    Content-Type: text/plain\r\n\
                    Content-Length: 13\r\n\
                    Connection: close\r\n\
                    \r\n\
                    Hello, world!";

    stream.write_all(response.as_bytes())?;
    stream.flush()?; // Ensure data is sent

    Ok(())
}
