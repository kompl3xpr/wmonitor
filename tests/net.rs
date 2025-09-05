use wmonitor::net;

// use image::ImageFormat;
// const CURRENT_IMG_PATH: &'static str = "tests_data/current.png";

#[tokio::test]
async fn fetch_current_image() {
    let (cached, _) = net::fetch_current_image([1687, 888]).await.unwrap();
    assert!(!cached.0);

    let (cached, _) = net::fetch_current_image([1687, 888]).await.unwrap();
    assert!(cached.0);
}
