use std::{fs, path::Path};

use crate::{document::Document, keymap::Keymap};

pub struct Keyboard {
    pub layers: Vec<Layer>,
}

pub struct Layer {
    pub name: String,
    pub keymap: Keymap,
}

impl Keyboard {
    pub fn from_path<P>(path: P) -> Result<Self, crate::Error>
    where
        P: AsRef<Path>,
    {
        let conf_path = path.as_ref().join("config.toml");
        let doc = Document::from_path(conf_path)?;
        let mut layers = Vec::new();
        for layer in doc.config.layers {
            let mut keymap = Keymap::new(&doc.config.layout);
            let mut keymap_path = path.as_ref().join(&layer);
            keymap_path.set_extension("txt");
            if fs::exists(&keymap_path)? {
                let buf = fs::read_to_string(&keymap_path)?;
                keymap.set_keymap(&buf);
            }
            layers.push(Layer {
                name: layer,
                keymap,
            });
        }
        Ok(Self { layers })
    }
}
