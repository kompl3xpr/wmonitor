use serde::{Serialize, Deserialize};
use std::{io::Read, sync::LazyLock};

#[derive(Clone, PartialEq, Eq, Debug, Serialize, Deserialize)]
pub struct Config {

}

pub static CONFIG: LazyLock<Config> = LazyLock::new(|| {
    let mut file = std::fs::File::open("./cfg.toml").unwrap();
    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();
    toml::de::from_str(&buf).unwrap()
});