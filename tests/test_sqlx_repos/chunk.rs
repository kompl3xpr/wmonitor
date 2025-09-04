use super::new_repo;
use std::collections::HashSet;
use wmonitor::{
    domains::{ChunkId, FiefId, Position},
    utils::img::ImagePng,
};

// [C]reate
#[tokio::test]
async fn create() {
    let repo = new_repo().await;
    let pos = Position::new(114, 514);

    let id = repo.chunk().create("左侧", FiefId(1), pos).await.unwrap();
    assert!(id.is_none());

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let id = repo.chunk().create("左侧", fief_id, pos).await.unwrap();
    assert!(id.is_some());

    let id = repo.chunk().create("左侧", fief_id, pos).await.unwrap();
    assert!(id.is_none());
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
    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();

    let chunk = repo.chunk().chunk_by_id(id).await.unwrap();
    assert_eq!(chunk.fief_id, fief_id);
    assert_eq!(chunk.name, "左侧");
}

#[tokio::test]
async fn chunk_by_name() {
    let repo = new_repo().await;
    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();

    repo.chunk()
        .chunk_by_name(fief_id, "左侧")
        .await
        .unwrap_err();

    let pos = Position::new(114, 514);
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();

    repo.chunk()
        .chunk_by_name(FiefId(114514), "左侧")
        .await
        .unwrap_err();
    let chunk = repo.chunk().chunk_by_name(fief_id, "左侧").await.unwrap();
    assert_eq!(chunk.id, id);
}

#[tokio::test]
async fn fief_id() {
    let repo = new_repo().await;

    repo.chunk().fief_id(ChunkId(114514)).await.unwrap_err();

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [114, 514].into();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    let expect = fief_id;
    let actual = repo.chunk().fief_id(id).await.unwrap();
    assert_eq!(expect, actual);
}

#[tokio::test]
async fn name() {
    let repo = new_repo().await;

    repo.chunk().name(ChunkId(114514)).await.unwrap_err();

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [114, 514].into();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    let name = repo.chunk().name(id).await.unwrap();
    assert_eq!(name, "左侧");
}

#[tokio::test]
async fn id() {
    let repo = new_repo().await;
    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();

    repo.chunk().id(fief_id, "左侧").await.unwrap_err();

    let pos = Position::new(114, 514);
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();

    repo.chunk().id(FiefId(114514), "左侧").await.unwrap_err();
    let expect = id;
    let actual = repo.chunk().id(fief_id, "左侧").await.unwrap();
    assert_eq!(expect, actual);
}

#[tokio::test]
async fn position() {
    let repo = new_repo().await;

    repo.chunk().position(ChunkId(114514)).await.unwrap_err();

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [114, 514].into();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    let expect = pos;
    let actual = repo.chunk().position(id).await.unwrap();
    assert_eq!(expect, actual);
}

#[tokio::test]
async fn ref_img() {
    let repo = new_repo().await;

    repo.chunk().ref_img(ChunkId(114514)).await.unwrap_err();

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [114, 514].into();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    let actual = repo.chunk().ref_img(id).await.unwrap();
    assert!(actual.is_none());
}

#[tokio::test]
async fn mask_img() {
    let repo = new_repo().await;

    repo.chunk().mask_img(ChunkId(114514)).await.unwrap_err();

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [114, 514].into();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    let actual = repo.chunk().mask_img(id).await.unwrap();
    assert!(actual.is_none());
}

#[tokio::test]
async fn diff_img() {
    let repo = new_repo().await;

    repo.chunk().diff_img(ChunkId(114514)).await.unwrap_err();

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [114, 514].into();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    let actual = repo.chunk().diff_img(id).await.unwrap();
    assert!(actual.is_none());
}

#[tokio::test]
async fn diff_count() {
    let repo = new_repo().await;

    repo.chunk().diff_count(ChunkId(114514)).await.unwrap_err();

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [114, 514].into();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(repo.chunk().diff_count(id).await.unwrap(), 0);
    repo.chunk().update_diff(id, None, 99).await.unwrap();
    assert_eq!(repo.chunk().diff_count(id).await.unwrap(), 99);
}

// - related
// *PASS*

// [U]pdate
// - self or fields
#[tokio::test]
async fn update_ref_img() {
    let repo = new_repo().await;

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [0, 0].into();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(repo.chunk().ref_img(id).await.unwrap(), None);

    let img = Some(ImagePng::new(vec![0xCA, 0xFE, 0xBA, 0xBE]));
    repo.chunk()
        .update_ref_img(id, img.as_ref().cloned())
        .await
        .unwrap();
    assert_eq!(repo.chunk().ref_img(id).await.unwrap(), img);
}

#[tokio::test]
async fn update_mask_img() {
    let repo = new_repo().await;

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [0, 0].into();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(repo.chunk().mask_img(id).await.unwrap(), None);

    let img = Some(ImagePng::new(vec![0xCA, 0xFE, 0xBA, 0xBE]));
    repo.chunk()
        .update_mask_img(id, img.as_ref().cloned())
        .await
        .unwrap();
    assert_eq!(repo.chunk().mask_img(id).await.unwrap(), img);
}

#[tokio::test]
async fn update_diff() {
    let repo = new_repo().await;

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [0, 0].into();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(repo.chunk().diff_img(id).await.unwrap(), None);
    assert_eq!(repo.chunk().diff_count(id).await.unwrap(), 0);

    let img = Some(ImagePng::new(vec![0xCA, 0xFE, 0xBA, 0xBE]));
    repo.chunk()
        .update_diff(id, img.as_ref().cloned(), 114514)
        .await
        .unwrap();
    assert_eq!(repo.chunk().diff_img(id).await.unwrap(), img);
    assert_eq!(repo.chunk().diff_count(id).await.unwrap(), 114514);
}

#[tokio::test]
async fn set_position() {
    let repo = new_repo().await;

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [0, 0].into();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(repo.chunk().position(id).await.unwrap(), [0, 0].into());

    repo.chunk().set_position(id, [1, 1].into()).await.unwrap();
    assert_eq!(repo.chunk().position(id).await.unwrap(), [1, 1].into());
}

#[tokio::test]
async fn set_name() {
    let repo = new_repo().await;

    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos = [0, 0].into();
    repo.chunk().create("右侧", fief_id, pos).await.unwrap();
    let id = repo
        .chunk()
        .create("左侧", fief_id, pos)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(repo.chunk().name(id).await.unwrap(), "左侧");

    repo.chunk().set_name(id, "左上").await.unwrap();
    assert_eq!(repo.chunk().name(id).await.unwrap(), "左上");

    repo.chunk().set_name(id, "右侧").await.unwrap_err();
}

// - related
// *PASS*

// [D]elete
#[tokio::test]
async fn remove_by_id() {
    let repo = new_repo().await;
    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos1 = Position::new(114, 514);
    let pos2 = Position::new(114, 515);
    let id1 = repo
        .chunk()
        .create("左侧", fief_id, pos1)
        .await
        .unwrap()
        .unwrap();
    let id2 = repo
        .chunk()
        .create("右侧", fief_id, pos2)
        .await
        .unwrap()
        .unwrap();

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
    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos1 = Position::new(114, 514);
    let pos2 = Position::new(114, 515);
    let _ = repo
        .chunk()
        .create("左侧", fief_id, pos1)
        .await
        .unwrap()
        .unwrap();
    let id = repo
        .chunk()
        .create("右侧", fief_id, pos2)
        .await
        .unwrap()
        .unwrap();

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
    let fief_id = repo.fief().create("协会横幅", None).await.unwrap().unwrap();
    let pos1 = Position::new(114, 514);
    let pos2 = Position::new(114, 515);
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
