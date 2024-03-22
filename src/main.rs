// Uncomment this block to pass the first stage
use std::net::{TcpListener, TcpStream};

let RESPONSE_OK = "HTTP/1.1 200 OK\r\n\r\n";

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    let response_ok = "HTTP/1.1 200 OK\r\n\r\n";
    
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
    let response_bytes = response_ok.as_bytes();
    stream.write(response_bytes).unwrap();
}