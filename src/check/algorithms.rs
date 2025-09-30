use std::ops::Add;

use anyhow::Result;
use image::{GenericImageView, GrayImage, Pixel, RgbaImage, imageops::FilterType};
use rayon::prelude::*;

use crate::{
    cfg,
    core::{Position, WPLACE_CHUNK_HEIGHT, WPLACE_CHUNK_WIDTH},
};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiffRecord {
    pub diffs: Vec<Diff>,
    pub diff_img: image::GrayImage,
    scope_rect: ScopeRect,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Diff {
    pub pos: Position,
    pub curr_px: [u8; 4],
    pub ref_px: [u8; 4],
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct ScopeRect {
    left_top: Position,
    right_bottom: Position,
}

impl Default for ScopeRect {
    fn default() -> Self {
        ScopeRect {
            left_top: Position::new(0, 0),
            right_bottom: Position::new(1000, 1000),
        }
    }
}

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

    let [x, y, w, h] = get_sub_image_params(rec.scope_rect);
    let out = out.view(x, y, w, h).to_image();
    let factor = WPLACE_CHUNK_WIDTH as u32 / w;
    Ok(image::imageops::resize(
        &out,
        w * factor,
        h * factor,
        FilterType::Nearest,
    ))
}

fn get_sub_image_params(scp: ScopeRect) -> [u32; 4] {
    let (min_w, min_h, margin_x, margin_y) = (
        cfg().visualization.minimum_width,
        cfg().visualization.minimum_height,
        cfg().visualization.horizontal_margin,
        cfg().visualization.vertical_margin,
    );
    let (scp_w, scp_h) = (
        WPLACE_CHUNK_WIDTH.min(scp.right_bottom.x - scp.left_top.x + 1 + margin_x * 2),
        WPLACE_CHUNK_HEIGHT.min(scp.right_bottom.y - scp.left_top.y + 1 + margin_y * 2),
    );
    let (w, h) = (min_w.max(scp_w), min_h.max(scp_h));
    let (x_mov, y_mov) = (
        (margin_x + min_w.saturating_sub(scp_w) / 2) as isize,
        (margin_y + min_h.saturating_sub(scp_h) / 2) as isize,
    );
    let (x1, y1, x2, y2) = (
        scp.left_top.x as isize - x_mov,
        scp.left_top.y as isize - y_mov,
        scp.right_bottom.x as isize - x_mov,
        scp.right_bottom.y as isize - y_mov,
    );
    let (cw, ch) = (WPLACE_CHUNK_WIDTH as isize, WPLACE_CHUNK_HEIGHT as isize);

    let x = if x2 >= cw { x1 - x2 + cw - 1 } else { x1 };
    let y = if y2 >= ch { y1 - y2 + ch - 1 } else { y1 };
    [x.max(0) as u32, y.max(0) as u32, w as u32, h as u32]
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
