mod chunk;
mod event;
mod fief;
mod user;

use wmonitor::Repositories;
async fn new_repo() -> Repositories {
    Repositories::from_sqlx("sqlite::memory:").await.unwrap()
}
