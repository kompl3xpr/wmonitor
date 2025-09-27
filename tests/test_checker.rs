use image::{DynamicImage, ImageFormat, ImageReader};
use wmonitor::check::algorithms::{find_diffs, gen_visual_result};

const REF_IMG_PATH: &str = "./tests_data/ref.png";
const MASK_IMG_PATH: &str = "./tests_data/mask.png";
const CURR_IMG_PATH: &str = "./tests_data/curr.png";
const DIFF_IMG_PATH: &str = "./tests_data/diff.png";
const RESULT_IMG_PATH: &str = "./tests_data/result.png";

fn load(path: &str) -> anyhow::Result<DynamicImage> {
    let mut reader = ImageReader::open(path)?;
    reader.set_format(ImageFormat::Png);
    let result = reader.decode()?;
    Ok(result)
}

#[test]
fn algorithms() {
    let ref_ = load(REF_IMG_PATH).unwrap().to_rgba8();
    let mask = load(MASK_IMG_PATH).unwrap().to_luma8();
    let curr = load(CURR_IMG_PATH).unwrap().to_rgba8();

    let expect_diff_img = load(DIFF_IMG_PATH).unwrap().to_luma8();
    let rec = find_diffs(&ref_, &mask, &curr).unwrap();
    assert_eq!(rec.diff_img, expect_diff_img);
    assert_eq!(rec.diffs.len(), 64);

    let result = gen_visual_result(&ref_, &mask, &curr, &rec).unwrap();
    result.save(RESULT_IMG_PATH).unwrap();
}
