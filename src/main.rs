use std::{
    fmt::Display,
    fs::{self, OpenOptions},
    io::{Write, stdout},
    path::Path,
    process,
    str::FromStr,
};

use clap::Parser;
use kgen::{
    document::{Config, Document},
    keyboard::Keyboard,
    keymap::Keymap,
};
use spinners::{Spinner, Spinners};

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(clap::ValueEnum, Clone, Debug)]
pub enum Manufacturer {
    Qmk,
    Zmk,
}

impl FromStr for Manufacturer {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "qmk" => Ok(Manufacturer::Qmk),
            "zmk" => Ok(Manufacturer::Zmk),
            _ => Err(format!("Failed to parse: {s}")),
        }
    }
}

impl Display for Manufacturer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Manufacturer::Qmk => write!(f, "qmk"),
            Manufacturer::Zmk => write!(f, "zmk"),
        }
    }
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
        #[arg(short, long, default_value_t = Manufacturer::Qmk)]
        manufacturer: Manufacturer,
        #[arg(short, long)]
        output: Option<String>,
    },
}

fn init<P>(path: P) -> Result<(), kgen::Error>
where
    P: AsRef<Path>,
{
    fs::create_dir_all(path.as_ref())?;
    let conf_path = path.as_ref().join("config.toml");

    if fs::exists(&conf_path)? {
        panic!("kgen already exists in {:?}", path.as_ref());
    } else {
        let mut file = OpenOptions::new()
            .read(true)
            .write(true)
            .create(true)
            .truncate(true)
            .open(&conf_path)?;

        let doc = Document::new(Config::new(vec![], vec!["base".to_string()]));
        let conf_content = toml::to_string_pretty(&doc)?;
        _ = file.write(conf_content.as_bytes())?;
    }

    Ok(())
}

fn format<P>(path: P) -> Result<(), kgen::Error>
where
    P: AsRef<Path>,
{
    let conf_path = path.as_ref().join("config.toml");
    let doc = Document::from_path(conf_path)?;

    for layer in doc.config.layers {
        let mut layout = Keymap::new(&doc.config.layout);
        let mut layer_path = path.as_ref().join(&layer);
        layer_path.set_extension("txt");
        if fs::exists(&layer_path)? {
            let buf = fs::read_to_string(&layer_path)?;
            layout
                .set_keymap(&buf)
                .unwrap_or_else(|err| panic!("{layer} failed: {err}"));
            let mut file = OpenOptions::new()
                .write(true)
                .truncate(true)
                .open(&layer_path)?;
            write!(file, "{layout}")?;
        } else {
            let mut file = OpenOptions::new()
                .read(true)
                .write(true)
                .create(true)
                .truncate(true)
                .open(&layer_path)?;
            write!(file, "{layout}")?;
        }
    }
    Ok(())
}

fn build<P>(path: P, output: Option<P>, man: Manufacturer) -> Result<(), kgen::Error>
where
    P: AsRef<Path>,
{
    let kb = Keyboard::from_path(path)?;
    let parsed = match man {
        Manufacturer::Qmk => kb.parse_qmk()?,
        Manufacturer::Zmk => todo!(),
    };

    if let Some(output) = output {
        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(&output)?;
        write!(file, "{parsed}")?;
    } else {
        write!(stdout(), "{parsed}")?;
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    match args.command {
        Command::Init { path } => {
            let mut spinner =
                Spinner::new(Spinners::Dots, format!("Creating kgen project @ {path}"));
            match init(&path) {
                Ok(_) => spinner.stop_with_message(format!("Project setup @ {path}")),

                Err(err) => {
                    spinner.stop_with_message(format!("Failed to setup project @ {path}: {err}"));
                    process::exit(1)
                }
            }
        }
        Command::Format { path } => {
            let mut spinner =
                Spinner::new(Spinners::Dots, format!("Formating kgen project @ {path}"));
            match format(&path) {
                Ok(_) => spinner.stop_with_message(format!("Formated project @ {path}")),

                Err(err) => {
                    spinner.stop_with_message(format!("Failed to format project @ {path}: {err}"));
                    process::exit(1)
                }
            }
        }
        Command::Build {
            path,
            output,
            manufacturer,
        } => {
            let mut spinner =
                Spinner::new(Spinners::Dots, format!("Building kgen project @ {path}"));
            match build(&path, output.as_ref(), manufacturer) {
                Ok(_) => spinner.stop_with_message(format!("Built project @ {path}")),

                Err(err) => {
                    spinner.stop_with_message(format!("Failed to build project @ {path}: {err}"));
                    process::exit(1)
                }
            }
        }
    }
}
