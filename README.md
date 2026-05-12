
# kgen

_A sophisticated keyboard layout generator and management system._

---

`kgen` (Key Gen) bridges the gap between human-readable layout design and firmware-specific configurations. It allows you to define your keyboard layers in intuitive text formats and compiles them into ready to use firmware code.

## Core Capabilities

*   **Declarative Configuration**
    Manage your entire keyboard through a central `config.toml` and clean text-based layer files.
*   **Visual Consistency**
    Automatic formatting ensures your layer files remain readable and aligned with your physical layout.
*   **Cross-Firmware Support**
    Generate configurations for QMK (fully supported) and ZMK (in development).

## Installation

Building from source requires the Rust toolchain:

```bash
git clone https://github.com/vincbro/kgen.git
cd kgen
cargo install --path .
```


## Technical Workflow

`kgen` operates on a structured project directory:

1.  **`config.toml`**
    The source of truth for your physical layout and layer definitions.
2.  **Layer Files (`.txt`)**
    Human-readable representations of individual keyboard layers.

The tool parses these inputs to generate the final keymap logic for your target firmware.


## Configuration (`config.toml`)

The `config.toml` file defines the physical architecture of your keyboard and identifies the layers to be compiled.

### Layout Definition

The `layout` field uses a visual grid to define key positions. Each `#` represents a physical key, while spaces represent gaps. This grid is used by `kgen format` to generate and align your layer files.

```toml
[config]
layout = [
	"######     ######",
	"######     ######",
	"######     ######",
	"######## ########",
	"   ##### #####   ",
	"####         ####",
]
```

### Layer Management

The `layers` field lists the names of your layer files (without the `.txt` extension). `kgen` expects to find a corresponding file for each entry (e.g., `base.txt`, `nav.txt`).

```toml
layers = ["base", "nav", "sym"]
```


## Command Reference

### `init`

Initializes a new project structure with a default configuration.

```bash
kgen init --path ./my-keyboard
```

### `format`

Standardizes the visual layout of your layer files. If a file is missing, `kgen` generates a template based on your configuration.

```bash
kgen format --path ./my-keyboard
```

### `build`

Compiles your project into firmware-specific source code.

```bash
kgen build --path ./my-keyboard --manufacturer qmk --output ./keymap.c
```

#### Parameters

| Option | Shorthand | Description | Default |
| :--- | :--- | :--- | :--- |
| `--path` | `-p` | Path to the keyboard project directory. | Required |
| `--manufacturer` | `-m` | Target firmware (e.g., `qmk`, `zmk`). | `qmk` |
| `--output` | `-o` | Path to save the generated output. | `stdout` |


## Ecosystem Support

### QMK Firmware

`kgen` provides first-class support for QMK, translating text layers into robust C code compatible with standard QMK builds.

### ZMK Firmware

Support for ZMK is currently under active development.


### Contribution

To add support for a new manufacturer, please refer to the implementation patterns in `src/parser/qmk.rs` and extend the `Manufacturers` enum. Pull requests are welcome.
