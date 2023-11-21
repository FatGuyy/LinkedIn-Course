use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

// Function to handle an incoming client connection
fn handle_client(mut stream: TcpStream) {
    // Buffer to store incoming data
    let mut buffer = [0; 1024];

    // Read data from the client into the buffer
    if let Err(e) = stream.read(&mut buffer) {
        eprintln!("Failed to read from connection: {}", e);
        return;
    }

    // Convert the buffer to a UTF-8 string
    let request = String::from_utf8_lossy(&buffer[..]);

    // Check if the request contains "GET /api/message"
    if request.contains("GET /api") {
        // Respond with a JSON message
        let response = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n{\"message\": \"Hello, Rust API!\"}";

        // Write the response to the client
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write to connection: {}", e);
            return;
        }
    } else {
        // Respond with a 404 Not Found error
        let response = "HTTP/1.1 404 Not Found\r\nContent-Type: text/plain\r\n\r\nNot Found";

        // Write the response to the client
        if let Err(e) = stream.write_all(response.as_bytes()) {
            eprintln!("Failed to write to connection: {}", e);
            return;
        }
    }

    // Flush the output stream
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush connection: {}", e);
    }
}

// Main function to start the server
fn main() {
    // Bind the server to the specified address and port
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");

    // Print a message indicating that the server is listening
    println!("Server listening on 127.0.0.1:8080...");

    // Accept incoming client connections and spawn a new thread to handle each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle the client connection
                std::thread::spawn(|| {
                    // Call the handle_client function to process the client request
                    handle_client(stream);
                });
            }
            Err(e) => {
                // Print an error message if there's an issue with the incoming connection
                eprintln!("Error: {}", e);
            }
        }
    }
}
