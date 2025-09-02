use sqlx;

#[derive(Debug, sqlx::FromRow)]
pub struct User {
    pub id: i64,
    pub is_admin: bool,
}

mod test {

    #[test]
    fn it_can_be_compiled() {
        let _ = <super::User as sqlx::FromRow<crate::utils::db::CurrentRow>>::from_row;
    }
}