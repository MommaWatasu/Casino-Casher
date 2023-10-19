use std::{
    net::{
        TcpListener,
        TcpStream
    },
    io::prelude::*,
};

use json::JsonValue;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8000").expect("failed to bind port");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {handle_connection(stream)},
            Err(_) => { println!("Failed to get stream"); }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut body = JsonValue::Null;

    stream.read(&mut buffer).unwrap();
    let request = String::from_utf8_lossy(&buffer[..]);
    let request_lines: Vec<&str> = request.split("\n").collect();
    for i in 0..request_lines.len() {
        if request_lines[i] == "\r" {
            body = json::parse(&format!(r#"{}"#, request_lines[i+1])).expect("failed to parse body");
        }
    }
    let method = request_lines[0].split_whitespace().next();
    match method {
        Some("GET") => {
            handle_get_request(body);
        },
        Some("POST") => {},
        _ => { println!("Failed to parse") }
    }
}

fn handle_get_request(body: JsonValue) {
    println!("operation: {}", body["operation"]);
}