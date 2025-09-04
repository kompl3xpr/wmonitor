use wmonitor::domains::{Permissions, User, UserId};
use super::new_repo;

fn new_user(id: i64, is_admin: bool) -> User {
    let id = UserId(id);
    User { id, is_admin }
}

#[tokio::test]
async fn create() {
    let repo = new_repo().await;

    let users = repo.user().all().await.unwrap();
    assert_eq!(users, vec![]);

    let result = repo.user().create(UserId(114514), true).await.unwrap();
    assert!(result);

    let users = repo.user().all().await.unwrap();
    assert_eq!(users, vec![new_user(114514, true)]);

    // expect: error
    let result = repo.user().create(UserId(114514), false).await.unwrap();
    assert!(!result);

    let users = repo.user().all().await.unwrap();
    assert_eq!(users, vec![new_user(114514, true)]);
}

#[tokio::test]
async fn user_by_id() {
    let repo = new_repo().await;

    let user = repo.user().user_by_id(UserId(114514)).await.unwrap();
    assert_eq!(user, None);

    repo.user().create(UserId(114514), true).await.unwrap();

    let user = repo.user().user_by_id(UserId(114514)).await.unwrap();
    assert_eq!(user, Some(new_user(114514, true)));
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
async fn fiefs() {
    let repo = new_repo().await;
    repo.user().create(UserId(114514), false).await.unwrap();
    repo.fief().create("协会横幅", None).await.unwrap();

    let actual = repo.user().fiefs(UserId(114514)).await.unwrap();
    assert_eq!(actual, vec![]);

    let fief_id = repo.fief().id("协会横幅").await.unwrap().unwrap();
    repo.user().join(UserId(114514), fief_id, None).await.unwrap();
    let actual = repo.user().fiefs(UserId(114514)).await.unwrap();
    assert_eq!(actual, vec![fief_id]);

    repo.user().leave(UserId(114514), fief_id).await.unwrap();
    let actual = repo.user().fiefs(UserId(114514)).await.unwrap();
    assert_eq!(actual, vec![]);
}

#[tokio::test]
async fn is_member_of() {
    let repo = new_repo().await;
    repo.user().create(UserId(114514), false).await.unwrap();
    repo.fief().create("协会横幅", None).await.unwrap();
    let fief_id = repo.fief().id("协会横幅").await.unwrap().unwrap();

    let is_member = repo.user().is_member_of(UserId(114514), fief_id).await.unwrap();
    assert!(!is_member);

    repo.user().join(UserId(114514), fief_id, None).await.unwrap();
    let is_member = repo.user().is_member_of(UserId(114514), fief_id).await.unwrap();
    assert!(is_member);
}

#[tokio::test]
async fn permissions_in() {
    let repo = new_repo().await;
    repo.user().create(UserId(114514), false).await.unwrap();
    repo.fief().create("协会横幅", None).await.unwrap();
    let fief_id = repo.fief().id("协会横幅").await.unwrap().unwrap();
    let p = Permissions::CHUNK_ALL;
    repo.user().join(UserId(114514), fief_id, Some(p)).await.unwrap();

    let expect = Permissions::CHUNK_ADD | Permissions::CHUNK_EDIT | Permissions::CHUNK_DELETE;
    let actual = repo.user().permissions_in(UserId(114514), fief_id).await.unwrap();
    assert_eq!(actual, expect);
}

#[tokio::test]
async fn set_admin() {
    let repo = new_repo().await;
    
    repo.user().set_admin(UserId(114514), true).await.unwrap();

    repo.user().create(UserId(114514), false).await.unwrap();
    let User {is_admin, ..} = repo.user().user_by_id(UserId(114514)).await.unwrap().unwrap();
    assert!(!is_admin);

    repo.user().set_admin(UserId(114514), true).await.unwrap();
    let User {is_admin, ..} = repo.user().user_by_id(UserId(114514)).await.unwrap().unwrap();
    assert!(is_admin);
}

#[tokio::test]
async fn set_permissions_in() {
    let repo = new_repo().await;
    repo.user().create(UserId(114514), false).await.unwrap();
    repo.fief().create("协会横幅", None).await.unwrap();
    let fief_id = repo.fief().id("协会横幅").await.unwrap().unwrap();
    repo.user().join(UserId(114514), fief_id, None).await.unwrap();
    let expect = Permissions::NONE;
    let actual = repo.user().permissions_in(UserId(114514), fief_id).await.unwrap();
    assert_eq!(actual, expect);

    let p = Permissions::ALL - Permissions::MEMBER_KICK - Permissions::MEMBER_EDIT_PERMS;
    repo.user().set_permissions_in(UserId(114514), fief_id, p).await.unwrap();

    let expect = Permissions::FIEF_ALL | Permissions::CHUNK_ALL | Permissions::MEMBER_INVITE;
    let actual = repo.user().permissions_in(UserId(114514), fief_id).await.unwrap();
    assert_eq!(actual, expect);
}

#[tokio::test]
async fn join() {
    let repo = new_repo().await;
    repo.user().create(UserId(114514), false).await.unwrap();
    repo.fief().create("协会横幅", None).await.unwrap();

    let actual = repo.user().fiefs(UserId(114514)).await.unwrap();
    assert_eq!(actual, vec![]);

    let fief_id = repo.fief().id("协会横幅").await.unwrap().unwrap();
    let success = repo.user().join(UserId(114514), fief_id, None).await.unwrap();
    assert!(success);
    let actual = repo.user().fiefs(UserId(114514)).await.unwrap();
    assert_eq!(actual, vec![fief_id]);

    let success = repo.user().join(UserId(114514), fief_id, None).await.unwrap();
    assert!(!success);
    let actual = repo.user().fiefs(UserId(114514)).await.unwrap();
    assert_eq!(actual, vec![fief_id]);
}

#[tokio::test]
async fn leave() {
    let repo = new_repo().await;
    repo.user().create(UserId(114514), false).await.unwrap();
    repo.fief().create("协会横幅", None).await.unwrap();
    let fief_id = repo.fief().id("协会横幅").await.unwrap().unwrap();
    repo.user().join(UserId(114514), fief_id, None).await.unwrap();
    let actual = repo.user().fiefs(UserId(114514)).await.unwrap();
    assert_eq!(actual, vec![fief_id]);

    let success = repo.user().leave(UserId(114514), fief_id).await.unwrap();
    assert!(success);
    let actual = repo.user().fiefs(UserId(114514)).await.unwrap();
    assert_eq!(actual, vec![]);

    let success = repo.user().leave(UserId(114514), fief_id).await.unwrap();
    assert!(!success);
    let actual = repo.user().fiefs(UserId(114514)).await.unwrap();
    assert_eq!(actual, vec![]);
}

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
