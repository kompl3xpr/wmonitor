use anyhow::Result;
use tokio::sync::mpsc::Sender;
use tracing::{debug, error};

use crate::check::algorithms;
use crate::core::ImagePng;
use crate::domains::{CheckErrorEvent, ChunkId, EventKind, FiefId};
use crate::{Repositories, net};


pub struct Checker {
    event_tx: Sender<EventKind>,
    repo: Repositories,
}

impl Checker {
    pub fn new(repo: Repositories, event_tx: Sender<EventKind>) -> Self {
        Self { repo, event_tx }
    }

    pub async fn check_one(&self, fief_id: FiefId) -> Result<()> {
        crate::core::lock_fief!(fief_id);

        let Ok(chunks) = self.repo.fief().chunks(fief_id).await else {
            let msg = format!("cannot get chunks from fief `{}`", fief_id.0);
            send_err(&self.event_tx, msg.clone()).await;
            return Err(anyhow::anyhow!(msg));
        };

        let chunk_checker = async |id: ChunkId| -> Result<()> {
            let pos = self.repo.chunk().position(id).await?;

            let ref_ = self.repo.chunk().ref_img(id).await?;
            let Some(ref_) = ref_.map(ImagePng::try_to_rgba) else {
                return Err(anyhow::anyhow!(""));
            };

            let mask = self.repo.chunk().mask_img(id).await?;
            let Some(mask) = mask.map(ImagePng::try_to_gray) else {
                return Err(anyhow::anyhow!(""));
            };

            let curr = net::fetch_current_image(pos).await?.1;
            let curr = curr.try_into()?;

            let (ref_, mask) = (ref_?, mask?);
            let rec = algorithms::find_diffs(&ref_, &mask, &curr)?;
            let result = algorithms::gen_visual_result(&ref_, &mask, &curr, &rec)?;

            self.repo
                .chunk()
                .update_result_img(id, result.try_into().ok())
                .await?;

            self.repo
                .chunk()
                .update_diff(id, rec.diff_img.try_into().ok(), rec.diffs.len())
                .await?;

            Ok(())
        };

        for id in chunks {
            if let Err(e) = chunk_checker(id).await {
                send_err(
                    &self.event_tx,
                    format!(
                        "failed to check chunks(chunk id:{}, fief id: {}): {}",
                        id.0, fief_id.0, e
                    ),
                )
                .await;
            }
        }

        Ok(())
    }

    pub async fn check_all(&self) -> Result<()> {
        debug!("checking all fiefs...");
        let Ok(fiefs) = self.repo.fief().fiefs_to_check().await else {
            send_err(&self.event_tx, "failed to get fiefs to check").await;
            return Err(anyhow::anyhow!("failed to get fiefs to check"));
        };

        for fief in fiefs {
            self.check_one(fief.id).await.ok();
        }

        Ok(())
    }
}
async fn send_err(tx: &Sender<EventKind>, desc: impl Into<String>) {
    let desc = desc.into();
    error!(desc);
    tx.send(EventKind::CheckError(CheckErrorEvent { description: desc }))
        .await
        .ok();
}
