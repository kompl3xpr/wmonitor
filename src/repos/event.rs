use async_trait::async_trait;

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
        DiffFound(DiffFoundEvent),
        AppStart,
        AppStop,
        SkipCheckBegin(SkipCheckBeginEvent),
        SkipCheckEndEvent(SkipCheckEndEvent),
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
        is_cancellation: bool,
    }
}
use domains::*;

#[async_trait]
pub trait EventRepo {}

pub struct SqlxEventRepo {}
