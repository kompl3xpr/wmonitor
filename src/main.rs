use sqlx::sqlite::{SqliteConnectOptions, SqlitePool};
use std::str::FromStr;

mod config;
pub use config::{cfg, cfg_mut};

pub mod utils;
pub mod entities;
pub mod repository;
pub mod checker;
pub mod commands;
pub mod bot;



#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenv::dotenv()?;

    cfg_mut().bar.a += 1;

    // let options =
    //     SqliteConnectOptions::from_str("sqlite://db/wmonitor.db")?.create_if_missing(true);
    // let pool = SqlitePool::connect_with(options).await?;
    // sqlx::migrate!("db/migrations").run(&pool).await?;

    // let mut client = bot::new_client().await?;
    // client.start().await?;
    Ok(())
}
