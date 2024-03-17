use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use crate::models::food::Food;
use crate::repositories::food_repository::{FoodRepository, Repository};

mod models;
mod repositories;
mod database;

const SERVER_ADDRESS: &str = "0.0.0.0";
const SERVER_PORT: &str = "8081";
const OK_RESPONSE: &str = "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\n\r\n";
const CREATED_RESPONSE: &str = "HTTP/1.1 201 CREATED\r\nContent-Type: application/json\r\n\r\n";
const NOT_FOUND_RESPONSE: &str = "HTTP/1.1 404 NOT FOUND\r\n\r\n";

fn main() {
    let listener = TcpListener::bind(format!("{}:{}", SERVER_ADDRESS, SERVER_PORT).to_string()).unwrap();
    println!("Server listening on port {}", SERVER_PORT);

    let db_conn = database::database::connect_db();
    let mut repository = FoodRepository::new(db_conn);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_client(stream, &mut repository);
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
}

fn handle_client(mut stream: TcpStream, repository: &mut FoodRepository) {
    let mut buffer = [0; 1024];
    let mut request = String::new();

    match stream.read(&mut buffer) {
        Ok(size) => {
            request.push_str(String::from_utf8_lossy(&buffer[..size]).as_ref());

            let (status_line, content) = match &*request {
                r if r.starts_with("GET /ping") => handle_ping_request(),
                r if r.starts_with("GET /foods/") => handle_get_request(repository, r),
                r if r.starts_with("PUT /foods/") => handle_put_request(repository, r),
                r if r.starts_with("GET /foods") => handle_get_all_request(repository),
                r if r.starts_with("POST /foods") => handle_post_request(repository, r),
                r if r.starts_with("DELETE /foods/") => handle_delete_request(repository, r),
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

fn handle_get_all_request(repository: &mut FoodRepository) -> (String, String) {
    let foods = repository.get_all().unwrap();

    ((OK_RESPONSE).to_string(), serde_json::to_string(&foods).unwrap())
}

fn extract_id_from_request(request: &str) -> &str {
    request.split("/").nth(2).unwrap_or_default().split_whitespace().next().unwrap_or_default()
}

fn handle_get_request(repository: &mut FoodRepository, request: &str) -> (String, String) {
    let id = extract_id_from_request(request).parse::<i32>().unwrap();
    let food = repository.get(id).unwrap();

    (OK_RESPONSE.to_string(), serde_json::to_string(&food).unwrap())
}

fn extract_request_body(request: &str) -> Result<Food, serde_json::Error> {
    // Divide request between headers and body
    serde_json::from_str(request.split("\r\n\r\n").last().unwrap_or_default())
}

// TODO - handle error
// TODO - handle correct response from repo
fn handle_post_request(repository: &mut FoodRepository, request: &str) -> (String, String) {
    let food = extract_request_body(request).unwrap();
    repository.save(food).expect("TODO: panic message");

    // TODO
    // (CREATED_RESPONSE.to_string(), serde_json::to_string(&saved_food).unwrap())
    (CREATED_RESPONSE.to_string(), "Ok".to_string())
}

fn handle_put_request(repository: &mut FoodRepository, request: &str) -> (String, String) {
    let id = extract_id_from_request(request).parse::<i32>().unwrap();
    let food = extract_request_body(request).unwrap();

    let saved_food = repository.update(id, food).unwrap();

    (OK_RESPONSE.to_string(), serde_json::to_string(&saved_food).unwrap())
}

// TODO - handle error
fn handle_delete_request(repository: &mut FoodRepository, request: &str) -> (String, String) {
    let id = extract_id_from_request(request).parse::<i32>().unwrap();
    repository.delete(id).expect("nope");

    (OK_RESPONSE.to_string(), "OK".to_string())
}

