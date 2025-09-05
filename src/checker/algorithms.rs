use anyhow::Result;
use image::{GenericImage, GenericImageView, GrayImage, Pixel, RgbaImage};
use rayon::prelude::*;

use super::{Diff, DiffRecord, ScopeRect};
use crate::{cfg, core::Position};
use std::ops::Add;

pub fn find_diffs(ref_: &RgbaImage, mask: &GrayImage, curr: &RgbaImage) -> Result<DiffRecord> {
    let (r, m, c) = (ref_, mask, curr);

    // check size
    let size @ (w, h) = r.dimensions();
    if m.dimensions() != size || c.dimensions() != size {
        return Err(anyhow::anyhow!("failed to find diffs: image size error"));
    }

    let mut out = GrayImage::new(w, h);
    let mut diffs = out
        .par_enumerate_pixels_mut()
        .zip(r.par_pixels())
        .zip(m.par_pixels())
        .zip(c.par_pixels())
        .filter_map(|((((x, y, o), r), m), c)| {
            if m.0[0] == 0xFF && r != c {
                *o = image::Luma([0xFF]);
                Some(Diff {
                    pos: [x as usize, y as usize].into(),
                    ref_px: r.0,
                    curr_px: c.0,
                })
            } else {
                *o = image::Luma([0]);
                None
            }
        })
        .collect::<Vec<_>>();

    diffs.sort();
    let scope_rect = calc_scope_rect(&diffs);
    Ok(DiffRecord {
        scope_rect,
        diffs,
        diff_img: out,
    })
}

pub fn gen_visual_result(
    ref_: &RgbaImage,
    mask: &GrayImage,
    curr: &RgbaImage,
    rec: &DiffRecord,
) -> Result<RgbaImage> {
    let (r, m, c, d) = (ref_, mask, curr, &rec.diff_img);

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

    let ScopeRect {
        left_top,
        right_bottom,
    } = rec.scope_rect;
    let (min_w, min_h, margin_x, margin_y) = (64, 64, 2, 2);
    let (scp_w, scp_h) = (
        1000.min(right_bottom.x - left_top.x + 1 + margin_x * 2),
        1000.min(right_bottom.y - left_top.y + 1 + margin_y * 2),
    );
    let (w, h, x, y) = (
        min_w.max(scp_w),
        min_h.max(scp_h),
        left_top.x,
        left_top.y,
    );

    Ok(out
        .view(x as u32, y as u32, w as u32, h as u32)
        .to_image())
}

fn calc_scope_rect(diffs: &[Diff]) -> ScopeRect {
    if diffs.is_empty() {
        return ScopeRect::default();
    }

    let min_x = diffs.iter().map(|x| x.pos.x).min().unwrap();
    let min_y = diffs.iter().map(|x| x.pos.y).min().unwrap();
    let max_x = diffs.iter().map(|x| x.pos.x).max().unwrap();
    let max_y = diffs.iter().map(|x| x.pos.y).max().unwrap();
    ScopeRect {
        left_top: Position::new(min_x, min_y),
        right_bottom: Position::new(max_x, max_y),
    }
}

fn rgb_usize_to_rgba(color: usize) -> [u8; 4] {
    let r = ((color >> 16) & 0xFF) as u8;
    let g = ((color >> 8) & 0xFF) as u8;
    let b = (color & 0xFF) as u8;
    let a = 255; // 默认 alpha 值

    [r, g, b, a]
}
