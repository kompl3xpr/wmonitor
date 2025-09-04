use crate::domains::FiefId;
use crate::core::{ImagePng, Position};
use anyhow::Result;
use async_trait::async_trait;

pub(super) mod domains {
    use crate::{domains::FiefId, core::Position};
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
    pub struct ChunkId(pub i64);

    impl From<i64> for ChunkId {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }

    #[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct Chunk {
        pub id: ChunkId,
        pub name: String,
        pub fief_id: FiefId,
        pub position: Position,
    }
}
use domains::*;

#[async_trait]
pub trait ChunkRepo {
    // [C]reate
    async fn create(&self, name: &str, fief_id: FiefId, pos: Position) -> Result<Option<ChunkId>>;

    // [R]ead
    // - self or fields
    async fn chunk_by_id(&self, id: ChunkId) -> Result<Chunk>;
    async fn chunk_by_name(&self, fief_id: FiefId, name: &str) -> Result<Chunk>;
    async fn fief_id(&self, id: ChunkId) -> Result<FiefId>;
    async fn name(&self, id: ChunkId) -> Result<String>;
    async fn id(&self, fief_id: FiefId, name: &str) -> Result<ChunkId>;
    async fn position(&self, id: ChunkId) -> Result<Position>;
    async fn ref_img(&self, id: ChunkId) -> Result<Option<ImagePng>>;
    async fn mask_img(&self, id: ChunkId) -> Result<Option<ImagePng>>;
    async fn diff_img(&self, id: ChunkId) -> Result<Option<ImagePng>>;
    async fn diff_count(&self, id: ChunkId) -> Result<usize>;
    // - related
    // *PASS*

    // [U]pdate
    // - self or fields
    async fn update_ref_img(&self, id: ChunkId, img: Option<ImagePng>) -> Result<()>;
    async fn update_mask_img(&self, id: ChunkId, img: Option<ImagePng>) -> Result<()>;
    async fn update_diff(&self, id: ChunkId, img: Option<ImagePng>, count: usize) -> Result<()>;
    async fn set_position(&self, id: ChunkId, pos: Position) -> Result<()>;
    async fn set_name(&self, id: ChunkId, name: &str) -> Result<()>;
    // - related
    // *PASS*

    // [D]elete
    async fn remove_by_id(&self, id: ChunkId) -> Result<bool>;
    async fn remove_by_name(&self, fief_id: FiefId, name: &str) -> Result<bool>;
    async fn remove_all_by_fief(&self, fief_id: FiefId) -> Result<bool>;
}
