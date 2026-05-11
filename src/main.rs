use std::{
    fs::{self, File, OpenOptions},
    io::{self, Read, Write},
    path::Path,
};

use clap::Parser;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
struct Document {
    pub config: Config,
}

impl Document {
    #[must_use]
    pub fn new(config: Config) -> Self {
        Self { config }
    }
}

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    pub layout: Vec<String>,
    pub layers: Vec<String>,
}

impl Config {
    #[must_use]
    pub fn new(layout: Vec<String>, layers: Vec<String>) -> Self {
        Self { layout, layers }
    }
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::Subcommand, Debug)]
enum Command {
    Init {
        #[arg(short, long)]
        path: String,
    },
    Format {
        #[arg(short, long)]
        path: String,
    },
    Build {
        #[arg(short, long)]
        path: String,
    },
}

fn init<P>(path: P) -> Result<(), io::Error>
where
    P: AsRef<Path>,
{
    fs::create_dir_all(&path)?;

    let conf_path = path.as_ref().join("config.toml");

    if !(fs::exists(&conf_path)?) {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&conf_path)?;

        let doc = Document::default();

        let conf_content = toml::to_string_pretty(&doc).unwrap();
        _ = file.write(conf_content.as_bytes())?;
    }

    Ok(())
}

// fn format<P>(path: P) where P: AsRef<Path> {
//
// }
//
// fn build<P>(path: P) where P: AsRef<Path> {
//
// }

fn main() {
    let args = Args::parse();
    dbg!(&args);

    match args.command {
        Command::Init { path } => init(path).unwrap(),
        Command::Format { path } => todo!(),
        Command::Build { path } => todo!(),
    }

    // let mut file = File::open(args.path).unwrap();
    // let mut buffer = String::new();
    //
    // file.read_to_string(&mut buffer).unwrap();
    // let doc = toml::from_str::<Document>(&buffer).unwrap();
    // for layer in doc.config.layers {
    //     println!("{layer}");
    // }
}
