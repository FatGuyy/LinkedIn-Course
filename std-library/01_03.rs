use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Failed to read from connection: {}", e);
        return;
    }

    let request = String::from_utf8_lossy(&buffer[..]);
    let response = match get_response(&request) {
        Ok(content) => content,
        Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\n500 Internal Server Error".to_string(),
    };

    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write to connection: {}", e);
        return;
    }

    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush connection: {}", e);
    }
}

fn get_response(request: &str) -> Result<String, &'static str> {
    if request.contains("GET /index") {
        Ok(read_file("index.html"))
    } else if request.contains("GET /second") {
        Ok(read_file("second.html"))
    } else {
        Err("Route not found")
    }
}

fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
        Ok(content) => format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", content),
        Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\n500 Internal Server Error".to_string(),
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");

    println!("Server listening on 127.0.0.1:8080...");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
            Err(e) => eprintln!("Error: {}", e),
        }
    }
}
