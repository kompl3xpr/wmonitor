use wmonitor::Repositories;
use wmonitor::domains::{User, UserId};

async fn new_repo() -> Repositories {
    Repositories::from_sqlx("sqlite::memory:").await.unwrap()
}

fn new_user(id: i64, is_admin: bool) -> User {
    let id = UserId(id);
    User { id, is_admin }
}

#[tokio::test]
async fn create() {
    let repo = new_repo().await;

    let users = repo.user().all().await.unwrap();
    assert_eq!(users, vec![]);

    // expect: ok
    repo.user().create(UserId(114514), true).await.unwrap();

    let users = repo.user().all().await.unwrap();
    assert_eq!(users, vec![new_user(114514, true)]);

    // expect: error
    repo.user().create(UserId(114514), false).await.unwrap_err();

    let users = repo.user().all().await.unwrap();
    assert_eq!(users, vec![new_user(114514, true)]);
}

#[tokio::test]
async fn user_by_id() {
    let repo = new_repo().await;
    // expect: error
    repo.user().user_by_id(UserId(114514)).await.unwrap_err();

    repo.user().create(UserId(114514), true).await.unwrap();

    // expect: ok
    let user = repo.user().user_by_id(UserId(114514)).await.unwrap();
    assert_eq!(user, new_user(114514, true))
}

#[tokio::test]
async fn all() {
    let repo = new_repo().await;

    let expect = HashSet::new();
    let actual = repo.user().all().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect::<HashSet<User>>());

    repo.user().create(UserId(114514), false).await.unwrap();
    repo.user().create(UserId(1919), true).await.unwrap();
    repo.user().create(UserId(810), false).await.unwrap();

    use std::collections::HashSet;
    let expect = HashSet::<User>::from_iter([
        new_user(114514, false),
        new_user(1919, true),
        new_user(810, false),
    ]);
    let actual = repo.user().all().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect::<HashSet<User>>());
}

#[tokio::test]
async fn admins() {
    let repo = new_repo().await;

    let expect = HashSet::new();
    let actual = repo.user().admins().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect::<HashSet<User>>());

    repo.user().create(UserId(114514), false).await.unwrap();
    repo.user().create(UserId(1919), true).await.unwrap();
    repo.user().create(UserId(810), false).await.unwrap();

    use std::collections::HashSet;
    let expect = HashSet::<User>::from_iter([new_user(1919, true)]);
    let actual = repo.user().admins().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect::<HashSet<User>>());
}

#[tokio::test]
async fn non_admins() {
    let repo = new_repo().await;

    let expect = HashSet::new();
    let actual = repo.user().non_admins().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect::<HashSet<User>>());

    repo.user().create(UserId(114514), false).await.unwrap();
    repo.user().create(UserId(1919), true).await.unwrap();
    repo.user().create(UserId(810), false).await.unwrap();

    use std::collections::HashSet;
    let expect = HashSet::<User>::from_iter([
        new_user(114514, false),
        new_user(810, false),
    ]);
    let actual = repo.user().non_admins().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect::<HashSet<User>>());
}

#[tokio::test]
async fn fiefs() {}

#[tokio::test]
async fn is_member_of() {}

#[tokio::test]
async fn permissions_in() {}

#[tokio::test]
async fn set_admin() {
    let repo = new_repo().await;
    
    let result = repo.user().set_admin(UserId(114514), true).await.unwrap();
    assert!(!result);

    repo.user().create(UserId(114514), false).await.unwrap();
    let User {is_admin, ..} = repo.user().user_by_id(UserId(114514)).await.unwrap();
    assert!(!is_admin);

    let result = repo.user().set_admin(UserId(114514), true).await.unwrap();
    let User {is_admin, ..} = repo.user().user_by_id(UserId(114514)).await.unwrap();
    assert!(result);
    assert!(is_admin);

    let result = repo.user().set_admin(UserId(114514), true).await.unwrap();
    assert!(result);
}

#[tokio::test]
async fn set_permissions_in() {}

#[tokio::test]
async fn join() {}

#[tokio::test]
async fn leave() {}

#[tokio::test]
async fn remove_by_id() {
    let repo = new_repo().await;

    repo.user().create(UserId(114514), false).await.unwrap();
    repo.user().create(UserId(1919), true).await.unwrap();
    repo.user().create(UserId(810), false).await.unwrap();

    use std::collections::HashSet;

    let result = repo.user().remove_by_id(UserId(1919)).await.unwrap();
    assert!(result);

    let expect = HashSet::<User>::from_iter([
        new_user(114514, false),
        new_user(810, false),
    ]);
    let actual = repo.user().all().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect::<HashSet<User>>());

    let result = repo.user().remove_by_id(UserId(1919)).await.unwrap();
    assert!(!result);
}
