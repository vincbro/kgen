use std::io;

pub mod document;
pub mod keyboard;
pub mod keymap;
pub mod parser;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("io error: {0}")]
    Io(#[from] io::Error),
    #[error("serialization error: {0}")]
    Serialize(#[from] toml::ser::Error),
    #[error("deserialization error: {0}")]
    Deserialize(#[from] toml::de::Error),
    #[error("parse key error: {0}")]
    ParseKey(String),
}
