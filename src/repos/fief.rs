use crate::domains::{ChunkId, UserId};
use anyhow::Result;
use async_trait::async_trait;

pub(super) mod domains {
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
    pub struct FiefId(pub i64);

    #[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct Fief {
        pub id: FiefId,
        pub name: String,
        pub check_interval: chrono::Duration,
        pub last_check: chrono::DateTime<chrono::Utc>,
        pub skip_check_until: chrono::DateTime<chrono::Utc>,
    }
}
use domains::*;

#[async_trait]
pub trait FiefRepo {
    // [C]reate
    async fn create(&self, name: &str, check_interval: Option<chrono::Duration>) -> Result<bool>;
    
    // [R]ead
    // - self or fields
    async fn name(&self, id: FiefId) -> Result<Option<String>>;
    async fn id(&self, name: &str) -> Result<Option<FiefId>>;
    async fn fief_by_id(&self, id: FiefId) -> Result<Option<Fief>>;
    async fn fief_by_name(&self, name: &str) -> Result<Option<Fief>>;
    async fn fiefs_to_check(&self) -> Result<Vec<Fief>>;
    async fn all(&self) -> Result<Vec<Fief>>;
    // - related
    async fn members(&self, id: FiefId) -> Result<Vec<UserId>>;
    async fn chunks(&self, id: FiefId) -> Result<Vec<ChunkId>>;
    async fn chunk_count(&self, id: FiefId) -> Result<usize>;
    async fn diff_count(&self, id: FiefId) -> Result<usize>;

    // [U]pdate
    // - self or fields
    async fn update_last_check(&self, id: FiefId) -> Result<()>;
    async fn set_check_interval(&self, id: FiefId, interval: chrono::Duration) -> Result<()>;
    async fn skip_check(&self, id: FiefId) -> Result<()>;
    async fn skip_check_for(&self, id: FiefId, dur: chrono::Duration) -> Result<()>;
    async fn set_name(&self, id: FiefId, name: &str) -> Result<()>;
    // - related
    // *PASS*

    // [D]elete
    async fn remove_by_id(&self, id: FiefId) -> Result<bool>;
    async fn remove_by_name(&self, name: &str) -> Result<bool>;
}


