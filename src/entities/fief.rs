#[derive(Debug, sqlx::FromRow)]
pub struct Fief {
    pub id: i64,
    pub name: String,
    pub check_duration_ms: i64,
    pub last_check: chrono::DateTime<chrono::Utc>,
    pub skip_check_until: chrono::DateTime<chrono::Utc>,
}

mod test {

    #[test]
    fn it_can_be_compiled() {
        let _ = <super::Fief as sqlx::FromRow<crate::utils::db::CurrentRow>>::from_row;
    }
}