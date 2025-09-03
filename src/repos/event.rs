use async_trait::async_trait;
use anyhow::Result;

pub(super) mod domains {
    use serde::{Deserialize, Serialize};
    use crate::domains::{FiefId, UserId};

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct EventId(pub i64);

    #[derive(Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct Event {
        id: EventId,
        date: chrono::DateTime<chrono::Utc>,
        kind: EventKind,
    }

    #[derive(Debug, Clone, Hash, Serialize, Deserialize)]
    pub enum EventKind {
        AppStart,
        AppStop,
        DiffFound(DiffFoundEvent),
        SkipCheckBegin(SkipCheckBeginEvent),
        SkipCheckEnd(SkipCheckEndEvent),
        AppError(AppErrorEvent),
        CheckError(CheckErrorEvent),
    }

    #[derive(Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct DiffFoundEvent {
        fief: FiefId,
        diff_count: usize,
    }

    #[derive(Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct SkipCheckBeginEvent {
        fief: FiefId,
        who: UserId,
        until: chrono::DateTime<chrono::Utc>,
    }  

    #[derive(Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct SkipCheckEndEvent {
        fief: FiefId,
        is_cancellation: bool,
    }

    #[derive(Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct AppErrorEvent {
        description: String,
    }

    #[derive(Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct CheckErrorEvent {
        fief: FiefId,
        description: String,
    }
}
use domains::*;

#[async_trait]
pub trait EventRepo {
    // [C]reate
    async fn save(&self, kind: EventKind) -> Result<()>;

    // [R]ead
    // - self or fields
    async fn kind(&self, id: EventId) -> Result<EventKind>;
    async fn date(&self, id: EventId) -> Result<chrono::DateTime<chrono::Utc>>;
    async fn event_by_id(&self, id: EventId) -> Result<Event>;

    // [U]pdate
    // *PASS*
    
    // [D]elete
    async fn remove_by_id(&self, id: EventId) -> Result<()>;
    async fn remove_all_by_kind(&self, kind: &str) -> Result<()>;
    async fn remove_all_before(&self, date: chrono::DateTime<chrono::Utc>) -> Result<()>;
    async fn remove_all(&self) -> Result<()>;
}

