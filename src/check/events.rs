use crate::domains::{ChunkId, FiefId};

#[derive(Debug, Clone)]
pub struct RetryTimes(pub usize);

#[derive(Debug, Clone)]
pub enum Event {
    CheckFailed(FiefId, RetryTimes),
    CheckSuccess(FiefId),
    DiffFound(FiefId, Vec<ChunkId>),
    NetworkError(String),
    ChunkRefMissing(FiefId, ChunkId),
    ChunkMaskMissing(FiefId, ChunkId),
}
