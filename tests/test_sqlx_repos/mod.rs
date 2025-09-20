mod test_chunk;
mod test_fief;
mod test_user;

use wmonitor::Repositories;
async fn new_repo() -> Repositories {
    Repositories::from_sqlx("sqlite::memory:").await.unwrap()
}
