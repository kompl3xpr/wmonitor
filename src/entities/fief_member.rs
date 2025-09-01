#[derive(Debug, sqlx::FromRow)]
pub struct FiefMember {
    pub fief_id: i64,
    pub user_id: i64,
    pub permissions: i64,
}

mod test {

    #[test]
    fn it_can_be_compiled() {
        let _ = <super::FiefMember as sqlx::FromRow<crate::utils::db::CurrentRow>>::from_row;
    }
}