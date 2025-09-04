use super::new_repo;
use wmonitor::domains::{Fief, FiefId};

// [C]reate
#[tokio::test]
async fn create() {
    let repo = new_repo().await;

    let result = repo.fief().fief_by_name("协会横幅").await.unwrap();
    assert_eq!(result, None);

    let success = repo.fief().create("协会横幅", Some(chrono::Duration::seconds(419))).await.unwrap();
    assert!(success);

    let result = repo.fief().fief_by_name("协会横幅").await.unwrap();
    assert_eq!(result.unwrap().check_interval, chrono::Duration::minutes(6));
}


// [R]ead
// - self or fields
#[tokio::test]
async fn name() {

}

#[tokio::test]
async fn id() {
}

#[tokio::test]
async fn fief_by_id() {
}

#[tokio::test]
async fn fief_by_name() {
}

#[tokio::test]
async fn fiefs_to_check() {
}

#[tokio::test]
async fn all() {
}

// - related
#[tokio::test]
async fn members() {
}

#[tokio::test]
async fn chunks() {
}

#[tokio::test]
async fn chunk_count() {
}

#[tokio::test]
async fn diff_count() {
}


// [U]pdate
// - self or fields
#[tokio::test]
async fn update_last_check() {
}

#[tokio::test]
async fn set_check_interval() {
}

#[tokio::test]
async fn skip_check() {
}

#[tokio::test]
async fn skip_check_for() {
}

#[tokio::test]
async fn set_name() {
}

// - related
// *PASS*

// [D]elete
#[tokio::test]
async fn remove_by_id() {
}

#[tokio::test]
async fn remove_by_name() {
}

