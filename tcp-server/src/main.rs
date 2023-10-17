use std::io::{ Read, Write };
use std::net::{ TcpListener, TcpStream };

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).expect("Failed to read from client");

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Received request: {}", request);

    let reponse = "Hello, Client!!".as_bytes();
    stream.write(reponse).expect("Failed to write response!!");
}
fn main() {
    let listener = TcpListener::bind("0.0.0.0:8080").expect("Failed to bind to address");
    println!("Server listening on 0.0.0.0:8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                std::thread::spawn(|| handle_client(stream));
            }
            Err(e) => {
                eprintln!("Failed to establish connection {}", e);
            }
        }
    }
}
