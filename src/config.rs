use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    sync::{LazyLock, RwLock, RwLockReadGuard},
};

use toml_edit::DocumentMut;
use tracing::{error, info};

const CONFIG_PATH: &'static str = "./cfg.toml";

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Config {}

static CONFIG_DOC: LazyLock<RwLock<DocumentMut>> = LazyLock::new(|| {
    info!("loading configuration...");
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

static CONFIG: LazyLock<RwLock<Config>> = LazyLock::new(|| {
    let doc = CONFIG_DOC
        .read()
        .unwrap_or_else(|e| {
            error!("failed to parse configuration: {e}");
            panic!();
        })
        .clone();
    let cfg = toml_edit::de::from_document(doc).unwrap_or_else(|e| {
        error!("failed to parse configuration: {e}");
        panic!();
    });
    RwLock::new(cfg)
});

pub fn init_cfg() {
    let _c = CONFIG.read().unwrap();
}

pub fn cfg<'a>() -> RwLockReadGuard<'a, Config> {
    CONFIG.read().unwrap()
}

pub fn save_cfg_with(f: impl FnOnce(&mut Config)) {
    {
        let mut cfg = CONFIG.write().unwrap();
        f(&mut cfg);
    }
    save_cfg();
}

fn save_cfg() {
    let cfg = CONFIG.read().unwrap();
    let mut doc = CONFIG_DOC.write().unwrap();

    let mut updater =
        crate::utils::cfg::DocUpdater::new(toml_edit::ser::to_document(&*cfg).unwrap());
    updater.update(&mut doc);

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
