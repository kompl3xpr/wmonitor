use std::str::FromStr;

use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};

pub mod entities;
pub mod commands;
pub mod checker;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options = SqliteConnectOptions::from_str("sqlite://db/wmonitor.db")?
        .create_if_missing(true);

    let pool = SqlitePool::connect_with(options).await?;
    sqlx::migrate!("db/migrations").run(&pool).await?;

    Ok(())
}
