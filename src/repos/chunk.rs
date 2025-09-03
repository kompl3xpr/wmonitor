use anyhow::Result;
use async_trait::async_trait;

use crate::domains::FiefId;
use image::{GrayImage, RgbaImage};

pub(super) mod domains {
    use crate::domains::FiefId;
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct ChunkId(pub i64);

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct Position {
        pub x: i64,
        pub y: i64,
    }

    #[derive(Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct Chunk {
        pub id: i64,
        pub name: String,
        pub fief_id: FiefId,
        pub position: Position,
    }
}
use domains::*;

#[async_trait]
pub trait ChunkRepo {
    // [C]reate
    async fn create(&self, name: &str, fief_id: FiefId, pos: Position) -> Result<()>;

    // [R]ead
    // - self or fields
    async fn fief(&self, id: ChunkId) -> Result<FiefId>;
    async fn name(&self, id: ChunkId) -> Result<String>;
    async fn position(&self, id: ChunkId) -> Result<Position>;
    async fn chunk_by_id(&self, id: ChunkId) -> Result<Chunk>;
    async fn ref_img(&self, id: ChunkId) -> Result<Option<RgbaImage>>;
    async fn mask_img(&self, id: ChunkId) -> Result<Option<GrayImage>>;
    async fn diff_img(&self, id: ChunkId) -> Result<Option<GrayImage>>;
    async fn diff_count(&self, id: FiefId) -> Result<usize>;
    // - related
    // *PASS*

    // [U]pdate
    // - self or fields
    async fn update_ref_img(&self, id: ChunkId, img: RgbaImage) -> Result<()>;
    async fn update_mask_img(&self, id: ChunkId, img: RgbaImage) -> Result<()>;
    async fn update_diff(&self, id: ChunkId, img: RgbaImage, count: usize) -> Result<()>;
    async fn set_position(&self, id: ChunkId, pos: Position) -> Result<()>;
    async fn set_name(&self, id: ChunkId, name: &str) -> Result<()>;
    // - related
    // *PASS*

    // [D]elete
    async fn remove_by_id(&self, id: ChunkId) -> Result<bool>;
}
