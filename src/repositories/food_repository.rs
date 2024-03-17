use mysql::{params, Pool, PooledConn};
use mysql::prelude::Queryable;
use crate::models::food::*;

pub trait Repository {
    fn get(&mut self, id: i32) -> Result<Food, ()>;
    fn get_all(&mut self) -> Result<Vec<Food>, ()>;
    fn save(&mut self, food: Food) -> Result<(), ()>;
    fn update(&mut self, id: i32, food: Food) -> Result<(), ()>;
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
    // TODO - handle errors
    // TODO - handle id not found
    // TODO - handle ingestion
    fn get(&mut self, id: i32) -> Result<Food, ()> {
        let res = self.conn.exec_first(
            "SELECT id, name, carbs, calories, proteins, electrolytes, comment FROM foods WHERE id = :id",
            params! {
                "id" => id
            }
        ).map(|row| {
            row.map(|(id, name, carbs, calories, proteins, electrolytes, comment)| Food {
                id,
                name,
                ingestion: IngestionType::EAT,
                carbs,
                calories,
                proteins,
                electrolytes,
                comment
            }).unwrap()
        });

       Ok(res.unwrap())
    }

    // TODO - handle errors
    // TODO - handle Ingestion
    fn get_all(&mut self) -> Result<Vec<Food>, ()> {
        let res = self.conn.query_map(
            "SELECT id, name, carbs, calories, proteins, electrolytes, comment FROM foods",
            |(id, name, carbs, calories, proteins, electrolytes, comment)|
                Food {
                    id,
                    name,
                    ingestion: IngestionType::EAT,
                    carbs,
                    calories,
                    proteins,
                    electrolytes,
                    comment
                }
        ).expect("Query failed");

        Ok(res)
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

    // TODO - return saved food
    // TODO - handle errors
    // TODO - handle id not found
    fn update(&mut self, id: i32, food: Food) -> Result<(), ()> {
        let stmt = self.conn.prep("UPDATE FOODS SET name = :name, carbs = :carbs, calories = :calories, proteins = :proteins, electrolytes = :electrolytes, comment = :comment WHERE id = :id").unwrap();
        let result = self.conn.exec_drop(&stmt, params! {
            "id" => id,
            "name" => food.name,
            "ingestion" => "EAT",
            "carbs" => food.carbs,
            "calories" => food.calories,
            "proteins" => food.proteins,
            "electrolytes" => food.electrolytes,
            "comment" => food.comment
        });

        Ok(result.unwrap())
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

