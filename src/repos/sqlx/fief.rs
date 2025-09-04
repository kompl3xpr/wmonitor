use anyhow::Result;
use async_trait::async_trait;
use chrono::TimeZone;
use sqlx::sqlite::SqlitePool;

use std::sync::Arc;

use crate::{cfg, entities};
use crate::domains::{FiefId, Fief, ChunkId, UserId};
use crate::repos::traits::FiefRepo;
use crate::utils::db::*;

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
    async fn create(&self, name: &str, check_interval: Option<chrono::Duration>) -> Result<bool> {
        let check_interval = check_interval
            .map(|i| i.num_minutes())
            .unwrap_or(cfg().checker.default_interval_min as i64);
        let min_interval = cfg().checker.minimum_interval_min as i64;
        let ago = chrono::Utc.with_ymd_and_hms(1919, 11, 4, 5, 14, 00).unwrap(); 
        
        let result = sqlx::query(
            "INSERT INTO Fiefs
            (id, name, check_interval_min, last_check, skip_check_until)
            VALUES ($1, $2, $3, $4, $5)"
        )
            .bind(1)
            .bind(name)
            .bind(check_interval.max(min_interval))
            .bind(ago)
            .bind(ago)
            .execute(&*self.0)
            .await;

        Ok(conv_create_result(result)?)
    }

    // [R]ead
    // - self or fields
    async fn name(&self, id: FiefId) -> Result<Option<String>> {
        let result = sqlx::query_as(
            "SELECT name FROM Fiefs WHERE id = $1"
        ).bind(id.0)
        .fetch_one(&*self.0)
        .await;

        let result: Option<(String,)> = conv_fetch_one_result(result)?;
        Ok(result.map(|r| r.0))
    }

    async fn id(&self, name: &str) -> Result<Option<FiefId>> {
        let result = sqlx::query_as(
            "SELECT id FROM Fiefs WHERE name = $1"
        ).bind(name)
        .fetch_one(&*self.0)
        .await;

        let result: Option<(i64,)> = conv_fetch_one_result(result)?;
        Ok(result.map(|r| FiefId(r.0)))
    }

    async fn fief_by_id(&self, id: FiefId) -> Result<Option<Fief>> {
        let result = sqlx::query_as(
            "SELECT * FROM Fiefs WHERE id = $1"
        ).bind(id.0)
        .fetch_one(&*self.0)
        .await;

        let result: Option<entities::Fief> = conv_fetch_one_result(result)?;
        Ok(result.map(|r| Fief {
            id: FiefId(r.id),
            name: r.name,
            check_interval: chrono::Duration::minutes(r.check_interval_min),
            last_check: r.last_check,
            skip_check_until: r.skip_check_until,
        }))
    }

    async fn fief_by_name(&self, name: &str) -> Result<Option<Fief>> {
        let result = sqlx::query_as(
            "SELECT * FROM Fiefs WHERE name = $1"
        ).bind(name)
        .fetch_one(&*self.0)
        .await;

        let result: Option<entities::Fief> = conv_fetch_one_result(result)?;
        Ok(result.map(|r| Fief {
            id: FiefId(r.id),
            name: r.name,
            check_interval: chrono::Duration::minutes(r.check_interval_min),
            last_check: r.last_check,
            skip_check_until: r.skip_check_until,
        }))
    }

    async fn fiefs_to_check(&self) -> Result<Vec<Fief>> {
        Ok(sqlx::query_as(
            "SELECT * FROM Fiefs
            WHERE datetime(last_check, '+' || check_interval_min || ' minutes') < datetime('now')
            AND (skip_check_until IS NULL OR datetime(skip_check_until) < datetime('now'))"
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
        Ok(sqlx::query_as("SELECT user_id FROM Members WHERE fief_id = $1")
            .bind(id.0)
            .fetch_all(&*self.0)
            .await?
            .into_iter()
            .map(|uid: (i64,)| UserId(uid.0))
            .collect())
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
            WHERE fief_id = $1"
        ).bind(id.0)
        .fetch_one(&*self.0)
        .await?;

        Ok(result.0 as usize)
    }

    async fn diff_count(&self, id: FiefId) -> Result<usize> {
        let result: (i64,) = sqlx::query_as(
            "SELECT COALESCE(SUM(diff_count), 0) AS total_diff_count
            FROM Chunks
            WHERE fief_id = $1"
        ).bind(id.0)
        .fetch_one(&*self.0)
        .await?;

        Ok(result.0 as usize)
    }

    // [U]pdate
    // - self or fields
    async fn update_last_check(&self, id: FiefId) -> Result<()> {
        sqlx::query("UPDATE Fiefs SET last_check = $1 WHERE id = $2")
            .bind(chrono::Utc::now())
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(())
    }

    async fn set_check_interval(&self, id: FiefId, interval: chrono::Duration) -> Result<()> {
        let min_interval = cfg().checker.minimum_interval_min as i64;
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

    async fn skip_check_for(&self, id: FiefId, dur: chrono::Duration) -> Result<()> {
        let datetime = chrono::Utc::now() + dur;
        sqlx::query("UPDATE Fiefs SET skip_check_until = $1 WHERE id = $2")
            .bind(datetime)
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(())
    }

    async fn set_name(&self, id: FiefId, name: &str) -> Result<()> {
        sqlx::query("UPDATE Fiefs SET name = $1 WHERE id = $2")
            .bind(name)
            .bind(id.0)
            .execute(&*self.0)
            .await?;
        Ok(())
    }
    // - related
    // *PASS*

    // [D]elete
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
