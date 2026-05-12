use std::{fs::File, io::Read, path::Path};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Document {
    pub config: Config,
}

impl Document {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    pub fn from_path<P>(path: P) -> Result<Self, crate::Error>
    where
        P: AsRef<Path>,
    {
        let mut buf = String::new();
        File::open(path)?.read_to_string(&mut buf)?;
        let conf = toml::from_str::<Self>(&buf)?;
        Ok(conf)
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct Config {
    pub layout: Vec<String>,
    pub layers: Vec<String>,
}

impl Config {
    #[must_use]
    pub fn new(layout: Vec<String>, layers: Vec<String>) -> Self {
        Self { layout, layers }
    }
}
