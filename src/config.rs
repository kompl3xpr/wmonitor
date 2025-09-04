use serde::{Deserialize, Serialize};
use std::{
    io::{Read, Write},
    sync::{LazyLock, RwLock, RwLockReadGuard},
};
use tap::prelude::*;
use toml_edit::{Datetime, DocumentMut, Formatted, Value, visit_mut::VisitMut};
use tracing::{error, info};

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
    pub image_cache_min: usize,
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
    pub event_filter: Vec<String>,
    pub discord_channel: String,
}

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct VisualizationConfig {
    pub diff_img_opacity_pct: usize,
    pub normal_color: usize,
    pub abnormal_color: usize,
    pub unmasked_color: usize,
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

pub fn init_cfg() {
    let c = CONFIG.read().unwrap();
    info!("CONFIG = {c:#?}");
}

pub fn cfg<'a>() -> RwLockReadGuard<'a, Config> {
    CONFIG.read().unwrap()
}

// pub fn save_cfg_with(f: impl FnOnce(&mut Config)) {
//     {
//         let mut cfg = CONFIG.write().unwrap();
//         f(&mut cfg);
//     }
//     save_cfg();
// }

// fn save_cfg() {
//     let cfg = CONFIG.read().unwrap();
//     let mut doc = CONFIG_DOC.write().unwrap();

//     let mut updater = DocUpdater::new(toml_edit::ser::to_document(&*cfg).unwrap());
//     updater.update(&mut doc);

//     let file = std::fs::File::options()
//         .write(true)
//         .truncate(true)
//         .open(CONFIG_PATH);
//     let mut file = file.unwrap_or_else(|e| {
//         error!("failed to save configuration file: {e}");
//         panic!();
//     });

//     if let Err(e) = file.write_all(doc.to_string().as_bytes()) {
//         error!("failed to save configuration file: {e}");
//         panic!();
//     }
// }

// pub struct DocUpdater {
//     to_be_merged: DocumentMut,
//     path: Vec<String>,
// }

// impl DocUpdater {
//     pub fn new(to_be_merged: DocumentMut) -> Self {
//         let path = vec![];
//         Self { to_be_merged, path }
//     }

//     pub fn update(&mut self, doc: &mut DocumentMut) {
//         self.visit_item_mut(doc.as_item_mut());
//     }

//     fn current_item(&self) -> &toml_edit::Item {
//         let mut item = self.to_be_merged.as_item();
//         for index in &self.path {
//             item = &item[index];
//         }
//         item
//     }
// }

// impl toml_edit::visit_mut::VisitMut for DocUpdater {
//     fn visit_boolean_mut(&mut self, node: &mut Formatted<bool>) {
//         let Some(Value::Boolean(formatted)) = self.current_item().as_value().cloned() else {
//             unreachable!()
//         };
//         // preserve comments
//         *node = formatted.tap_mut(|f| f.decor_mut().clone_from(node.decor()));
//     }

//     fn visit_datetime_mut(&mut self, node: &mut Formatted<Datetime>) {
//         let Some(Value::Datetime(formatted)) = self.current_item().as_value().cloned() else {
//             unreachable!()
//         };
//         *node = formatted.tap_mut(|f| f.decor_mut().clone_from(node.decor()));
//     }

//     fn visit_float_mut(&mut self, node: &mut Formatted<f64>) {
//         let Some(Value::Float(formatted)) = self.current_item().as_value().cloned() else {
//             unreachable!()
//         };
//         *node = formatted.tap_mut(|f| f.decor_mut().clone_from(node.decor()));
//     }

//     fn visit_integer_mut(&mut self, node: &mut Formatted<i64>) {
//         let Some(Value::Integer(formatted)) = self.current_item().as_value().cloned() else {
//             unreachable!()
//         };
//         *node = formatted.tap_mut(|f| f.decor_mut().clone_from(node.decor()));
//     }

//     fn visit_string_mut(&mut self, node: &mut Formatted<String>) {
//         let Some(Value::String(formatted)) = self.current_item().as_value().cloned() else {
//             unreachable!()
//         };
//         *node = formatted.tap_mut(|f| f.decor_mut().clone_from(node.decor()));
//     }

//     fn visit_table_like_kv_mut(&mut self, key: toml_edit::KeyMut<'_>, node: &mut toml_edit::Item) {
//         self.path.push(key.to_string());
//         toml_edit::visit_mut::visit_table_like_kv_mut(self, key, node);
//         self.path.pop();
//     }

//     fn visit_array_mut(&mut self, node: &mut toml_edit::Array) {
//         let decor = node.iter().next().map(|v| v.decor().clone());

//         let Some(array) = self.current_item().as_array().cloned() else {
//             unreachable!()
//         };
//         node.clear();
//         node.extend(array.into_iter().map(|mut value| {
//             decor.as_ref().map(|d| value.decor_mut().clone_from(d));
//             value
//         }));
//     }

//     fn visit_array_of_tables_mut(&mut self, node: &mut toml_edit::ArrayOfTables) {
//         let decor = node.iter().next().map(|t| t.decor().clone());
//         node.clear();
//         if let Some(array) = self.current_item().as_array() {
//             node.extend(array.iter().map(|v| {
//                 v.as_inline_table()
//                     .unwrap()
//                     .clone()
//                     .into_table()
//                     .tap_mut(|t| {
//                         decor.as_ref().map(|d| t.decor_mut().clone_from(d));
//                     })
//             }));
//         } else {
//             unreachable!();
//         }
//     }
// }
