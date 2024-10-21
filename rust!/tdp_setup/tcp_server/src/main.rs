use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 512]; 
    while true {
        match stream.read(&mut buffer) {
            Ok(bytes_read) => {
                let rec = String::from_utf8_lossy(&buffer[..bytes_read]);
                if rec.trim().is_empty() {
                    println!("Connection closed!");
                    let response = b"Connection closed!\n";
                    if let Err(e) = stream.write_all(response) {
                        eprintln!("Failed to send response: {}", e);
                    }
                    return;
                }

                println!("Received: {}", rec);
                let response = b"Message received\n";
                if let Err(e) = stream.write_all(response) {
                    eprintln!("Failed to send response: {}", e);
                }
            }
            
            Err(e) => {
                eprintln!("Failed to read from stream: {}", e);
                return;
            }
        }
    }
}

fn main() -> std::io::Result<()> {
    // connect with nc 127.0.0.1 7878
    let listener = TcpListener::bind("127.0.0.1:7878")?;

    println!("Server listening on 127.0.0.1:7878");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New client connected");

                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
