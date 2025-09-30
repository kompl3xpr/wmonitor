use std::{collections::HashMap, time::Duration};

use anyhow::Result;
use tokio::sync::mpsc::Sender;

use super::Event;
use crate::{
    Repositories,
    check::{RetryTimes, algorithms},
    core::{
        ImagePng,
        log::{error, info, warn},
    },
    domains::{ChunkId, FiefId},
    net,
};

pub const MAX_RETRY_TIMES: usize = 3;

pub struct Checker {
    event_tx: Sender<Event>,
    repo: &'static Repositories,
    retries: HashMap<FiefId, usize>,
}

impl Checker {
    pub fn new(repositories: &'static Repositories, event_sender: Sender<Event>) -> Self {
        Self {
            repo: repositories,
            event_tx: event_sender,
            retries: HashMap::new(),
        }
    }

    async fn send(&self, event: Event) {
        self.event_tx
            .send_timeout(event, Duration::from_secs(10))
            .await
            .ok();
    }

    pub async fn check_one(&mut self, fief_id: FiefId) -> Result<()> {
        info!("running check for fief {}", fief_id.0);

        let Ok(chunks) = self.repo.fief().chunks(fief_id).await else {
            let msg = format!("cannot get chunks from fief `{}`", fief_id.0);
            return Err(anyhow::anyhow!(msg));
        };

        let chunk_checker = async |id: ChunkId| -> Result<bool> {
            let pos = self.repo.chunk().position(id).await?;

            let ref_ = self.repo.chunk().ref_img(id).await?;
            let Some(ref_) = ref_.map(ImagePng::try_to_rgba) else {
                warn!("reference image of chunk {}.{} is null", fief_id.0, id.0);
                self.send(Event::ChunkRefMissing(fief_id, id)).await;
                return Ok(true);
            };

            let mask = self.repo.chunk().mask_img(id).await?;
            let Some(mask) = mask.map(ImagePng::try_to_gray) else {
                warn!("mask image of chunk {}.{} is null", fief_id.0, id.0);
                self.send(Event::ChunkMaskMissing(fief_id, id)).await;
                return Ok(true);
            };

            let curr = match net::fetch_current_image(pos).await {
                Ok(curr) => curr,
                Err(e) => {
                    warn!("{e}");
                    self.send(Event::NetworkError(e.to_string())).await;
                    return Err(e);
                }
            };
            let curr = curr.1.try_into()?;

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

            Ok(rec.diffs.is_empty())
        };

        let mut failed_chunks = vec![];
        let mut errors = vec![];
        for id in chunks {
            match chunk_checker(id).await {
                Ok(true) => (),
                Ok(false) => failed_chunks.push(id),
                Err(e) => errors.push(e),
            }
        }

        if !errors.is_empty() {
            let times = self.retries.entry(fief_id).or_insert(0);
            *times = MAX_RETRY_TIMES.min(*times) + 1;
            if *times > MAX_RETRY_TIMES {
                self.repo.fief().update_last_check(fief_id, None).await?;
            }
            let event = Event::CheckFailed(fief_id, RetryTimes(*times - 1));
            self.send(event).await;
            error!("failed to check fief {}: {:?}", fief_id.0, errors);
            return Ok(());
        }

        self.retries.remove(&fief_id);
        self.repo.fief().update_last_check(fief_id, None).await?;
        if !failed_chunks.is_empty() {
            info!("there are abnormal pixels in fief {}", fief_id.0);
            self.send(Event::DiffFound(fief_id, failed_chunks)).await;
        } else {
            info!("fief {} has no problem", fief_id.0);
            self.send(Event::CheckSuccess(fief_id)).await;
        }

        Ok(())
    }

    pub async fn check_all(&mut self) -> Result<()> {
        let Ok(fiefs) = self.repo.fief().fiefs_to_check().await else {
            error!("failed to get fiefs to check");
            return Err(anyhow::anyhow!("failed to get fiefs to check"));
        };

        for fief in fiefs {
            self.check_one(fief.id).await.ok();
        }

        net::clear_cache();
        Ok(())
    }
}
