
pub mod config;
pub mod utils;
pub use config::{cfg, init_cfg, save_cfg_with};

pub mod entities;

pub mod repos;
use repos::Repositories;

pub mod domains {
    pub use crate::repos::domains::*;
}

pub mod app;
pub mod bot;
pub mod checker;
pub mod commands;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenv::dotenv()?;
    init_cfg();

    let wmonitor = app::WMonitor::builder()
        .bot(bot::new_client().await?)
        .repo(Repositories::from_sqlx().await?)
        .build();

    wmonitor.run().await?;
    Ok(())
}
