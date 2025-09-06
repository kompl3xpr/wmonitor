use std::{
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};

use crate::check::Checker;
use anyhow::Result;
use tokio::time::sleep;

#[derive(typed_builder::TypedBuilder)]
pub struct WMonitor {
    repo: crate::repos::Repositories,
    bot: serenity::Client,
}

impl WMonitor {
    pub async fn run(self) -> Result<()> {
        let (tx, mut rx) = tokio::sync::mpsc::channel(100);
        let should_close_atomic = Arc::new(AtomicBool::new(false));

        let should_close = should_close_atomic.clone();
        let checker = Checker::new(self.repo.clone(), tx.clone());
        let check_task = tokio::spawn(async move {
            while !should_close.load(Ordering::SeqCst) {
                checker.check_all().await.ok();
                sleep(Duration::from_secs(60)).await;
            }
        });

        let notify_task = tokio::spawn(async move {
            while let Some(ev) = rx.recv().await {
                println!("{ev:?}");
            }
        });

        let should_close = should_close_atomic.clone();
        let bot_task = tokio::spawn(async move {
            let _should_close = should_close;
            let _bot = self.bot;
        });

        tokio::try_join!(check_task, notify_task, bot_task)?;
        Ok(())
    }
}
