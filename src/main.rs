// Uncomment this block to pass the first stage
use std::{
    io::{Write, Read},
    net::{TcpListener, TcpStream}
};

const RESPONSE_OK: &str = "HTTP/1.1 200 OK\r\n\r\n";
const RESPONSE_404: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                handle_stream(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_stream(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    stream.read(&mut buffer).unwrap();
    
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_lines: Vec<&str> = request.split("\r\n").collect();
    
    let path = request_lines[0]
        .split_whitespace()
        .collect::<Vec<&str>>()[1];

    let response_bytes = match path {
        "/" => RESPONSE_OK.as_bytes(),
        _ => RESPONSE_404.as_bytes()
    };

    stream.write(response_bytes).unwrap();
}