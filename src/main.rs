use crate::models::food::IngestionType;
use crate::dto::food_list_dto::FoodListDTO;
use crate::models::food::Food;
mod dto;
mod models;
mod repositories;

fn main() {
    println!("Hello, world!");
    let c: FoodListDTO = FoodListDTO {
        foods: vec![],
    };

    let f: Food = Food {
        id: 0,
        name: "".to_string(),
        ingestion: IngestionType::EAT,
        carbs: 0,
        calories: 0,
        proteins: 0,
        electrolytes: false,
        comment: "".to_string(),
    };
}
