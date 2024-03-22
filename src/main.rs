// Uncomment this block to pass the first stage
use std::{
    io::{Write, Read},
    net::{TcpListener, TcpStream}
};

const RESPONSE_OK: &str = "HTTP/1.1 200 OK";
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
        "/" => format!("{}\r\n\r\n", RESPONSE_OK).into_bytes(),
        _ => get_response(path, request_lines)
    };

    stream.write(&response_bytes).unwrap();
}

fn get_response(path: &str, request_lines: Vec<&str>) -> Vec<u8> {
    if path.starts_with("/echo/") {
        return build_response_from_path_content(path);
    } else if path.starts_with("/user-agent") {
        return build_response_from_user_agent(request_lines);
    }
        
    RESPONSE_404.to_string().into_bytes()
}

fn build_response_from_path_content(path: &str) -> Vec<u8> {
    let payload = path.split("/echo/").collect::<Vec<&str>>()[1];
    let payload_length = payload.as_bytes().len();

    return format!("{}\r\nContent-type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n", RESPONSE_OK, payload_length, payload).into_bytes();
}

fn build_response_from_user_agent(request_lines: Vec<&str>) -> Vec<u8> {
    let user_agent = request_lines.iter().find(|line| line.starts_with("User-Agent: ")).unwrap();
    let user_agent = user_agent.split("User-Agent: ").collect::<Vec<&str>>()[1];
    let user_agent_length = user_agent.as_bytes().len();

    return format!("{}\r\nContent-type: text/plain\r\nContent-Length: {}\r\n\r\n{}\r\n", RESPONSE_OK, user_agent_length, user_agent).into_bytes();
}