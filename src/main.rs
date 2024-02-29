use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::models::food::IngestionType;
use crate::dto::food_list_dto::FoodListDTO;
use crate::models::food::Food;
mod dto;
mod models;
mod repositories;

const SERVER_ADDRESS: &str = "0.0.0.0";
const SERVER_PORT: &str = "8081";
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", SERVER_ADDRESS, SERVER_PORT).to_string()).unwrap();
    println!("Server listening on port {}", SERVER_PORT);


    let food = Food {
        id: 0,
        name: "".to_string(),
        ingestion: IngestionType::EAT,
        carbs: 0,
        calories: 0,
        proteins: 0,
        electrolytes: false,
        comment: "".to_string(),
    };

    let s = serde_json::to_string(&food);
    println!("{:?}", s);

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
                r if r.starts_with("GET /ping") => handle_ping(),
                _ => (NOT_FOUND_RESPONSE.to_string(), "404 Not found".to_string())
            };
            stream.write_all(format!("{}{}", status_line, content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}

fn handle_ping() -> (String, String) {
    (OK_RESPONSE.to_string(), String::from("pong"))
}
