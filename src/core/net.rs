use crate::{
    cfg,
    core::{ImagePng, Position},
};
use anyhow::Result;
use moka::future::{Cache, CacheBuilder};
use std::{sync::LazyLock, time::Duration};
use tokio::time::sleep;

static CACHE: LazyLock<Cache<Position, ImagePng>> = LazyLock::new(|| {
    CacheBuilder::new(cfg().network.image_cache_capacity as u64)
        .time_to_live(Duration::from_secs(
            60 * cfg().network.image_cache_life_min as u64,
        ))
        .time_to_idle(Duration::from_secs(
            60 * cfg().network.image_cache_life_min as u64,
        ))
        .build()
});

pub struct IsCached(pub bool);

pub async fn fetch_current_image(pos: impl Into<Position>) -> Result<(IsCached, ImagePng)> {
    let pos = pos.into();
    if let Some(img) = CACHE.get(&pos).await {
        return Ok((IsCached(true), img));
    }

    let dur = Duration::from_secs(cfg().network.sleep_between_requests_sec as u64);
    sleep(dur).await;
    
    let Position { x, y } = pos;
    let resp = reqwest::get(format!(
        "https://backend.wplace.live/files/s0/tiles/{x}/{y}.png"
    ))
    .await?;

    let img = ImagePng::new(resp.bytes().await?.into());
    CACHE.insert(pos, img.clone()).await;
    Ok((IsCached(false), img))
}
