#[derive(Debug, sqlx::FromRow)]
pub struct Fief {
    pub id: i32,
    pub name: String,
    pub check_duration_ms: i32,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub skip_check_until: chrono::DateTime<chrono::Utc>,
}