use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;
use chrono::TimeZone;
use sqlx::sqlite::SqlitePool;

use crate::{
    cfg,
    domains::{ChunkId, Fief, FiefId, UserId},
    entities,
    repos::traits::FiefRepo,
};

pub struct SqlxFiefRepo(Arc<SqlitePool>);

impl SqlxFiefRepo {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self(pool)
    }
}

#[allow(unused)]
#[async_trait]
impl FiefRepo for SqlxFiefRepo {
    // [C] Create
    async fn create(
        &self,
        name: &str,
        check_interval: Option<chrono::Duration>,
    ) -> Result<Option<FiefId>> {
        let check_interval = check_interval
            .map(|i| i.num_minutes())
            .unwrap_or(cfg().check.default_interval_min as i64);
        let min_interval = cfg().check.minimum_interval_min as i64;
        let ago = chrono::Utc.with_ymd_and_hms(1919, 11, 4, 5, 1, 4).unwrap();

        let result = sqlx::query(
            "INSERT INTO Fiefs
            (name, check_interval_min, last_check, skip_check_until, should_check_now)
            VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(name)
        .bind(check_interval.max(min_interval))
        .bind(ago)
        .bind(ago)
        .bind(false)
        .execute(&*self.0)
        .await;

        Ok(super::conv_create_result(result)?)
    }

    // [R] Read
    // - self or fields
    async fn name(&self, id: FiefId) -> Result<String> {
        let result: (String,) = sqlx::query_as("SELECT name FROM Fiefs WHERE id = $1")
            .bind(id.0)
            .fetch_one(&*self.0)
            .await?;

        Ok(result.0)
    }

    async fn id(&self, name: &str) -> Result<FiefId> {
        let result: (i64,) = sqlx::query_as("SELECT id FROM Fiefs WHERE name = $1")
            .bind(name)
            .fetch_one(&*self.0)
            .await?;

        Ok(FiefId(result.0))
    }

    async fn fief_by_id(&self, id: FiefId) -> Result<Fief> {
        let r: entities::Fief = sqlx::query_as("SELECT * FROM Fiefs WHERE id = $1")
            .bind(id.0)
            .fetch_one(&*self.0)
            .await?;

        Ok(Fief {
            id: FiefId(r.id),
            name: r.name,
            check_interval: chrono::Duration::minutes(r.check_interval_min),
            last_check: r.last_check,
            skip_check_until: r.skip_check_until,
        })
    }

    async fn fief_by_name(&self, name: &str) -> Result<Fief> {
        let r: entities::Fief = sqlx::query_as("SELECT * FROM Fiefs WHERE name = $1")
            .bind(name)
            .fetch_one(&*self.0)
            .await?;

        Ok(Fief {
            id: FiefId(r.id),
            name: r.name,
            check_interval: chrono::Duration::minutes(r.check_interval_min),
            last_check: r.last_check,
            skip_check_until: r.skip_check_until,
        })
    }

    async fn fiefs_to_check(&self) -> Result<Vec<Fief>> {
        Ok(sqlx::query_as(
            "SELECT * FROM Fiefs
            WHERE should_check_now = TRUE OR
            (datetime(last_check, '+' || check_interval_min || ' minutes') < datetime('now')
            AND (skip_check_until IS NULL OR datetime(skip_check_until) < datetime('now')))",
        )
        .fetch_all(&*self.0)
        .await?
        .into_iter()
        .map(|f: entities::Fief| Fief {
            id: FiefId(f.id),
            name: f.name,
            check_interval: chrono::Duration::minutes(f.check_interval_min),
            last_check: f.last_check,
            skip_check_until: f.skip_check_until,
        })
        .collect())
    }

    async fn all(&self) -> Result<Vec<Fief>> {
        Ok(sqlx::query_as("SELECT * FROM Fiefs")
            .fetch_all(&*self.0)
            .await?
            .into_iter()
            .map(|f: entities::Fief| Fief {
                id: FiefId(f.id),
                name: f.name,
                check_interval: chrono::Duration::minutes(f.check_interval_min),
                last_check: f.last_check,
                skip_check_until: f.skip_check_until,
            })
            .collect())
    }

    // - related
    async fn members(&self, id: FiefId) -> Result<Vec<UserId>> {
        Ok(
            sqlx::query_as("SELECT user_id FROM Members WHERE fief_id = $1")
                .bind(id.0)
                .fetch_all(&*self.0)
                .await?
                .into_iter()
                .map(|uid: (i64,)| UserId(uid.0))
                .collect(),
        )
    }

    async fn chunks(&self, id: FiefId) -> Result<Vec<ChunkId>> {
        Ok(sqlx::query_as("SELECT id FROM Chunks WHERE fief_id = $1")
            .bind(id.0)
            .fetch_all(&*self.0)
            .await?
            .into_iter()
            .map(|cid: (i64,)| ChunkId(cid.0))
            .collect())
    }

    async fn chunk_count(&self, id: FiefId) -> Result<usize> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) AS chunk_count
            FROM Chunks
            WHERE fief_id = $1",
        )
        .bind(id.0)
        .fetch_one(&*self.0)
        .await?;

        Ok(result.0 as usize)
    }

    async fn diff_count(&self, id: FiefId) -> Result<usize> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(diff_count), 0) AS total_diff_count
            FROM Chunks
            WHERE fief_id = $1",
        )
        .bind(id.0)
        .fetch_one(&*self.0)
        .await?;

        Ok(result.0 as usize)
    }

    // [U] Update
    // - self or fields
    async fn update_last_check(
        &self,
        id: FiefId,
        date: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<()> {
        let date = date.unwrap_or(chrono::Utc::now());
        sqlx::query("UPDATE Fiefs SET last_check = $1, should_check_now = FALSE WHERE id = $2")
            .bind(date)
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(())
    }

    async fn set_check_interval(&self, id: FiefId, interval: chrono::Duration) -> Result<()> {
        let min_interval = cfg().check.minimum_interval_min as i64;
        sqlx::query("UPDATE Fiefs SET check_interval_min = $1 WHERE id = $2")
            .bind(interval.num_minutes().max(min_interval))
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(())
    }

    async fn skip_check(&self, id: FiefId) -> Result<()> {
        sqlx::query("UPDATE Fiefs SET skip_check_until = $1 WHERE id = $2")
            .bind(chrono::Utc.with_ymd_and_hms(2077, 1, 1, 0, 0, 0).unwrap())
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(())
    }

    async fn keep_check(&self, id: FiefId) -> Result<()> {
        sqlx::query("UPDATE Fiefs SET skip_check_until = $1 WHERE id = $2")
            .bind(chrono::Utc.with_ymd_and_hms(1919, 11, 4, 5, 1, 4).unwrap())
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(())
    }

    async fn mark_should_check_now(&self, id: FiefId) -> Result<()> {
        sqlx::query("UPDATE Fiefs SET should_check_now = TRUE WHERE id = $1")
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(())
    }

    async fn skip_check_for(
        &self,
        id: FiefId,
        dur: chrono::Duration,
        from: Option<chrono::DateTime<chrono::Utc>>,
    ) -> Result<()> {
        let datetime = from.unwrap_or(chrono::Utc::now()) + dur;
        sqlx::query("UPDATE Fiefs SET skip_check_until = $1 WHERE id = $2")
            .bind(datetime)
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(())
    }

    async fn rename(&self, id: FiefId, name: &str) -> Result<()> {
        sqlx::query("UPDATE Fiefs SET name = $1 WHERE id = $2")
            .bind(name)
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(())
    }

    // - related
    // *PASS*

    // [D] Delete
    async fn remove_by_id(&self, id: FiefId) -> Result<bool> {
        let result = sqlx::query("DELETE FROM Fiefs WHERE id = $1")
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(result.rows_affected() == 1)
    }

    async fn remove_by_name(&self, name: &str) -> Result<bool> {
        let result = sqlx::query("DELETE FROM Fiefs WHERE name = $1")
            .bind(name)
            .execute(&*self.0)
            .await?;
        Ok(result.rows_affected() == 1)
    }
}
