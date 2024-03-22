// Uncomment this block to pass the first stage
use std::{
    borrow::Cow, env, fs, io::{Read, Write}, net::{TcpListener, TcpStream}, thread
};

const RESPONSE_OK: &str = "HTTP/1.1 200 OK";
const RESPONSE_404: &str = "HTTP/1.1 404 Not Found\r\n\r\n";
const RESPONSE_CREATED: &str = "HTTP/1.1 201 Created\r\n\r\n";

const CONTENT_TYPE_TEXT: &str = "text/plain";
const CONTENT_TYPE_OCTET: &str = "application/octet-stream";

const DIRECTORY_FLAG: &str = "--directory";

pub struct Request<'a> {
    method: String,
    path: String,
    body: String,
    request_lines: Vec<&'a str>,
}

impl<'a> Request<'a> {
    pub fn new(request: &'a Cow<'a, str>) -> Self {
        let request_lines: Vec<&str> = request.split("\r\n").collect();

        let method = request_lines[0]
            .split_whitespace()
            .collect::<Vec<&str>>()[0];
        
        let path = request_lines[0]
            .split_whitespace()
            .collect::<Vec<&str>>()[1];
    
        let body = request.split("\r\n\r\n").collect::<Vec<&str>>()[1].trim_end();

        Self {
            method: method.to_string(),
            path: path.to_string(),
            body: body.to_string(),
            request_lines,
        }
    }
}

fn main() {
    println!("Logs from your program will appear here!");

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
    let parsed_request = Request::new(&request);

    let response_bytes = match parsed_request.path.as_str() {
        "/" => format!("{}\r\n\r\n", RESPONSE_OK).into_bytes(),
        _ => get_response(&parsed_request)
    };

    stream.write(&response_bytes).unwrap();
}

fn get_response(parsed_request: &Request) -> Vec<u8> {
    if parsed_request.path.starts_with("/echo/") {
        return build_response_from_path_content(&parsed_request.path);
    } else if parsed_request.path.starts_with("/user-agent") {
        return build_response_from_user_agent(&parsed_request.request_lines);
    } else if parsed_request.path.starts_with("/files/") {
        return build_response_from_file(&parsed_request);
    }
        
    RESPONSE_404.to_string().into_bytes()
}

fn build_response_from_path_content(path: &String) -> Vec<u8> {
    let payload = path.split("/echo/").collect::<Vec<&str>>()[1];
    let payload_length = payload.as_bytes().len();

    build_response(RESPONSE_OK, CONTENT_TYPE_TEXT, payload_length, payload)
}

fn build_response_from_user_agent(request_lines: &Vec<&str>) -> Vec<u8> {
    let user_agent = request_lines.iter().find(|line| line.starts_with("User-Agent: ")).unwrap();
    let user_agent = user_agent.split("User-Agent: ").collect::<Vec<&str>>()[1];
    let user_agent_length = user_agent.as_bytes().len();

    build_response(RESPONSE_OK, CONTENT_TYPE_TEXT, user_agent_length, user_agent)
}

fn build_response_from_file(parsed_request: &Request) -> Vec<u8> {
    let file_name = parsed_request.path.split("/files/").collect::<Vec<&str>>()[1];
    let args: Vec<String> = env::args().collect();

    if args.len() > 0 && args.contains(&DIRECTORY_FLAG.to_string()) {
        let flag_position = args.iter().position(|arg| arg == DIRECTORY_FLAG).unwrap();
        let file_path = format!("{}/{}", &args[flag_position + 1], file_name);

        return match parsed_request.method.as_str() {
            "GET" => get_file_content(file_path),
            "POST" => post_file_content(file_path, &parsed_request.body),
            _ => RESPONSE_404.to_string().into_bytes()
        }
    }

    RESPONSE_404.to_string().into_bytes()
}

fn get_file_content(file_path: String) -> Vec<u8> {
    let file_content = fs::read_to_string(file_path);

    match file_content {
        Ok(content) => {
            let content_length = content.as_bytes().len();
            build_response(RESPONSE_OK, CONTENT_TYPE_OCTET, content_length, &content)
        }
        Err(_) => RESPONSE_404.to_string().into_bytes()
    }
}

fn post_file_content(file_path: String, body: &String) -> Vec<u8> {
    let mut file = fs::File::create(file_path).unwrap();

    file.write_all(body.as_bytes()).unwrap();

    RESPONSE_CREATED.to_string().into_bytes()
}

fn build_response(response: &str, ctype: &str, length: usize, content: &str) -> Vec<u8> {
    format!("{}\r\nContent-type: {}\r\nContent-Length: {}\r\n\r\n{}\r\n",
        response, ctype, length, content).into_bytes()
}