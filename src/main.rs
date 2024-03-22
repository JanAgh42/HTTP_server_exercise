// Uncomment this block to pass the first stage
use std::{
    fs,
    env,
    thread,
    io::{Write, Read},
    net::{TcpListener, TcpStream}
};

const RESPONSE_OK: &str = "HTTP/1.1 200 OK";
const RESPONSE_404: &str = "HTTP/1.1 404 Not Found\r\n\r\n";

const CONTENT_TYPE_TEXT: &str = "text/plain";
const CONTENT_TYPE_OCTET: &str = "application/octet-stream";

const DIRECTORY_FLAG: &str = "--directory";

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    //
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    
    for stream in listener.incoming() {
        thread::spawn(move || match stream {
            Ok(_stream) => {
                handle_stream(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        });
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
    } else if path.starts_with("/files/") {
        return build_response_from_file(path);
    }
        
    RESPONSE_404.to_string().into_bytes()
}

fn build_response_from_path_content(path: &str) -> Vec<u8> {
    let payload = path.split("/echo/").collect::<Vec<&str>>()[1];
    let payload_length = payload.as_bytes().len();

    build_response(RESPONSE_OK, CONTENT_TYPE_TEXT, payload_length, payload)
}

fn build_response_from_user_agent(request_lines: Vec<&str>) -> Vec<u8> {
    let user_agent = request_lines.iter().find(|line| line.starts_with("User-Agent: ")).unwrap();
    let user_agent = user_agent.split("User-Agent: ").collect::<Vec<&str>>()[1];
    let user_agent_length = user_agent.as_bytes().len();

    build_response(RESPONSE_OK, CONTENT_TYPE_TEXT, user_agent_length, user_agent)
}

fn build_response_from_file(path: &str) -> Vec<u8> {
    let file_name = path.split("/files/").collect::<Vec<&str>>()[1];
    let args: Vec<String> = env::args().collect();

    if args.len() > 0 && args.contains(&DIRECTORY_FLAG.to_string()) {
        let flag_position = args.iter().position(|arg| arg == DIRECTORY_FLAG).unwrap();
        let dir_name = &args[flag_position + 1];

        return get_file_content(dir_name, file_name);
    }

    RESPONSE_404.to_string().into_bytes()
}

fn get_file_content(dir_name: &str, file_name: &str) -> Vec<u8> {
    let file_path = format!("{}/{}", dir_name, file_name);
    let file_content = fs::read_to_string(file_path);

    match file_content {
        Ok(content) => {
            let content_length = content.as_bytes().len();
            build_response(RESPONSE_OK, CONTENT_TYPE_OCTET, content_length, &content)
        }
        Err(_) => RESPONSE_404.to_string().into_bytes()
    }
}

fn build_response(response: &str, ctype: &str, length: usize, content: &str) -> Vec<u8> {
    format!("{}\r\nContent-type: {}\r\nContent-Length: {}\r\n\r\n{}\r\n",
        response, ctype, length, content).into_bytes()
}