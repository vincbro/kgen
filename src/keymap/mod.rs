pub mod key;

use std::{fmt::Display, str::FromStr};

use key::Key;

#[derive(Debug, Clone, Default)]
pub struct Keymap {
    pub matrix: Vec<Vec<Option<Key>>>,
}

impl Keymap {
    pub fn new(layout: &[String]) -> Self {
        let mut matrix: Vec<Vec<Option<Key>>> = Vec::with_capacity(layout.len());
        for row in layout {
            let mut matrix_row = Vec::with_capacity(row.len());
            for key in row.chars() {
                match key {
                    '#' => matrix_row.push(Some(Key::No)),
                    _ => matrix_row.push(None),
                }
            }
            matrix.push(matrix_row);
        }
        Self { matrix }
    }

    pub fn set_keymap(&mut self, keymap: &str) {
        let mut matrix = self.matrix.iter_mut().peekable();
        for (line_idx, line) in keymap.lines().enumerate() {
            let keys: Vec<&str> = line.split_whitespace().collect();
            if keys.is_empty() {
                continue;
            }
            let row = matrix
                .next()
                .unwrap_or_else(|| panic!("To few keys on line {line_idx}"));
            if row.len() < keys.len() {
                panic!("To many keys on line {line_idx}");
            }
            let mut available_slots = row.iter_mut().filter_map(|slot| slot.as_mut());
            for key_str in keys {
                if let Some(slot) = available_slots.next() {
                    *slot = Key::from_str(key_str).expect("Invalid keycode");
                } else {
                    panic!("Too many keys on line {line_idx}");
                }
            }
        }
    }
}

impl Display for Keymap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let largest_key = self
            .matrix
            .iter()
            .flatten()
            .flatten()
            .reduce(|a, b| {
                if a.as_str().len() >= b.as_str().len() {
                    a
                } else {
                    b
                }
            })
            .map(|v| v.as_str().len())
            .unwrap_or(0);

        let pad = largest_key + 2;

        for row in self.matrix.iter() {
            for key in row.iter() {
                if let Some(key) = key {
                    write!(f, "{:^pad$}", key.as_str())?;
                } else {
                    write!(f, "{:>pad$}", " ")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}
