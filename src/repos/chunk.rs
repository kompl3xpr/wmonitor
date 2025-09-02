use async_trait::async_trait;

pub struct Chunk;


#[async_trait]
pub trait ChunkRepo {

}

pub struct SqlxChunkRepo {

}