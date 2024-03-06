use mysql::{params, Pool, PooledConn};
use mysql::prelude::Queryable;
use crate::models::food::*;

pub trait Repository {
    fn get(&self, id: i32) -> Result<Food, ()>;
    fn get_all(&self) -> Result<Vec<Food>, ()>;
    fn save(&mut self, food: Food) -> Result<(), ()>;
    fn update(&self, id: i32, food: Food) -> Result<Food, ()>;
    fn delete(&mut self, id: i32) -> Result<(), ()>;
}

pub struct FoodRepository {
    conn: PooledConn
}

impl FoodRepository {
    pub fn new(conn: PooledConn) -> Self {
        Self { conn }
    }
}

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

    // TODO - get back inserted result
    // TODO - handle error
    fn save(&mut self, food: Food) -> Result<(), ()> {
        let result= self.conn.exec_drop(
            "INSERT INTO foods(name, ingestion, carbs, calories, proteins, electrolytes, comment) \
            VALUES(:name, :ingestion, :carbs, :calories, :proteins, :electrolytes, :comment)",
            params! {
                "name" => food.name,
                "ingestion" => "EAT",
                "carbs" => food.carbs,
                "calories" => food.calories,
                "proteins" => food.proteins,
                "electrolytes" => food.electrolytes,
                "comment" => food.comment
            }
        );
        println!("{}", self.conn.last_insert_id());

        Ok(result.unwrap())
    }

    // TODO
    fn update(&self, id: i32, food: Food) -> Result<Food, ()> {
        let food = Food {
            id: food.id,
            name: food.name,
            ingestion: food.ingestion,
            carbs: food.carbs,
            calories: food.calories,
            proteins: food.proteins,
            electrolytes: food.electrolytes,
            comment: food.comment
        };

        Ok(food)
    }

    // TODO - if id not found ?
    // TODO - Handle error
    fn delete(&mut self, id: i32) -> Result<(), ()> {
        let stmt = self.conn.prep("DELETE FROM foods WHERE id = :id").unwrap();
        self.conn.exec_drop(&stmt, params! {
            "id" => id,
        }).expect("TODO: panic message");
        Ok(())
    }
}

