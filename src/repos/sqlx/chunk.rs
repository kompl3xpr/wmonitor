use anyhow::Result;
use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;

use std::sync::Arc;

use crate::{
    domains::{Chunk, ChunkId, FiefId},
    entities,
    repos::traits::ChunkRepo,
    core::{ImagePng, Position},
};

pub struct SqlxChunkRepo(Arc<SqlitePool>);

impl SqlxChunkRepo {
    pub fn new(pool: Arc<SqlitePool>) -> Self {
        Self(pool)
    }
}

#[allow(unused)]
#[async_trait]
impl ChunkRepo for SqlxChunkRepo {
    // [C]reate
    async fn create(&self, name: &str, fief_id: FiefId, pos: Position) -> Result<Option<ChunkId>> {
        let result: Vec<(i64,)> = sqlx::query_as(
            "SELECT id FROM Chunks
            WHERE fief_id = $1 AND name = $2",
        )
        .bind(fief_id.0)
        .bind(name)
        .fetch_all(&*self.0)
        .await?;

        if !result.is_empty() {
            return Ok(None);
        }

        let result = sqlx::query(
            "INSERT INTO Chunks
            (name, fief_id, pos_x, pos_y, diff_count)
            VALUES ($1, $2, $3, $4, $5)",
        )
        .bind(name)
        .bind(fief_id.0)
        .bind(pos.x as i64)
        .bind(pos.y as i64)
        .bind(0)
        .execute(&*self.0)
        .await;

        Ok(super::conv_create_result(result)?)
    }

    // [R]ead
    // - self or fields
    async fn chunk_by_id(&self, id: ChunkId) -> Result<Chunk> {
        let result: entities::ChunkWithoutImgs = sqlx::query_as(
            "SELECT id, name, fief_id, pos_x, pos_y, diff_count
            FROM Chunks
            WHERE id = $1",
        )
        .bind(id.0)
        .fetch_one(&*self.0)
        .await?;

        let position = Position {
            x: result.position.x as usize,
            y: result.position.y as usize,
        };
        Ok(Chunk {
            id: ChunkId(result.id),
            name: result.name,
            fief_id: FiefId(result.fief_id),
            position,
        })
    }

    async fn chunk_by_name(&self, fief_id: FiefId, name: &str) -> Result<Chunk> {
        let result: entities::ChunkWithoutImgs = sqlx::query_as(
            "SELECT id, name, fief_id, pos_x, pos_y, diff_count
            FROM Chunks
            WHERE fief_id = $1 AND name = $2",
        )
        .bind(fief_id.0)
        .bind(name)
        .fetch_one(&*self.0)
        .await?;

        let position = Position {
            x: result.position.x as usize,
            y: result.position.y as usize,
        };
        Ok(Chunk {
            id: ChunkId(result.id),
            name: result.name,
            fief_id: FiefId(result.fief_id),
            position,
        })
    }

    async fn fief_id(&self, id: ChunkId) -> Result<FiefId> {
        let result: (i64,) = sqlx::query_as("SELECT fief_id FROM Chunks WHERE id = $1")
            .bind(id.0)
            .fetch_one(&*self.0)
            .await?;

        Ok(FiefId(result.0))
    }

    async fn name(&self, id: ChunkId) -> Result<String> {
        let result: (String,) = sqlx::query_as("SELECT name FROM Chunks WHERE id = $1")
            .bind(id.0)
            .fetch_one(&*self.0)
            .await?;

        Ok(result.0)
    }

    async fn id(&self, fief_id: FiefId, name: &str) -> Result<ChunkId> {
        let result: (i64,) = sqlx::query_as(
            "SELECT id FROM Chunks
            WHERE fief_id = $1 AND name = $2",
        )
        .bind(fief_id.0)
        .bind(name)
        .fetch_one(&*self.0)
        .await?;

        Ok(ChunkId(result.0))
    }

    async fn position(&self, id: ChunkId) -> Result<Position> {
        let result: entities::Position =
            sqlx::query_as("SELECT pos_x, pos_y FROM Chunks WHERE id = $1")
                .bind(id.0)
                .fetch_one(&*self.0)
                .await?;

        Ok(Position {
            x: result.x as usize,
            y: result.y as usize,
        })
    }

    async fn ref_img(&self, id: ChunkId) -> Result<Option<ImagePng>> {
        let result: (Option<Vec<u8>>,) = sqlx::query_as("SELECT img_ref FROM Chunks WHERE id = $1")
            .bind(id.0)
            .fetch_one(&*self.0)
            .await?;

        Ok(result.0.map(ImagePng::new))
    }

    async fn mask_img(&self, id: ChunkId) -> Result<Option<ImagePng>> {
        let result: (Option<Vec<u8>>,) =
            sqlx::query_as("SELECT img_mask FROM Chunks WHERE id = $1")
                .bind(id.0)
                .fetch_one(&*self.0)
                .await?;

        Ok(result.0.map(ImagePng::new))
    }

    async fn diff_img(&self, id: ChunkId) -> Result<Option<ImagePng>> {
        let result: (Option<Vec<u8>>,) =
            sqlx::query_as("SELECT img_diff FROM Chunks WHERE id = $1")
                .bind(id.0)
                .fetch_one(&*self.0)
                .await?;

        Ok(result.0.map(ImagePng::new))
    }

    async fn diff_count(&self, id: ChunkId) -> Result<usize> {
        let result: (i64,) = sqlx::query_as("SELECT diff_count FROM Chunks WHERE id = $1")
            .bind(id.0)
            .fetch_one(&*self.0)
            .await?;

        Ok(result.0 as usize)
    }
    // - related
    // *PASS*

    // [U]pdate
    // - self or fields
    async fn update_ref_img(&self, id: ChunkId, img: Option<ImagePng>) -> Result<()> {
        sqlx::query("UPDATE Chunks SET img_ref = $1 WHERE id = $2")
            .bind(img.map(ImagePng::into_inner))
            .bind(id.0)
            .execute(&*self.0)
            .await?;

        Ok(())
    }

    async fn update_mask_img(&self, id: ChunkId, img: Option<ImagePng>) -> Result<()> {
        sqlx::query("UPDATE Chunks SET img_mask = $1 WHERE id = $2")
            .bind(img.map(ImagePng::into_inner))
            .bind(id.0)
            .execute(&*self.0)
            .await?;

        Ok(())
    }

    async fn update_diff(&self, id: ChunkId, img: Option<ImagePng>, count: usize) -> Result<()> {
        sqlx::query(
            "UPDATE Chunks
            SET img_diff = $1, diff_count = $2
            WHERE id = $3",
        )
        .bind(img.map(ImagePng::into_inner))
        .bind(count as i64)
        .bind(id.0)
        .execute(&*self.0)
        .await?;

        Ok(())
    }

    async fn set_position(&self, id: ChunkId, pos: Position) -> Result<()> {
        sqlx::query(
            "UPDATE Chunks
            SET pos_x = $1, pos_y = $2
            WHERE id = $3",
        )
        .bind(pos.x as i64)
        .bind(pos.y as i64)
        .bind(id.0)
        .execute(&*self.0)
        .await?;

        Ok(())
    }

    async fn set_name(&self, id: ChunkId, name: &str) -> Result<()> {
        let result: Vec<(i64,)> = sqlx::query_as(
            "SELECT id FROM Chunks
            WHERE fief_id = (SELECT fief_id FROM Chunks WHERE id = $1) AND name = $2",
        )
        .bind(id.0)
        .bind(name)
        .fetch_all(&*self.0)
        .await?;

        if !result.is_empty() {
            return Err(anyhow::anyhow!("name {name} already exists"));
        }

        sqlx::query("UPDATE Chunks SET name = $1 WHERE id = $2")
            .bind(name)
            .bind(id.0)
            .execute(&*self.0)
            .await?;

        Ok(())
    }
    // - related
    // *PASS*

    // [D]elete
    async fn remove_by_id(&self, id: ChunkId) -> Result<bool> {
        let result = sqlx::query("DELETE FROM Chunks WHERE id = $1")
            .bind(id.0)
            .execute(&*self.0)
            .await?;

        Ok(result.rows_affected() == 1)
    }

    async fn remove_by_name(&self, fief_id: FiefId, name: &str) -> Result<bool> {
        let result = sqlx::query("DELETE FROM Chunks WHERE fief_id = $1 AND name = $2")
            .bind(fief_id.0)
            .bind(name)
            .execute(&*self.0)
            .await?;

        Ok(result.rows_affected() == 1)
    }

    async fn remove_all_by_fief(&self, fief_id: FiefId) -> Result<bool> {
        let result = sqlx::query("DELETE FROM Chunks WHERE fief_id = $1")
            .bind(fief_id.0)
            .execute(&*self.0)
            .await?;

        Ok(result.rows_affected() >= 1)
    }
}
