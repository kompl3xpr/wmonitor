#[derive(Debug, sqlx::FromRow)]
pub struct Event {
    pub id: i64,
    pub date: chrono::DateTime<chrono::Utc>,
    pub kind: String,
    pub value: sqlx::types::JsonValue,
}

mod test {

    #[test]
    fn it_can_be_compiled() {
        let _ = <super::Event as sqlx::FromRow<crate::utils::db::CurrentRow>>::from_row;
    }
}