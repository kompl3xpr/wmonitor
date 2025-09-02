use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    sync::{LazyLock, RwLock, RwLockReadGuard, RwLockWriteGuard},
};

use toml_edit::DocumentMut;
use tracing::{error, info};

const CONFIG_PATH: &'static str = "./cfg.toml";

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Config {
    pub foo: String,
    pub bar: Bar,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Bar {
    pub a: usize,
    pub b: bool,
}

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

pub fn cfg<'a>() -> RwLockReadGuard<'a, Config> {
    CONFIG.read().unwrap()
}

pub struct CfgSaverGuard<'a>(RwLockWriteGuard<'a, Config>);

impl<'a> std::ops::Deref for CfgSaverGuard<'a> {
    type Target = Config;
    fn deref(&self) -> &Self::Target {
        &*self.0
    }
}

impl<'a> std::ops::DerefMut for CfgSaverGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.0
    }
}

impl<'a> Drop for CfgSaverGuard<'a> {
    fn drop(&mut self) {
        save_cfg(&self.0)
    }
}

pub fn cfg_mut<'a>() -> CfgSaverGuard<'a> {
    CfgSaverGuard(CONFIG.write().unwrap())
}

fn save_cfg(cfg: &Config) {
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
