
# kgen (Key Gen)

`kgen` is a keyboard layout generator and management tool. It helps you maintain your keyboard layers in a human-readable text format and builds them into firmware-specific configurations (like QMK).

## Features

- **Project Initialization**: Easily set up a new keyboard configuration project.
- **Layer Formatting**: Keep your layer files organized and visually consistent.
- **Firmware Building**: Generate firmware code for supported manufacturers.

## Installation

To install `kgen` from source, clone the repository and use `cargo`:

```bash
git clone https://github.com/vincbro/kgen.git
cd kgen
cargo install --path .
```

## How It Works

`kgen` operates on a project directory containing a `config.toml` and several layer files (`.txt`).

1. **`config.toml`**: Defines the keyboard layout and the list of layers.
2. **Layer Files**: Text files representing each layer of your keyboard. `kgen` uses these to generate the final keymap.

## CLI Usage

`kgen` provides several subcommands to manage your keyboard project.

### `init`

Initializes a new `kgen` project at the specified path.

```bash
kgen init --path ./my-keyboard
```

This creates the directory and a default `config.toml`.

### `format`

Formats the layer files according to the layout defined in `config.toml`. If a layer file doesn't exist, it creates a template for it.

```bash
kgen format --path ./my-keyboard
```

### `build`

Builds the keymap for a specific manufacturer.

```bash
kgen build --path ./my-keyboard --manufacturer qmk --output ./keymap.c
```

#### Options:
- `-p, --path <PATH>`: Path to the keyboard project directory.
- `-m, --manufacturer <MANUFACTURER>`: The target firmware manufacturer (currently supported: `qmk`; `zmk` is planned). Default is `qmk`.
- `-o, --output <OUTPUT>`: Optional path to save the generated output. If omitted, it prints to stdout.

## Manufacturers

- **QMK**: Fully supported.
- **ZMK**: Support is currently in development.

If your keyboard manufacturer is not listed, feel free to make a pull request adding it! When contributing, please follow the setup in `src/parser/qmk.rs` and ensure you add the new manufacturer to the `Manufacturers` enum.
