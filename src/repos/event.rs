use anyhow::Result;
use async_trait::async_trait;

pub(super) mod domains {
    use crate::domains::{FiefId, UserId};
    use anyhow::Result;
    use serde::{Deserialize, Serialize};
    use serde_json::{from_value, to_value};
    use sqlx::types::JsonValue;

    #[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy, Hash, Serialize, Deserialize)]
    pub struct EventId(pub i64);

    impl From<i64> for EventId {
        fn from(value: i64) -> Self {
            Self(value)
        }
    }

    #[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct Event {
        pub id: EventId,
        pub date: chrono::DateTime<chrono::Utc>,
        pub kind: EventKind,
    }

    #[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
    pub enum EventKind {
        AppStart,
        AppStop,
        DiffFound(DiffFoundEvent),
        SkipCheckBegin(SkipCheckBeginEvent),
        SkipCheckEnd(SkipCheckEndEvent),
        AppError(AppErrorEvent),
        CheckError(CheckErrorEvent),
    }

    impl EventKind {
        pub fn into_raw(self) -> Result<(&'static str, JsonValue)> {
            Ok(match self {
                EventKind::AppStart => ("APP_START", JsonValue::Null),
                EventKind::AppStop => ("APP_STOP", JsonValue::Null),
                EventKind::DiffFound(ev) => ("DIFF_FOUND", to_value(ev)?),
                EventKind::SkipCheckBegin(ev) => ("SKIP_CHECK_BEGIN", to_value(ev)?),
                EventKind::SkipCheckEnd(ev) => ("SKIP_CHECK_END", to_value(ev)?),
                EventKind::AppError(ev) => ("APP_ERROR", to_value(ev)?),
                EventKind::CheckError(ev) => ("CHECK_ERROR", to_value(ev)?),
            })
        }

        pub fn from_raw<K: AsRef<str>>(kind: &K, json_value: JsonValue) -> Result<Self> {
            use EventKind::*;

            Ok(match kind.as_ref() {
                "APP_START" => AppStart,
                "APP_STOP" => AppStop,
                "DIFF_FOUND" => DiffFound(from_value(json_value)?),
                "SKIP_CHECK_BEGIN" => SkipCheckBegin(from_value(json_value)?),
                "SKIP_CHECK_END" => SkipCheckEnd(from_value(json_value)?),
                "APP_ERROR" => AppError(from_value(json_value)?),
                "CHECK_ERROR" => CheckError(from_value(json_value)?),
                _ => return Err(anyhow::anyhow!("invalid event kind")),
            })
        }
    }

    #[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct DiffFoundEvent {
        pub fief: FiefId,
        pub diff_count: usize,
    }

    #[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct SkipCheckBeginEvent {
        pub fief: FiefId,
        pub who: UserId,
        pub until: chrono::DateTime<chrono::Utc>,
    }

    #[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct SkipCheckEndEvent {
        pub fief: FiefId,
        pub is_cancellation: bool,
    }

    #[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct AppErrorEvent {
        pub description: String,
    }

    #[derive(PartialEq, Eq, Debug, Clone, Hash, Serialize, Deserialize)]
    pub struct CheckErrorEvent {
        pub description: String,
    }
}
use domains::*;

#[async_trait]
pub trait EventRepo: Sync + Send {
    fn clone(&self) -> Box<dyn EventRepo>;

    // [C]reate
    async fn save(&self, kind: EventKind) -> Result<Option<EventId>>;

    // [R]ead
    // - self or fields
    async fn kind(&self, id: EventId) -> Result<EventKind>;
    async fn date(&self, id: EventId) -> Result<chrono::DateTime<chrono::Utc>>;
    async fn event_by_id(&self, id: EventId) -> Result<Event>;
    async fn all(&self) -> Result<Vec<Event>>;
    async fn all_by_kind(&self, kind: &str) -> Result<Vec<Event>>;
    async fn all_before(&self, date: chrono::DateTime<chrono::Utc>) -> Result<Vec<Event>>;

    // [U]pdate
    // *PASS*

    // [D]elete
    async fn remove_by_id(&self, id: EventId) -> Result<bool>;
    async fn remove_all_by_kind(&self, kind: &str) -> Result<bool>;
    async fn remove_all_before(&self, date: chrono::DateTime<chrono::Utc>) -> Result<bool>;
    async fn remove_all(&self) -> Result<bool>;
}
