use crate::domains::{FiefId, Fief, ChunkId, UserId};
use crate::repos::traits::FiefRepo;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

pub struct SqlxFiefRepo(Arc<SqlitePool>);

impl SqlxFiefRepo {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self(pool)
    }
}

#[allow(unused)]
#[async_trait]
impl FiefRepo for SqlxFiefRepo {
    // [C]reate
    async fn create(&self, name: &str, check_dur: chrono::Duration) -> Result<bool> {
        todo!()
    }

    // [R]ead
    // - self or fields
    async fn name(&self, id: FiefId) -> Result<String> {
        todo!()
    }
    async fn id(&self, name: &str) -> Result<FiefId> {
        todo!()
    }
    async fn fief_by_id(&self, id: FiefId) -> Result<Fief> {
        todo!()
    }
    async fn fief_by_name(&self, name: &str) -> Result<Fief> {
        todo!()
    }
    async fn fiefs_to_check(&self) -> Result<Vec<Fief>> {
        todo!()
    }
    async fn all(&self) -> Result<Vec<Fief>> {
        todo!()
    }
    // - related
    async fn members(&self, id: FiefId) -> Result<Vec<UserId>> {
        todo!()
    }
    async fn chunks(&self, id: FiefId) -> Result<Vec<ChunkId>> {
        todo!()
    }
    async fn chunk_count(&self, id: FiefId) -> Result<usize> {
        todo!()
    }
    async fn diff_count(&self, id: FiefId) -> Result<usize> {
        todo!()
    }

    // [U]pdate
    // - self or fields
    async fn update_last_check(&self, id: FiefId) -> Result<()> {
        todo!()
    }
    async fn set_check_interval(&self, id: FiefId, interval: chrono::Duration) -> Result<()> {
        todo!()
    }
    async fn skip_check(&self, id: FiefId) -> Result<()> {
        todo!()
    }
    async fn skip_check_for(&self, id: FiefId, dur: chrono::Duration) -> Result<()> {
        todo!()
    }
    async fn set_name(&self, id: FiefId, name: &str) -> Result<()> {
        todo!()
    }
    // - related
    // *PASS*

    // [D]elete
    async fn remove_by_id(&self, id: FiefId) -> Result<bool> {
        todo!()
    }
    async fn remove_by_name(&self, id: FiefId) -> Result<bool> {
        todo!()
    }
}
