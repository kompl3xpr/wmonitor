#[derive(Debug, sqlx::FromRow)]
pub struct FiefMember {
    pub fief_id: i32,
    pub user_id: i32,
    pub permissions: i32,
}