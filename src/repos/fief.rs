use crate::domains::ChunkId;
use anyhow::Result;
use async_trait::async_trait;

pub(super) mod domains {
    use serde::{Deserialize, Serialize};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct FiefId(pub i64);

    #[derive(Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct Fief {
        pub id: FiefId,
        pub name: String,
        pub check_duration: chrono::Duration,
        pub last_check: chrono::DateTime<chrono::Utc>,
        pub skip_check_until: chrono::DateTime<chrono::Utc>,
    }
}
use domains::*;

#[async_trait]
pub trait FiefRepo {
    async fn name_of(&self, id: FiefId) -> Result<String>;
    async fn id_of(&self, name: &str) -> Result<FiefId>;
    async fn fief_by_id(&self, id: FiefId) -> Result<Fief>;
    async fn fief_by_name(&self, name: &str) -> Result<Fief>;
    async fn create(&self, name: &str, check_dur: chrono::Duration) -> Result<bool>;
    async fn remove_by_id(&self, id: FiefId) -> Result<bool>;
    async fn remove_by_name(&self, id: FiefId) -> Result<bool>;
    async fn members(&self, id: FiefId) -> Result<bool>;
    async fn update_last_check(&self, id: FiefId) -> Result<()>;
    async fn set_check_duration(&self, id: FiefId, check_dur: chrono::Duration) -> Result<()>;
    async fn skip_check(&self, id: FiefId) -> Result<()>;
    async fn skip_check_for(&self, id: FiefId, dur: chrono::Duration) -> Result<()>;
    async fn set_name(&self, id: FiefId, name: &str) -> Result<()>;
    async fn fiefs_to_check(&self) -> Result<Vec<FiefId>>;
    async fn chunks(&self, id: FiefId) -> Result<Vec<ChunkId>>;
    async fn chunk_count(&self, id: FiefId) -> Result<usize>;
    async fn diff_count(&self, id: FiefId) -> Result<usize>;
}

pub struct SqlxFiefRepo {}
