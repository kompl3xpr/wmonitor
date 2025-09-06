use tracing::{error, info, level_filters::LevelFilter, warn};
use wmonitor::{Repositories, app, bot, cfg, init_cfg};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::DEBUG)
        .init();
    info!("starting logging...");

    info!("attempting to get environment variables from `.env`(if exists)...");
    if let Err(e) = dotenv::dotenv() {
        warn!("failed to apply `.env`: {e}");
    }

    info!("initializing configurations...");
    init_cfg();
    info!("loaded configurations:\n{:#?}", cfg());

    let wmonitor = app::WMonitor::builder()
        .discord_token(discord_token())
        .repo(Repositories::from_sqlx(&datebase_url()).await?)
        .build();

    wmonitor.run().await?;
    Ok(())
}

fn discord_token() -> String {
    let token = cfg().common.discord_token.clone();
    match token.is_empty() {
        true => {
            info!("attempting to use environment variable `DISCORD_TOKEN`...");
            std::env::var("DISCORD_TOKEN").unwrap_or_else(|e| {
                error!("failed to get variable `DISCORD_TOKEN`: {e}");
                panic!();
            })
        }
        _ => token,
    }
}

fn datebase_url() -> String {
    let url = cfg().common.database_url.clone();
    match url.is_empty() {
        true => {
            info!("attempting to use environment variable `DATABASE_URL`...");
            std::env::var("DATABASE_URL").unwrap_or_else(|e| {
                error!("failed to get variable `DATABASE_URL`: {e}");
                panic!();
            })
        }
        _ => url,
    }
}
