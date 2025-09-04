use super::new_repo;
use std::collections::HashSet;
use chrono::TimeZone;
use wmonitor::{
    cfg,
    domains::{Fief, FiefId, Position, UserId},
};

// [C]reate
#[tokio::test]
async fn create() {
    let repo = new_repo().await;

    repo.fief().fief_by_name("协会横幅").await.unwrap_err();

    let success = repo
        .fief()
        .create("协会横幅", Some(chrono::Duration::seconds(419)))
        .await
        .unwrap();
    assert!(success);

    let result = repo.fief().fief_by_name("协会横幅").await.unwrap();
    let min = cfg().checker.minimum_interval_min as i64;
    assert_eq!(
        result.check_interval,
        chrono::Duration::minutes(6.max(min))
    );

    let success = repo.fief().create("协会横幅", None).await.unwrap();
    assert!(!success);
}

// [R]ead
// - self or fields
#[tokio::test]
async fn name() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    repo.fief().create("布莉姬特", None).await.unwrap();

    let id1 = repo.fief().id("协会横幅").await.unwrap();
    let id2 = repo.fief().id("布莉姬特").await.unwrap();
    let name1 = repo.fief().name(id1).await.unwrap();
    let name2 = repo.fief().name(id2).await.unwrap();
    assert_eq!(name1, "协会横幅".to_owned());
    assert_eq!(name2, "布莉姬特".to_owned());

    repo.fief().name(FiefId(1145141919810)).await.unwrap_err();
}

#[tokio::test]
async fn id() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    repo.fief().create("布莉姬特", None).await.unwrap();

    let id1 = repo.fief().id("协会横幅").await.unwrap();
    let id2 = repo.fief().id("布莉姬特").await.unwrap();
    assert_ne!(id1, id2);

    repo.fief().id("初音未来").await.unwrap_err();
}

#[tokio::test]
async fn fief_by_id() {
    let repo = new_repo().await;

    repo.fief().fief_by_id(FiefId(1145141919810)).await.unwrap_err();

    repo.fief().create("协会横幅", None).await.unwrap();
    let id = repo.fief().id("协会横幅").await.unwrap();
    let fief = repo.fief().fief_by_id(id).await.unwrap();
    assert_eq!(fief.name, "协会横幅".to_owned());
}

#[tokio::test]
async fn fief_by_name() {
    let repo = new_repo().await;

    repo.fief().fief_by_name("协会横幅").await.unwrap_err();

    repo.fief().create("协会横幅", None).await.unwrap();
    let id = repo.fief().id("协会横幅").await.unwrap();
    let fief = repo.fief().fief_by_name("协会横幅").await.unwrap();
    assert_eq!(fief.id, id);
}

#[tokio::test]
async fn all() {
    let repo = new_repo().await;

    let expect = HashSet::new();
    let actual = repo.fief().all().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect());

    repo.fief().create("协会横幅", None).await.unwrap();
    repo.fief().create("布莉姬特", None).await.unwrap();

    let expect = HashSet::<String>::from_iter(["协会横幅".to_owned(), "布莉姬特".to_owned()]);
    let actual = repo.fief().all().await.unwrap();
    assert_eq!(expect, actual.into_iter().map(|f| f.name).collect());
}

#[tokio::test]
async fn fiefs_to_check() {
    let repo = new_repo().await;

    let expect = HashSet::new();
    let actual = repo.fief().fiefs_to_check().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect());

    repo.fief().create("协会横幅", None).await.unwrap();
    repo.fief().create("布莉姬特", None).await.unwrap();
    let expect = HashSet::<String>::from_iter(["协会横幅".to_owned(), "布莉姬特".to_owned()]);
    let actual = repo.fief().fiefs_to_check().await.unwrap();
    assert_eq!(expect, actual.into_iter().map(|f| f.name).collect());

    let id1 = repo.fief().id("协会横幅").await.unwrap();
    let id2 = repo.fief().id("布莉姬特").await.unwrap();
    repo.fief().update_last_check(id1, None).await.unwrap();
    let expect = HashSet::<String>::from_iter(["布莉姬特".to_owned()]);
    let actual = repo.fief().fiefs_to_check().await.unwrap();
    assert_eq!(expect, actual.into_iter().map(|f| f.name).collect());

    repo.fief().skip_check(id2).await.unwrap();
    let expect = HashSet::new();
    let actual = repo.fief().fiefs_to_check().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect());

    repo.fief().keep_check(id2).await.unwrap();
    let expect = HashSet::<String>::from_iter(["布莉姬特".to_owned()]);
    let actual = repo.fief().fiefs_to_check().await.unwrap();
    assert_eq!(expect, actual.into_iter().map(|f| f.name).collect());
}

// - related
#[tokio::test]
async fn members() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let id = repo.fief().id("协会横幅").await.unwrap();

    let members = repo.fief().members(id).await.unwrap();
    assert_eq!(members, vec![]);

    repo.user().create(UserId(114514), false).await.unwrap();
    repo.user().join(UserId(114514), id, None).await.unwrap();
    repo.user().create(UserId(1919810), true).await.unwrap();
    repo.user().join(UserId(1919810), id, None).await.unwrap();
    let actual = repo.fief().members(id).await.unwrap();
    let expect = HashSet::<UserId>::from_iter([UserId(114514), UserId(1919810)]);
    assert_eq!(expect, HashSet::from_iter(actual));

    repo.user().leave(UserId(1919810), id).await.unwrap();
    let actual = repo.fief().members(id).await.unwrap();
    let expect = HashSet::<UserId>::from_iter([UserId(114514)]);
    assert_eq!(expect, HashSet::from_iter(actual));
}

#[tokio::test]
async fn chunks() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let id = repo.fief().id("协会横幅").await.unwrap();

    assert!(repo.fief().chunks(id).await.unwrap().is_empty());

    let pos = Position::new(114, 514);
    repo.chunk().create("左侧", id, pos).await.unwrap();
    assert_eq!(repo.fief().chunks(id).await.unwrap().len(), 1);

    let pos = Position::new(114, 515);
    repo.chunk().create("右侧", id, pos).await.unwrap();
    assert_eq!(repo.fief().chunks(id).await.unwrap().len(), 2);
}

#[tokio::test]
async fn chunk_count() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let id = repo.fief().id("协会横幅").await.unwrap();

    assert_eq!(repo.fief().chunk_count(id).await.unwrap(), 0);

    let pos = Position::new(114, 514);
    repo.chunk().create("左侧", id, pos).await.unwrap();
    assert_eq!(repo.fief().chunk_count(id).await.unwrap(), 1);

    let pos = Position::new(114, 515);
    repo.chunk().create("右侧", id, pos).await.unwrap();
    assert_eq!(repo.fief().chunk_count(id).await.unwrap(), 2);
}

#[tokio::test]
async fn diff_count() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let id = repo.fief().id("协会横幅").await.unwrap();

    assert_eq!(repo.fief().diff_count(id).await.unwrap(), 0);

    let pos = Position::new(114, 514);
    repo.chunk().create("左侧", id, pos).await.unwrap();
    let chunk_id = repo.chunk().id(id, "左侧").await.unwrap();
    repo.chunk().update_diff(chunk_id, None, 3).await.unwrap();
    assert_eq!(repo.fief().diff_count(id).await.unwrap(), 3);

    let pos = Position::new(114, 515);
    repo.chunk().create("右侧", id, pos).await.unwrap();
    let chunk_id = repo.chunk().id(id, "右侧").await.unwrap();
    repo.chunk().update_diff(chunk_id, None, 4).await.unwrap();
    assert_eq!(repo.fief().diff_count(id).await.unwrap(), 7);
}

// [U]pdate
// - self or fields
#[tokio::test]
async fn update_last_check() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let Fief { last_check, id, .. } = repo.fief().fief_by_name("协会横幅").await.unwrap();
    let old = last_check;

    repo.fief().update_last_check(id, None).await.unwrap();
    let Fief { last_check, .. } = repo.fief().fief_by_name("协会横幅").await.unwrap();
    let new = last_check;
    assert_ne!(old, new);
}

#[tokio::test]
async fn set_check_interval() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let id = repo.fief().id("协会横幅").await.unwrap();

    repo.fief()
        .set_check_interval(id, chrono::Duration::nanoseconds(1))
        .await
        .unwrap();
    let Fief { check_interval, .. } = repo.fief().fief_by_name("协会横幅").await.unwrap();
    let new = check_interval;

    let min = cfg().checker.minimum_interval_min as i64;
    assert_eq!(new.num_minutes(), min);

    repo.fief()
        .set_check_interval(id, chrono::Duration::weeks(1))
        .await
        .unwrap();
    let Fief { check_interval, .. } = repo.fief().fief_by_name("协会横幅").await.unwrap();
    let new = check_interval;
    assert_eq!(new.num_minutes(), chrono::Duration::weeks(1).num_minutes());
}

#[tokio::test]
async fn skip_check() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let Fief { skip_check_until, id, .. } = repo.fief().fief_by_name("协会横幅").await.unwrap();
    assert_eq!(skip_check_until, chrono::Utc.with_ymd_and_hms(1919, 11, 4, 5, 1, 4).unwrap());

    repo.fief().skip_check(id).await.unwrap();
    let Fief { skip_check_until, .. } = repo.fief().fief_by_name("协会横幅").await.unwrap();
    assert_eq!(skip_check_until, chrono::Utc.with_ymd_and_hms(2077, 1, 1, 0, 0, 0).unwrap());
}

#[tokio::test]
async fn keep_check() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let id = repo.fief().id("协会横幅").await.unwrap();
    repo.fief().skip_check(id).await.unwrap();
    let Fief { skip_check_until, .. } = repo.fief().fief_by_name("协会横幅").await.unwrap();
    assert_eq!(skip_check_until, chrono::Utc.with_ymd_and_hms(2077, 1, 1, 0, 0, 0).unwrap());

    repo.fief().keep_check(id).await.unwrap();
    let Fief { skip_check_until, .. } = repo.fief().fief_by_name("协会横幅").await.unwrap();
    assert_eq!(skip_check_until, chrono::Utc.with_ymd_and_hms(1919, 11, 4, 5, 1, 4).unwrap());
}

#[tokio::test]
async fn skip_check_for() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let id = repo.fief().id("协会横幅").await.unwrap();

    repo.fief().skip_check_for(id, chrono::Duration::seconds(1), None).await.unwrap();
    let expect = HashSet::new();
    let actual = repo.fief().fiefs_to_check().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect());

    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let expect = HashSet::<String>::from_iter(["协会横幅".to_owned()]);
    let actual = repo.fief().fiefs_to_check().await.unwrap();
    assert_eq!(expect, actual.into_iter().map(|f| f.name).collect());
}

#[tokio::test]
async fn set_name() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let id = repo.fief().id("协会横幅").await.unwrap();

    repo.fief().set_name(id, "协会横幅#0").await.unwrap();
    let name = repo.fief().name(id).await.unwrap();
    assert_eq!(name, "协会横幅#0".to_owned());
}

// - related
// *PASS*

// [D]elete
#[tokio::test]
async fn remove_by_id() {
    let repo = new_repo().await;

    repo.fief().create("协会横幅", None).await.unwrap();
    repo.fief().create("布莉姬特", None).await.unwrap();
    let id = repo.fief().id("布莉姬特").await.unwrap();

    use std::collections::HashSet;

    let result = repo.fief().remove_by_id(id).await.unwrap();
    assert!(result);

    let expect = HashSet::<String>::from_iter(["协会横幅".to_owned()]);
    let actual = repo.fief().all().await.unwrap();
    assert_eq!(expect, actual.into_iter().map(|f| f.name).collect());

    let result = repo.fief().remove_by_id(id).await.unwrap();
    assert!(!result);
}

#[tokio::test]
async fn remove_by_name() {
    let repo = new_repo().await;

    repo.fief().create("协会横幅", None).await.unwrap();
    repo.fief().create("布莉姬特", None).await.unwrap();

    use std::collections::HashSet;

    let result = repo.fief().remove_by_name("布莉姬特").await.unwrap();
    assert!(result);

    let expect = HashSet::<String>::from_iter(["协会横幅".to_owned()]);
    let actual = repo.fief().all().await.unwrap();
    assert_eq!(expect, actual.into_iter().map(|f| f.name).collect());

    let result = repo.fief().remove_by_name("布莉姬特").await.unwrap();
    assert!(!result);
}
