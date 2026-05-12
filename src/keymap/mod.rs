use std::fmt::Display;

#[derive(Debug, Clone, Default)]
pub struct Keymap {
    pub matrix: Vec<Vec<Option<String>>>,
}

impl Keymap {
    pub fn new(layout: &[String]) -> Self {
        let mut matrix = Vec::with_capacity(layout.len());
        for row in layout {
            let mut matrix_row = Vec::with_capacity(row.len());
            for key in row.chars() {
                match key {
                    '#' => matrix_row.push(Some("KC_NO".to_string())),
                    _ => matrix_row.push(None),
                }
            }
            matrix.push(matrix_row);
        }
        Self { matrix }
    }

    pub fn set_keymap(&mut self, keymap: &str) -> Result<(), crate::Error> {
        let mut matrix = self.matrix.iter_mut().peekable();
        for (line_idx, line) in keymap.lines().enumerate() {
            let keys: Vec<&str> = line.split_whitespace().collect();
            if keys.is_empty() {
                continue;
            }
            let row = matrix.next().ok_or(crate::Error::TooManyRows)?;
            if row.len() < keys.len() {
                return Err(crate::Error::TooManyKeys(line_idx));
            }
            let mut available_slots = row.iter_mut().filter_map(|slot| slot.as_mut());
            for key_str in keys {
                if let Some(slot) = available_slots.next() {
                    *slot = key_str.to_string();
                } else {
                    return Err(crate::Error::TooManyKeys(line_idx));
                }
            }
        }
        Ok(())
    }
}

impl Display for Keymap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut largest_keys = Vec::new();
        for row in self.matrix.iter() {
            if row.len() > largest_keys.len() {
                largest_keys.resize(row.len(), 0);
            }
            for (idx, key) in row.iter().enumerate() {
                let size = if let Some(key) = key { key.len() } else { 0 };
                if largest_keys[idx] < size {
                    largest_keys[idx] = size;
                }
            }
        }

        for row in self.matrix.iter() {
            for (idx, key) in row.iter().enumerate() {
                let pad = (largest_keys[idx] + 2).max(4);
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
