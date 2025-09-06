use super::new_repo;
use std::collections::HashSet;
use wmonitor::domains::{CheckErrorEvent, DiffFoundEvent, Event, EventId, EventKind, FiefId};

fn kinds() -> Vec<EventKind> {
    Vec::<EventKind>::from_iter([
        EventKind::DiffFound(DiffFoundEvent {
            fief: FiefId(114514),
            diff_count: 99,
        }),
        EventKind::CheckError(CheckErrorEvent {
            description: "network error".to_owned(),
        }),
        EventKind::AppStop,
    ])
}

// [C]reate
#[tokio::test]
async fn save() {
    let repo = new_repo().await;

    assert!(repo.event().all().await.unwrap().is_empty());

    let kind = EventKind::AppStart;
    let id = repo.event().save(kind).await.unwrap();
    assert!(id.is_some());

    assert!(repo.event().all().await.unwrap().len() == 1);
}

// [R]ead
// - self or fields
#[tokio::test]
async fn kind() {
    let repo = new_repo().await;

    repo.event().kind(EventId(114514)).await.unwrap_err();

    let kind = EventKind::AppStart;
    let id = repo.event().save(kind.clone()).await.unwrap().unwrap();

    assert_eq!(repo.event().kind(id).await.unwrap(), kind);
}

#[tokio::test]
async fn date() {
    let repo = new_repo().await;

    repo.event().date(EventId(114514)).await.unwrap_err();

    let now = chrono::Utc::now();
    let kind = EventKind::AppStart;
    let id = repo.event().save(kind.clone()).await.unwrap().unwrap();

    let date = repo.event().date(id).await.unwrap();
    let dur = (date - now).num_seconds();
    assert!(dur <= 1);
}

#[tokio::test]
async fn event_by_id() {
    let repo = new_repo().await;

    repo.event().event_by_id(EventId(114514)).await.unwrap_err();

    let kind = EventKind::AppStop;
    let id = repo.event().save(kind.clone()).await.unwrap().unwrap();

    let event = repo.event().event_by_id(id).await.unwrap();
    assert_eq!(event.kind, kind);
}

#[tokio::test]
async fn all() {
    let repo = new_repo().await;

    let expect = HashSet::new();
    let actual = repo.event().all().await.unwrap();
    assert_eq!(expect, actual.into_iter().collect::<HashSet<Event>>());

    let kinds = kinds();
    repo.event().save(kinds[0].clone()).await.unwrap();
    repo.event().save(kinds[1].clone()).await.unwrap();
    repo.event().save(kinds[2].clone()).await.unwrap();

    use std::collections::HashSet;

    let actual = repo.event().all().await.unwrap();
    assert_eq!(
        kinds.into_iter().collect::<HashSet<_>>(),
        actual.into_iter().map(|ev| ev.kind).collect::<HashSet<_>>()
    );
}

#[tokio::test]
async fn all_by_kind() {
    let repo = new_repo().await;

    let expect = HashSet::new();
    let actual = repo.event().all_by_kind("DIFF_FOUND").await.unwrap();
    assert_eq!(expect, actual.into_iter().collect::<HashSet<Event>>());

    let kinds = kinds();
    repo.event().save(kinds[0].clone()).await.unwrap();
    repo.event().save(kinds[1].clone()).await.unwrap();
    repo.event().save(kinds[2].clone()).await.unwrap();

    use std::collections::HashSet;

    let actual = repo.event().all_by_kind("DIFF_FOUND").await.unwrap();
    assert_eq!(
        kinds.into_iter().take(1).collect::<HashSet<_>>(),
        actual.into_iter().map(|ev| ev.kind).collect::<HashSet<_>>()
    );
}

#[tokio::test]
async fn all_before() {
    let repo = new_repo().await;

    let expect = HashSet::new();
    let actual = repo.event().all_before(chrono::Utc::now()).await.unwrap();
    assert_eq!(expect, actual.into_iter().collect::<HashSet<Event>>());

    let kinds = kinds();
    repo.event().save(kinds[0].clone()).await.unwrap();
    repo.event().save(kinds[1].clone()).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let datetime = chrono::Utc::now();
    repo.event().save(kinds[2].clone()).await.unwrap();

    use std::collections::HashSet;

    let actual = repo.event().all_before(datetime).await.unwrap();
    assert_eq!(
        kinds.into_iter().take(2).collect::<HashSet<_>>(),
        actual.into_iter().map(|ev| ev.kind).collect::<HashSet<_>>()
    );
}

// [U]pdate
// *PASS*

// [D]elete
#[tokio::test]
async fn remove_by_id() {
    let repo = new_repo().await;

    let kinds = kinds();
    repo.event().save(kinds[0].clone()).await.unwrap();
    repo.event().save(kinds[1].clone()).await.unwrap();
    let id = repo.event().save(kinds[2].clone()).await.unwrap().unwrap();

    let result = repo.event().remove_by_id(id).await.unwrap();
    assert!(result);

    let actual = repo.event().all().await.unwrap();
    assert_eq!(
        HashSet::from_iter([kinds[0].clone(), kinds[1].clone()]),
        actual.into_iter().map(|ev| ev.kind).collect::<HashSet<_>>()
    );

    let result = repo.event().remove_by_id(id).await.unwrap();
    assert!(!result);
}

#[tokio::test]
async fn remove_all_by_kind() {
    let repo = new_repo().await;

    let kinds = kinds();

    repo.event().save(kinds[0].clone()).await.unwrap();
    repo.event().save(kinds[1].clone()).await.unwrap();
    repo.event().save(kinds[2].clone()).await.unwrap().unwrap();

    let result = repo
        .event()
        .remove_all_by_kind("CHECK_ERROR")
        .await
        .unwrap();
    assert!(result);

    let actual = repo.event().all().await.unwrap();
    assert_eq!(
        HashSet::from_iter([kinds[0].clone(), kinds[2].clone()]),
        actual.into_iter().map(|ev| ev.kind).collect::<HashSet<_>>()
    );

    let result = repo
        .event()
        .remove_all_by_kind("CHECK_ERROR")
        .await
        .unwrap();
    assert!(!result);
}

#[tokio::test]
async fn remove_all_before() {
    let repo = new_repo().await;

    let kinds = kinds();
    repo.event().save(kinds[0].clone()).await.unwrap();
    repo.event().save(kinds[1].clone()).await.unwrap();
    tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    let datetime = chrono::Utc::now();
    repo.event().save(kinds[2].clone()).await.unwrap().unwrap();

    let result = repo.event().remove_all_before(datetime).await.unwrap();
    assert!(result);

    let actual = repo.event().all().await.unwrap();
    assert_eq!(
        HashSet::from_iter([kinds[2].clone()]),
        actual.into_iter().map(|ev| ev.kind).collect::<HashSet<_>>()
    );

    let result = repo.event().remove_all_before(datetime).await.unwrap();
    assert!(!result);
}

#[tokio::test]
async fn remove_all() {
    let repo = new_repo().await;

    let kinds = kinds();
    repo.event().save(kinds[0].clone()).await.unwrap();
    repo.event().save(kinds[1].clone()).await.unwrap();
    repo.event().save(kinds[2].clone()).await.unwrap().unwrap();

    let result = repo.event().remove_all().await.unwrap();
    assert!(result);

    let actual = repo.event().all().await.unwrap();
    assert_eq!(HashSet::new(), actual.into_iter().collect::<HashSet<_>>());

    let result = repo.event().remove_all().await.unwrap();
    assert!(!result);
}
