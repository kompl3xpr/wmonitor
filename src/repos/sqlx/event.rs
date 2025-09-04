use crate::domains::{Event, EventId, EventKind};
use crate::entities;
use crate::repos::traits::EventRepo;
use crate::utils::db::*;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;
use tap::prelude::*;

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
    async fn save(&self, kind: EventKind) -> Result<Option<EventId>> {
        let (kind, value) = kind.into_raw()?;
        let result = sqlx::query("INSERT INTO Events (date, kind, value) VALUES ($1, $2, $3)")
            .bind(chrono::Utc::now())
            .bind(kind)
            .bind(value)
            .execute(&*self.0)
            .await;

        Ok(conv_create_result(result)?)
    }

    // [R]ead
    // - self or fields
    async fn kind(&self, id: EventId) -> Result<EventKind> {
        let (kind, json_value): (String, sqlx::types::JsonValue) =
            sqlx::query_as("SELECT kind, value FROM Events WHERE id = $1")
                .bind(id.0)
                .fetch_one(&*self.0)
                .await?;

        Ok(EventKind::from_raw(&kind, json_value)?)
    }

    async fn date(&self, id: EventId) -> Result<chrono::DateTime<chrono::Utc>> {
        let result: (chrono::DateTime<chrono::Utc>,) =
            sqlx::query_as("SELECT date FROM Events WHERE id = $1")
                .bind(id.0)
                .fetch_one(&*self.0)
                .await?;

        Ok(result.0)
    }

    async fn event_by_id(&self, id: EventId) -> Result<Event> {
        let result: entities::Event = sqlx::query_as("SELECT * FROM Events WHERE id = $1")
            .bind(id.0)
            .fetch_one(&*self.0)
            .await?;

        Ok(Event {
            id: EventId(result.id),
            date: result.date,
            kind: EventKind::from_raw(&result.kind, result.value)?,
        })
    }

    async fn all(&self) -> Result<Vec<Event>> {
        let result: Vec<entities::Event> = sqlx::query_as("SELECT * FROM Events")
            .fetch_all(&*self.0)
            .await?;

        let filter_mapper = |r: entities::Event| {
            EventKind::from_raw(&r.kind, r.value)
                .ok()
                .map(|ev| (EventId(r.id), r.date, ev))
        };
        result
            .into_iter()
            .filter_map(filter_mapper)
            .map(|(id, date, kind)| Event { id, date, kind })
            .collect::<Vec<_>>()
            .pipe(Ok)
    }

    async fn all_by_kind(&self, kind: &str) -> Result<Vec<Event>> {
        let result: Vec<entities::Event> = sqlx::query_as("SELECT * FROM Events WHERE kind = $1")
            .bind(kind)
            .fetch_all(&*self.0)
            .await?;

        let filter_mapper = |r: entities::Event| {
            EventKind::from_raw(&r.kind, r.value)
                .ok()
                .map(|ev| (EventId(r.id), r.date, ev))
        };
        result
            .into_iter()
            .filter_map(filter_mapper)
            .map(|(id, date, kind)| Event { id, date, kind })
            .collect::<Vec<_>>()
            .pipe(Ok)
    }

    async fn all_before(&self, date: chrono::DateTime<chrono::Utc>) -> Result<Vec<Event>> {
        let result: Vec<entities::Event> =
            sqlx::query_as("SELECT * FROM Events WHERE datetime(date) < datetime($1)")
                .bind(date)
                .fetch_all(&*self.0)
                .await?;

        let filter_mapper = |r: entities::Event| {
            EventKind::from_raw(&r.kind, r.value)
                .ok()
                .map(|ev| (EventId(r.id), r.date, ev))
        };
        result
            .into_iter()
            .filter_map(filter_mapper)
            .map(|(id, date, kind)| Event { id, date, kind })
            .collect::<Vec<_>>()
            .pipe(Ok)
    }

    // [U]pdate
    // *PASS*

    // [D]elete
    async fn remove_by_id(&self, id: EventId) -> Result<bool> {
        let result = sqlx::query("DELETE FROM Events WHERE id = $1")
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(result.rows_affected() == 1)
    }

    async fn remove_all_by_kind(&self, kind: &str) -> Result<bool> {
        let result = sqlx::query("DELETE FROM Events WHERE kind = $1")
            .bind(kind)
            .execute(&*self.0)
            .await?;
        Ok(result.rows_affected() >= 1)
    }

    async fn remove_all_before(&self, date: chrono::DateTime<chrono::Utc>) -> Result<bool> {
        let result = sqlx::query("DELETE FROM Events WHERE datetime(date) < datetime($1)")
            .bind(date)
            .execute(&*self.0)
            .await?;
        Ok(result.rows_affected() >= 1)
    }

    async fn remove_all(&self) -> Result<bool> {
        let result = sqlx::query("DELETE FROM Events").execute(&*self.0).await?;
        Ok(result.rows_affected() >= 1)
    }
}
