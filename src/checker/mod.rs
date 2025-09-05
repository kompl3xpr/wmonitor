pub mod algorithms;

use crate::core::Position;


#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct DiffRecord {
    pub diffs: Vec<Diff>,
    pub diff_img: image::GrayImage,
    pub scope_rect: ScopeRect,
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Diff {
    pub pos: Position,
    pub curr_px: [u8; 4],
    pub ref_px: [u8; 4],
}



#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ScopeRect {
    left_top: Position,
    right_bottom: Position,
}

impl Default for ScopeRect {
    fn default() -> Self {
        ScopeRect {
            left_top: Position::new(0, 0),
            right_bottom: Position::new(0, 0),
        }
    }
}

