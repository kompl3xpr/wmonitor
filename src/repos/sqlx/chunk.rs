use crate::domains::{Chunk, ChunkId, FiefId, Position};
use crate::repos::traits::ChunkRepo;
use crate::utils::img::ImagePng;
use anyhow::Result;
use async_trait::async_trait;
use sqlx::sqlite::SqlitePool;
use std::sync::Arc;

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
    async fn create(&self, name: &str, fief_id: FiefId, pos: Position) -> Result<()> {
        todo!()
    }

    // [R]ead
    // - self or fields
    async fn fief(&self, id: ChunkId) -> Result<FiefId> {
        todo!()
    }
    async fn name(&self, id: ChunkId) -> Result<String> {
        todo!()
    }
    async fn position(&self, id: ChunkId) -> Result<Position> {
        todo!()
    }
    async fn chunk_by_id(&self, id: ChunkId) -> Result<Chunk> {
        todo!()
    }
    async fn ref_img(&self, id: ChunkId) -> Result<Option<ImagePng>> {
        todo!()
    }
    async fn mask_img(&self, id: ChunkId) -> Result<Option<ImagePng>> {
        todo!()
    }
    async fn diff_img(&self, id: ChunkId) -> Result<Option<ImagePng>> {
        todo!()
    }
    async fn diff_count(&self, id: FiefId) -> Result<usize> {
        todo!()
    }
    // - related
    // *PASS*

    // [U]pdate
    // - self or fields
    async fn update_ref_img(&self, id: ChunkId, img: ImagePng) -> Result<()> {
        todo!()
    }
    async fn update_mask_img(&self, id: ChunkId, img: ImagePng) -> Result<()> {
        todo!()
    }
    async fn update_diff(&self, id: ChunkId, img: ImagePng, count: usize) -> Result<()> {
        todo!()
    }
    async fn set_position(&self, id: ChunkId, pos: Position) -> Result<()> {
        todo!()
    }
    async fn set_name(&self, id: ChunkId, name: &str) -> Result<()> {
        todo!()
    }
    // - related
    // *PASS*

    // [D]elete
    async fn remove_by_id(&self, id: ChunkId) -> Result<bool> {
        todo!()
    }
}
