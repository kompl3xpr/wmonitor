use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

pub mod bot;
pub mod checker;
pub mod commands;
pub mod utils;
pub mod entities;
pub mod config;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv()?;

    let options = SqliteConnectOptions::from_str("sqlite://db/wmonitor.db")?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    sqlx::migrate!("db/migrations").run(&pool).await?;

    Ok(())
}
