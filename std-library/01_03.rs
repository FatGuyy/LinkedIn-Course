// import required 
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::fs;

// Function to read the content of a file and format an HTTP response
fn read_file(filename: &str) -> String {
    match fs::read_to_string(filename) {
                                // This is a raw html response in raw text format, with html file passed in as string
                                // HTTP/1.1 200 OK - status line
                                // "\r\n" - required new line to separate header from body
                                // Content-Type: text/html - HTTP header telling the type of content
        Ok(content) => format!("HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n{}", content),
                    // If there is an error, we return the error html response
                    // "HTTP/1.1 500 Internal Server Error" - This is the status line
                    // "\r\n\r\n" - sequence of characters representing a carriage return
                    // "500 Internal Server Error" - response message
        Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\n500 Internal Server Error".to_string(),
        }
}

// Function to determine the response based on the request
fn get_response(request: &str) -> Result<String, &'static str> {
    // Check if the request contains "/"
    if request.contains("GET /") {
        // If yes, read the content of "index.html" and format an HTTP response
        Ok(read_file("index.html"))
    } 
    // Check if the request contains "/second"
    else if request.contains("GET /second") {
        // If yes, read the content of "second.html" and format an HTTP response
        Ok(read_file("second.html"))
    }
    // If no matching route is found, return an error
    else {
        Err("Route not found")
    }
}

// Function to handle each client connection
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

    // Generate a response based on the request
    let response = match get_response(&request) {
        Ok(content) => content,
        Err(_) => "HTTP/1.1 500 Internal Server Error\r\n\r\n500 Internal Server Error".to_string(),
    };

    // Write the response back to the client
    if let Err(e) = stream.write_all(response.as_bytes()) {
        eprintln!("Failed to write to connection: {}", e);
        return;
    }

    // Flush the output stream
    if let Err(e) = stream.flush() {
        eprintln!("Failed to flush connection: {}", e);
    }
}

// Main function where the server binds to an address and listens for incoming connections
fn main() {
    // make a new tcp_listener which is binded to a ip address and its port number
    // expect make sures that if the port or ip is not available, we handle the error
    let listener = TcpListener::bind("127.0.0.1:8080").expect("Failed to bind");

    println!("Server listening on 127.0.0.1:8080...");

    // Accept incoming connections and spawn a new thread for each
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                // Spawn a new thread to handle each client connection
                thread::spawn(|| {
                    handle_client(stream);
                });
            }
                // If there is any error in accepting the connection, the error is printed
            Err(e) => eprintln!("Error accepting connection: {}", e),
        }
    }
}
