use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::models::food::IngestionType;
use crate::dto::food_list_dto::FoodListDTO;
use crate::models::food::Food;
mod dto;
mod models;
mod repositories;

fn main() {
    let listener = TcpListener::bind(format!("0.0.0.0:8081")).unwrap();
    println!("Server listening on port 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("OK stream");
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
                r if r.starts_with("GET /health") => "get".to_string(),
                _ => "Not found".to_string()
            };
            stream.write_all(format!("{}", content).as_bytes()).unwrap();
        }
        Err(e) => eprintln!("Unable to read stream: {}", e),
    }
}
