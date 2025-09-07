use std::time::Duration;

use anyhow::Result;
use tokio::sync::mpsc::Sender;

use crate::check::algorithms;
use crate::core::ImagePng;
use crate::domains::{ChunkId, FiefId};
use crate::{Repositories, net};

use super::Event;

pub struct Checker {
    event_tx: Sender<Event>,
    repo: Repositories,
}

impl Checker {
    pub fn new(repo: Repositories, event_tx: Sender<Event>) -> Self {
        Self { repo, event_tx }
    }

    async fn send(&self, event: Event) {
        self.event_tx.send_timeout(event, Duration::from_secs(10)).await.ok();
    }

    pub async fn check_one(&self, fief_id: FiefId) -> Result<()> {
        crate::core::lock_fief!(fief_id);

        let Ok(chunks) = self.repo.fief().chunks(fief_id).await else {
            let msg = format!("cannot get chunks from fief `{}`", fief_id.0);
            return Err(anyhow::anyhow!(msg));
        };

        let chunk_checker = async |id: ChunkId| -> Result<bool> {
            let pos = self.repo.chunk().position(id).await?;

            let ref_ = self.repo.chunk().ref_img(id).await?;
            let Some(ref_) = ref_.map(ImagePng::try_to_rgba) else {
                self.send(Event::ChunkRefMissing(fief_id, id)).await;
                return Err(anyhow::anyhow!("ref image not found"));
            };

            let mask = self.repo.chunk().mask_img(id).await?;
            let Some(mask) = mask.map(ImagePng::try_to_gray) else {
                self.send(Event::ChunkMaskMissing(fief_id, id)).await;
                return Err(anyhow::anyhow!("mask image not found"));
            };

            let curr = match net::fetch_current_image(pos).await {
                Ok(curr) => curr,
                Err(e) => {
                    let msg = format!("网络错误: {e}");
                    self.send(Event::NetworkError(msg)).await;
                    return Err(e);
                }
            };
            let curr = curr.1.try_into()?;

            let (ref_, mask) = (ref_?, mask?);
            let rec = algorithms::find_diffs(&ref_, &mask, &curr)?;

            if !rec.diffs.is_empty() {
                let result = algorithms::gen_visual_result(&ref_, &mask, &curr, &rec)?;
                self.repo
                    .chunk()
                    .update_result_img(id, result.try_into().ok())
                    .await?;
            }

            self.repo
                .chunk()
                .update_diff(id, rec.diff_img.try_into().ok(), rec.diffs.len())
                .await?;

            Ok(rec.diffs.is_empty())
        };

        let mut failed_chunks = vec![];
        for id in chunks {
            if !chunk_checker(id).await.unwrap_or(true) {
                failed_chunks.push(id);
            }
        }
        self.repo.fief().update_last_check(fief_id, None).await?;

        if !failed_chunks.is_empty() {
            self.send(Event::DiffFound(fief_id, failed_chunks)).await;
        }

        Ok(())
    }

    pub async fn check_all(&self) -> Result<()> {
        let Ok(fiefs) = self.repo.fief().fiefs_to_check().await else {
            return Err(anyhow::anyhow!("failed to get fiefs to check"));
        };

        for fief in fiefs {
            self.check_one(fief.id).await.ok();
        }

        Ok(())
    }
}
