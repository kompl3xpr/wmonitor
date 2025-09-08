use crate::core::log::error;
use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    sync::{LazyLock, RwLock, RwLockReadGuard},
};
use toml_edit::DocumentMut;

const CONFIG_PATH: &'static str = "./cfg.toml";

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Config {
    pub common: CommonConfig,
    pub network: NetworkConfig,
    pub check: CheckConfig,
    pub notification: NotificationConfig,
    pub visualization: VisualizationConfig,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CommonConfig {
    pub database_url: String,
    pub discord_token: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct NetworkConfig {
    pub image_cache_capacity: usize,
    pub image_cache_life_min: usize,
    pub sleep_between_requests_sec: usize,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct CheckConfig {
    pub minimum_interval_min: usize,
    pub default_interval_min: usize,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct NotificationConfig {
    pub enabled: bool,
    pub discord_channel: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub diff_img_opacity_pct: usize,
    pub normal_color: usize,
    pub abnormal_color: usize,
    pub unmasked_color: usize,
    pub minimum_width: usize,
    pub minimum_height: usize,
    pub horizontal_margin: usize,
    pub vertical_margin: usize,
}

static CONFIG_DOC: LazyLock<RwLock<DocumentMut>> = LazyLock::new(|| {
    let mut file = std::fs::File::open(CONFIG_PATH).unwrap_or_else(|e| {
        error!("failed to open configuration file: {e}");
        panic!();
    });

    let mut cgf_text = String::new();
    if let Err(e) = file.read_to_string(&mut cgf_text) {
        error!("failed to read configuration file: {e}");
        panic!();
    };

    let doc = cgf_text
        .parse::<toml_edit::DocumentMut>()
        .unwrap_or_else(|e| {
            error!("failed to parse configuration: {e}");
            panic!();
        });

    RwLock::new(doc)
});

fn cfg_from_doc(doc: DocumentMut) -> Config {
    toml_edit::de::from_document(doc).unwrap_or_else(|e| {
        error!("failed to parse configuration: {e}");
        panic!();
    })
}

static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| {
    let doc = CONFIG_DOC.read().unwrap().clone();
    RwLock::new(cfg_from_doc(doc))
});

pub fn init_cfg() {
    let _cfg = CONFIG.read().unwrap();
}

pub fn cfg<'a>() -> RwLockReadGuard<'a, Config> {
    CONFIG.read().unwrap()
}

pub fn save_cfg_with(f: impl FnOnce(&mut DocumentMut)) {
    {
        let mut doc = CONFIG_DOC.write().unwrap();
        f(&mut *doc);
    }

    let doc = CONFIG_DOC.read().unwrap();
    {
        let mut cfg = CONFIG.write().unwrap();
        *cfg = cfg_from_doc(doc.clone());
    }

    let file = std::fs::File::options()
        .write(true)
        .truncate(true)
        .open(CONFIG_PATH);
    let mut file = file.unwrap_or_else(|e| {
        error!("failed to save configuration file: {e}");
        panic!();
    });

    if let Err(e) = file.write_all(doc.to_string().as_bytes()) {
        error!("failed to save configuration file: {e}");
        panic!();
    }
}
