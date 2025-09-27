use image::{GrayImage, RgbaImage};
use wmonitor::check::algorithms::find_diffs;

#[test]
fn algorithms() {
    let ref_ =
        RgbaImage::from_fn(2, 2, |x, _| [x as u8, 0x01, 0x02, 0xff].into());
    let curr =
        RgbaImage::from_fn(2, 2, |_, y| [y as u8, 0x01, 0x02, 0xff].into());
    let mask = GrayImage::from_vec(2, 2, vec![0xff, 0xff, 0x00, 0x00]).unwrap();

    let expect_diff_img =
        GrayImage::from_vec(2, 2, vec![0x00, 0xff, 0x00, 0x00]).unwrap();

    let rec = find_diffs(&ref_, &mask, &curr).unwrap();
    assert_eq!(rec.diff_img, expect_diff_img);
    assert_eq!(rec.diffs.len(), 1);
}
