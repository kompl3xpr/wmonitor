use async_trait::async_trait;

pub struct User;

#[async_trait]
pub trait UserRepo {
    async fn user_by_name(name: &str) -> User;
}

pub struct SqlxUserRepo {

}