use crate::domains::{ChunkId, FiefId};

#[derive(Debug, Clone)]
pub enum Event {
    DiffFound(FiefId, Vec<ChunkId>),
    NetworkError(String),
    ChunkRefMissing(FiefId, ChunkId),
    ChunkMaskMissing(FiefId, ChunkId),
}
