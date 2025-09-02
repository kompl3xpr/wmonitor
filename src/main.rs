use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

pub mod config;
pub use config::{cfg, save_cfg_with, init_cfg};

pub mod bot;
pub mod checker;
pub mod commands;
pub mod entities;

pub mod repos;
pub mod domains {
    pub use crate::repos::domains::*;
}

pub mod utils;
pub mod app;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenv::dotenv()?;
    init_cfg();

    let options =
        SqliteConnectOptions::from_str("sqlite://db/wmonitor.db")?.create_if_missing(true);
    let pool = SqlitePool::connect_with(options).await?;
    sqlx::migrate!("db/migrations").run(&pool).await?;

    let mut client = bot::new_client().await?;
    client.start().await?;
    Ok(())
}
