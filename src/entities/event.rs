#[derive(Debug, sqlx::FromRow)]
pub struct Event {
    pub id: i32,
    pub date: chrono::DateTime<chrono::Utc>,
    pub value: sqlx::types::JsonValue,
}