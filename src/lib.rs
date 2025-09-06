

pub mod entities;

pub mod repos;
pub use repos::Repositories;

pub mod domains {
    pub use crate::repos::domains::*;
}

pub mod app;
pub mod bot;
pub mod check;
pub mod commands;
pub mod core;
pub use core::config::{cfg, init_cfg, save_cfg_with};
pub use core::net;
