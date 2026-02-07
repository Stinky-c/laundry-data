mod db;
use sql_middleware::prelude::*;
use std::env::var;
use refinery::AsyncMigrate;

#[tokio::main]
async fn main() {
    let host = var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
    let port = var("DB_PORT").unwrap_or_else(|_| "5432".to_string());
    let username = var("DB_USER").unwrap_or_else(|_| "postgres".to_string());
    let password = var("DB_PASSWORD").unwrap_or_else(|_| "password".to_string());
    println!("Hello, world!");


    let v = ConfigAndPool::new_mssql().await.unwrap();

    let mut conn = v.get_connection().await.unwrap();

    let runner = db::embedded::migrations::runner();
    let v = runner.run_async(&mut db::wrap(conn)).await; // Untested

    let query = QueryAndParams::new_without_params("SELECT * FROM users");
    let res = conn.query(&query.query).select().await?;
}

