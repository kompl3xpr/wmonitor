use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crate::{Repositories, bot, check::Checker};
use anyhow::Result;
use tokio::{sync::Mutex, time::sleep};
use tracing::{error, info};

#[derive(typed_builder::TypedBuilder)]
pub struct WMonitor {
    repo: Repositories,
    discord_token: String,
}

impl WMonitor {
    pub async fn run(self) -> Result<()> {
        let (tx, rx) = tokio::sync::mpsc::channel(100);
        let should_close_atomic = Arc::new(AtomicBool::new(false));

        let should_close = should_close_atomic.clone();
        let mut checker = Checker::new(self.repo.clone(), tx);
        let check_task = tokio::spawn(async move {
            info!("running checker...");
            while !should_close.load(Ordering::SeqCst) {
                if let Err(e) = checker.check_all().await {
                    error!("{e}");
                }
                sleep(Duration::from_secs(60)).await;
            }
        });

        let data = bot::Data {
            repo: self.repo.clone(),
            event_rx: Mutex::new(Some(rx)),
            should_close: should_close_atomic.clone(),
        };
        let mut bot = bot::new_client(&self.discord_token, data).await?;
        let bot_task = tokio::spawn(async move {
            info!("running bot...");
            if let Err(e) = bot.start().await {
                error!("{e}");
            }
        });

        tokio::select!(
            _ = check_task => (),
            _ = bot_task => (),
        );

        Ok(())
    }
}
