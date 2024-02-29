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

            let content = match &*request {
                r if r.starts_with("GET /health") => {
                    println!("Received request to /health");
                    "Health check OK".to_string()
                },
                _ => "Not found".to_string()
            };
            stream.write_all(format!("{}", content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}
