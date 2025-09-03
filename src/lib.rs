pub mod config;
pub mod utils;
pub use config::{cfg, init_cfg, save_cfg_with};

pub mod entities;

pub mod repos;
pub use repos::Repositories;

pub mod domains {
    pub use crate::repos::domains::*;
}

pub mod app;
pub mod bot;
pub mod checker;
pub mod commands;