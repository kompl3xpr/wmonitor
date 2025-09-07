use tracing::{info, level_filters::LevelFilter, warn};
use wmonitor::{Repositories, app, cfg, core::get_or_env, init_cfg};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(LevelFilter::INFO)
        .init();
    info!("starting logging...");

    info!("attempting to get environment variables from `.env`(if exists)...");
    if let Err(e) = dotenv::dotenv() {
        warn!("failed to apply `.env`: {e}");
    }

    info!("initializing configurations...");
    init_cfg();
    info!("loaded configurations:\n{:#?}", cfg());

    let discord_token = get_or_env(&cfg().common.discord_token, "", "DISCORD_TOKEN");
    let datebase_url = get_or_env(&cfg().common.database_url, "", "DATABASE_URL");
    let wmonitor = app::WMonitor::builder()
        .discord_token(discord_token)
        .repo(Repositories::from_sqlx(&datebase_url).await?)
        .build();

    wmonitor.run().await?;
    Ok(())
}
