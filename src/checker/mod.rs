use anyhow::Result;
use image::{GrayImage, Pixel, RgbaImage};
use rayon::prelude::*;

use std::ops::Add;

use crate::{cfg, core::Position};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DiffRecord {
    ref_px: [u8; 4],
    curr_px: [u8; 4],
    pos: Position,
}

fn find_diffs(
    ref_: &RgbaImage,
    mask: &GrayImage,
    curr: &RgbaImage,
) -> Result<(GrayImage, Vec<DiffRecord>)> {
    let (r, m, c) = (ref_, mask, curr);

    // check size
    let size @ (w, h) = r.dimensions();
    if m.dimensions() != size || c.dimensions() != size {
        return Err(anyhow::anyhow!("failed to find diffs: image size error"));
    }

    let mut out = GrayImage::new(w, h);
    let recs = out
        .par_enumerate_pixels_mut()
        .zip(r.par_pixels())
        .zip(m.par_pixels())
        .zip(c.par_pixels())
        .map(|((((x, y, o), r), m), c)| {
            *o = image::Luma([if m.0[0] == 0xFF && r != c { 0xFF } else { 0 }]);
            DiffRecord {
                pos: [x as usize, y as usize].into(),
                ref_px: r.0,
                curr_px: c.0,
            }
        })
        .collect::<Vec<_>>();

    Ok((out, recs))
}

fn gen_visual_result(
    ref_: &RgbaImage,
    mask: &GrayImage,
    curr: &RgbaImage,
    diff: &GrayImage,
) -> Result<RgbaImage> {
    let (r, m, c, d) = (ref_, mask, curr, diff);

    let size @ (w, h) = r.dimensions();
    if m.dimensions() != size || c.dimensions() != size || d.dimensions() != size {
        return Err(anyhow::anyhow!("failed to find diffs: image size error"));
    }

    let mut out = RgbaImage::new(w, h);
    let abnormal = rgb_usize_to_rgba(cfg().visualization.abnormal_color);
    let normal = rgb_usize_to_rgba(cfg().visualization.normal_color);
    let unmasked = rgb_usize_to_rgba(cfg().visualization.unmasked_color);
    let pct = cfg().visualization.diff_img_opacity_pct;
    let mix = |mut base: image::Rgba<u8>, appended: image::Rgba<u8>| -> image::Rgba<u8> {
        if base.0[3] < 0xFF {
            base = image::Rgba([0x00, 0x00, 0x00, 0xFF]);
        }
        let base = base.map(|b| ((b as usize) * (100 - pct) / 100) as u8);
        let appended = appended.map(|a| ((a as usize) * pct / 100) as u8);
        base.map2(&appended, u8::add)
    };

    out.par_pixels_mut()
        .zip(r.par_pixels())
        .zip(m.par_pixels())
        .zip(c.par_pixels())
        .zip(d.par_pixels())
        .for_each(|((((o, r), m), _), d)| {
            let a = image::Rgba(match (m.0[0] == 0xFF, d.0[0] == 0xFF) {
                (_, true) => abnormal,
                (true, false) => normal,
                _ => unmasked,
            });
            *o = mix(*r, a);
        });

    Ok(out)
}

fn rgb_usize_to_rgba(color: usize) -> [u8; 4] {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    let a = 255; // 默认 alpha 值

    [r, g, b, a]
}
