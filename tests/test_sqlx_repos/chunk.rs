use super::new_repo;
use std::collections::HashSet;
use wmonitor::domains::{Chunk, ChunkId, FiefId, Position};

// [C]reate
#[tokio::test]
async fn create() {
    let repo = new_repo().await;
    let pos = Position::new(114, 514);

    let success = repo.chunk().create("左侧", FiefId(1), pos).await.unwrap();
    assert!(!success);

    repo.fief().create("协会横幅", None).await.unwrap();
    let id = repo.fief().id("协会横幅").await.unwrap();

    let success = repo.chunk().create("左侧", id, pos).await.unwrap();
    assert!(success);

    let success = repo.chunk().create("左侧", id, pos).await.unwrap();
    assert!(!success);
}

// [R]ead
// - self or fields
#[tokio::test]
async fn chunk_by_id() {
    let repo = new_repo().await;

    repo.chunk()
        .chunk_by_id(ChunkId(1145141919810))
        .await
        .unwrap_err();

    let pos = Position::new(114, 514);
    repo.fief().create("协会横幅", None).await.unwrap();
    let fief_id = repo.fief().id("协会横幅").await.unwrap();
    repo.chunk().create("左侧", fief_id, pos).await.unwrap();

    let id = repo.chunk().id(fief_id, "左侧").await.unwrap();
    let chunk = repo.chunk().chunk_by_id(id).await.unwrap();
    assert_eq!(chunk.fief_id, fief_id);
    assert_eq!(chunk.name, "左侧");
}

#[tokio::test]
async fn chunk_by_name() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let fief_id = repo.fief().id("协会横幅").await.unwrap();

    repo.chunk()
        .chunk_by_name(fief_id, "左侧")
        .await
        .unwrap_err();

    let pos = Position::new(114, 514);
    repo.chunk().create("左侧", fief_id, pos).await.unwrap();

    let id = repo.chunk().id(fief_id, "左侧").await.unwrap();
    let chunk = repo.chunk().chunk_by_name(fief_id, "左侧").await.unwrap();
    assert_eq!(chunk.id, id);
}

#[tokio::test]
async fn fief_id() {
    let repo = new_repo().await;
}

#[tokio::test]
async fn name() {
    let repo = new_repo().await;
}

#[tokio::test]
async fn id() {
    let repo = new_repo().await;
}

#[tokio::test]
async fn position() {
    let repo = new_repo().await;
}

#[tokio::test]
async fn ref_img() {
    let repo = new_repo().await;
}

#[tokio::test]
async fn mask_img() {
    let repo = new_repo().await;
}

#[tokio::test]
async fn diff_img() {
    let repo = new_repo().await;
}

#[tokio::test]
async fn diff_count() {
    let repo = new_repo().await;
}

// - related
// *PASS*

// [U]pdate
// - self or fields
#[tokio::test]
async fn update_ref_img() {
    let repo = new_repo().await;
}

#[tokio::test]
async fn update_mask_img() {
    let repo = new_repo().await;
}

#[tokio::test]
async fn update_diff() {
    let repo = new_repo().await;
}

#[tokio::test]
async fn set_position() {
    let repo = new_repo().await;
}

#[tokio::test]
async fn set_name() {
    let repo = new_repo().await;
}

// - related
// *PASS*

// [D]elete
#[tokio::test]
async fn remove_by_id() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let pos1 = Position::new(114, 514);
    let pos2 = Position::new(114, 515);
    let fief_id = repo.fief().id("协会横幅").await.unwrap();
    repo.chunk().create("左侧", fief_id, pos1).await.unwrap();
    repo.chunk().create("右侧", fief_id, pos2).await.unwrap();
    let id1 = repo.chunk().id(fief_id, "左侧").await.unwrap();
    let id2 = repo.chunk().id(fief_id, "右侧").await.unwrap();

    let result = repo.chunk().remove_by_id(id1).await.unwrap();
    assert!(result);

    let expect = HashSet::<ChunkId>::from_iter([id2]);
    let actual = repo.fief().chunks(fief_id).await.unwrap();
    assert_eq!(expect, actual.into_iter().collect());

    let result = repo.chunk().remove_by_id(id1).await.unwrap();
    assert!(!result);
}

#[tokio::test]
async fn remove_by_name() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let pos1 = Position::new(114, 514);
    let pos2 = Position::new(114, 515);
    let fief_id = repo.fief().id("协会横幅").await.unwrap();
    repo.chunk().create("左侧", fief_id, pos1).await.unwrap();
    repo.chunk().create("右侧", fief_id, pos2).await.unwrap();
    let id = repo.chunk().id(fief_id, "右侧").await.unwrap();

    let result = repo.chunk().remove_by_name(fief_id, "左侧").await.unwrap();
    assert!(result);

    let expect = HashSet::<ChunkId>::from_iter([id]);
    let actual = repo.fief().chunks(fief_id).await.unwrap();
    assert_eq!(expect, actual.into_iter().collect());

    let result = repo.chunk().remove_by_name(fief_id, "左侧").await.unwrap();
    assert!(!result);
}

#[tokio::test]
async fn remove_all_by_fief() {
    let repo = new_repo().await;
    repo.fief().create("协会横幅", None).await.unwrap();
    let pos1 = Position::new(114, 514);
    let pos2 = Position::new(114, 515);
    let fief_id = repo.fief().id("协会横幅").await.unwrap();
    repo.chunk().create("左侧", fief_id, pos1).await.unwrap();
    repo.chunk().create("右侧", fief_id, pos2).await.unwrap();

    let errid = FiefId(1145141919810);
    let result = repo.chunk().remove_all_by_fief(errid).await.unwrap();
    assert!(!result);
    assert_eq!(repo.fief().chunks(fief_id).await.unwrap().len(), 2);

    let result = repo.chunk().remove_all_by_fief(fief_id).await.unwrap();
    assert!(result);
    assert!(repo.fief().chunks(fief_id).await.unwrap().is_empty());

    let result = repo.chunk().remove_all_by_fief(fief_id).await.unwrap();
    assert!(!result);
}
