use tracing::{error, info};
use wmonitor::{Repositories, app, bot, cfg, config::init_cfg};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    tracing_subscriber::fmt().init();
    dotenv::dotenv()?;
    init_cfg();

    // let wmonitor = app::WMonitor::builder()
    //     .bot(bot::new_client().await?)
    //     .repo(Repositories::from_sqlx(&datebase_url()).await?)
    //     .build();

    // wmonitor.run().await?;
    Ok(())
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
