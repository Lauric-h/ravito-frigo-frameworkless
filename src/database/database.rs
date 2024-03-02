use std::process;
use mysql::*;
use mysql::Pool;
use mysql::prelude::Queryable;

pub fn connect_db() -> PooledConn {
    // TODO
    // Use ENV var
    let url = "mysql://root:root@127.0.0.1/frigo";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    if let Err(err) = create_database(&mut conn) {
        eprintln!("Failed to create database: {}", err);
        process::exit(1);
    }

    if let Err(err) = create_table(&mut conn) {
        eprintln!("Failed to create table: {}", err);
        process::exit(1);
    }

    conn
}

fn create_database(conn: &mut PooledConn) -> Result<(), Error> {
    match conn.query_drop("CREATE DATABASE IF NOT EXISTS frigo") {
        Ok(_) => match conn.query_drop("USE frigo") {
            Ok(_) => Ok(()),
            Err(err) => Err(err)
        },
        Err(err) => Err(err)
    }
}

fn create_table(conn: &mut PooledConn) -> Result<(), Error> {
    match conn.query_drop("
        CREATE TABLE IF NOT EXISTS foods (
            id INT AUTO_INCREMENT PRIMARY KEY,
            name VARCHAR(255) NOT NULL,
            ingestion ENUM('drink', 'eat') NOT NULL,
            carbs INT,
            calories INT,
            proteins INT,
            electrolytes BOOLEAN NOT NULL,
            comment TEXT
        )
    ") {
        Ok(_) => Ok(()),
        Err(err) => Err(err)
    }
}