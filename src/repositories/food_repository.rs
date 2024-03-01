use crate::models::food::*;

pub trait Repository {
    fn get(&self, id: i32) -> Result<Food, ()>;
    fn get_all(&self) -> Result<Vec<Food>, ()>;
    fn save(&self, food: Food) -> Result<Food, ()>;
    fn delete(&self, id: i32) -> Result<(), ()>;
}

pub struct FoodRepository {}

impl Repository for FoodRepository {
    fn get(&self, id: i32) -> Result<Food, ()> {
        let food = Food {
            id: id.clone(),
            name: "get".to_string(),
            ingestion: IngestionType::EAT,
            carbs: 1,
            calories: 1,
            proteins: 1,
            electrolytes: false,
            comment: "get".to_string(),
        };

        Ok(food)
    }

    fn get_all(&self) -> Result<Vec<Food>, ()> {
        let mut foods = Vec::new();

        for i in 1..=5 {
            foods.push(
            Food {
                id: i,
                name: i.to_string(),
                ingestion: IngestionType::EAT,
                carbs: i,
                calories: i,
                proteins: i,
                electrolytes: false,
                comment: i.to_string(),
            });
        }

        Ok(foods)
    }

    fn save(&self, food: Food) -> Result<Food, ()> {
        let food = Food {
            id: 1,
            name: "save".to_string(),
            ingestion: IngestionType::EAT,
            carbs: 1,
            calories: 1,
            proteins: 1,
            electrolytes: false,
            comment: "save".to_string(),
        };

        Ok(food)
    }

    fn delete(&self, id: i32) -> Result<(), ()> {
        Ok(())
    }
}

