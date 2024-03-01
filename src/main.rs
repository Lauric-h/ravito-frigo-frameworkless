use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::models::food::IngestionType;
use crate::models::food::Food;
use crate::repositories::food_repository;
use crate::repositories::food_repository::Repository;

mod models;
mod repositories;

const SERVER_ADDRESS: &str = "0.0.0.0";
const SERVER_PORT: &str = "8081";
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", SERVER_ADDRESS, SERVER_PORT).to_string()).unwrap();
    println!("Server listening on port {}", SERVER_PORT);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("GET /ping") => handle_ping_request(),
                r if r.starts_with("GET /foods/") => handle_get_request(r),
                r if r.starts_with("GET /foods") => handle_get_all_request(),
                _ => (NOT_FOUND_RESPONSE.to_string(), "404 Not found".to_string())
            };
            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}

fn handle_ping_request() -> (String, String) {
    (OK_RESPONSE.to_string(), String::from("pong"))
}

fn handle_get_all_request() -> (String, String) {
    let foods = food_repository::FoodRepository{}.get_all().unwrap();

    ((OK_RESPONSE).to_string(), serde_json::to_string(&foods).unwrap())
}

fn extract_id_from_request(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

fn handle_get_request(request: &str) -> (String, String) {
    let id = extract_id_from_request(request).parse::<i32>().unwrap();
    let food = food_repository::FoodRepository{}.get(id).unwrap();

    (OK_RESPONSE.to_string(), serde_json::to_string(&food).unwrap())
}

