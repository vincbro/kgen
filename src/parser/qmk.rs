use crate::keyboard::Keyboard;

impl Keyboard {
    pub fn parse_qmk(&self) -> Result<String, crate::Error> {
        let mut layers = String::new();
        for layer in &self.layers {
            layers.push_str(&format!("    {},\n", layer.name.to_uppercase()));
        }
        if layers.ends_with(",\n") {
            layers.truncate(layers.len() - 2);
            layers.push('\n');
        }

        let mut keymaps = String::new();
        for layer in &self.layers {
            let mut keys = Vec::new();
            for row in &layer.keymap.matrix {
                let mut row_keys = Vec::new();
                for k in row.iter().flatten() {
                    row_keys.push(k.as_str());
                }
                if !row_keys.is_empty() {
                    keys.push(format!("     {}", row_keys.join(", ")));
                }
            }

            keymaps.push_str(&format!(
                "    [{}] = LAYOUT_elora_hlc(\n{}\n    ),\n",
                layer.name.to_uppercase(),
                keys.join(",\n")
            ));
        }
        if keymaps.ends_with(",\n") {
            keymaps.truncate(keymaps.len() - 2);
            keymaps.push('\n');
        }

        Ok(format!(
            r#"// GENERATED FROM KGEN 

#include QMK_KEYBOARD_H

enum layers {{
{layers}}};

// clang-format off
const uint16_t PROGMEM keymaps[][MATRIX_ROWS][MATRIX_COLS] = {{
{keymaps}}};
"#
        ))
    }
}
