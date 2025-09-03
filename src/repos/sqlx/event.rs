use crate::domains::{Event, EventId, EventKind};
use crate::repos::traits::EventRepo;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

pub struct SqlxEventRepo(Arc<SqlitePool>);

impl SqlxEventRepo {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self(pool)
    }
}

#[allow(unused)]
#[async_trait]
impl EventRepo for SqlxEventRepo {
    // [C]reate
    async fn save(&self, kind: EventKind) -> Result<()> {
        todo!()
    }

    // [R]ead
    // - self or fields
    async fn kind(&self, id: EventId) -> Result<EventKind> {
        todo!()
    }
    async fn date(&self, id: EventId) -> Result<chrono::DateTime<chrono::Utc>> {
        todo!()
    }
    async fn event_by_id(&self, id: EventId) -> Result<Event> {
        todo!()
    }

    // [U]pdate
    // *PASS*

    // [D]elete
    async fn remove_by_id(&self, id: EventId) -> Result<()> {
        todo!()
    }
    async fn remove_all_by_kind(&self, kind: &str) -> Result<()> {
        todo!()
    }
    async fn remove_all_before(&self, date: chrono::DateTime<chrono::Utc>) -> Result<()> {
        todo!()
    }
    async fn remove_all(&self) -> Result<()> {
        todo!()
    }
}
