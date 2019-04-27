#[macro_use]
extern crate log;

use clap_log_flag;
use clap_verbosity_flag;
use nest::{Error, Schema, Store, Value};
use serde_json as json;
use std::env;
use std::fs::read_to_string;
use std::io::{self, Read};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(
    name = "nest",
    rename_all = "kebab-case",
    long_about = "\nUse your filesystem as a nested data store!",
    raw(setting = "structopt::clap::AppSettings::ColoredHelp")
)]
struct Cli {
    #[structopt(subcommand)]
    command: Command,
    #[structopt(flatten)]
    log: clap_log_flag::Log,
    #[structopt(long = "root", parse(from_os_str))]
    root: Option<PathBuf>,
    #[structopt(flatten)]
    verbose: clap_verbosity_flag::Verbosity,
}

#[derive(Debug, StructOpt)]
enum Command {
    #[structopt(name = "get")]
    /// Get value from Nest.
    Get { path: String },

    #[structopt(name = "set")]
    /// Set value in Nest.
    Set { path: String, value: Option<String> },
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Cli::from_args();

    args.log.log_all(Some(args.verbose.log_level()))?;

    info!("nest-cli: {}", env::var("CARGO_PKG_VERSION")?);
    debug!("args: {:#?}", args);

    let root = match args.root {
        Some(path) => path,
        None => env::var("NEST_ROOT")
            .map(|s| s.into())
            .or(env::current_dir())?,
    };

    debug!("root: {:#?}", root);
    let schema_path = root.join(".nest.json");
    debug!("schema path: {:#?}", schema_path);
    if !schema_path.is_file() {
        error!("schema path is not a file: {:#?}", schema_path);
        return Ok(());
    }
    let schema_str = read_to_string(schema_path)?;
    debug!("schema string: {:#?}", schema_str);
    let schema_json: json::Value = json::from_str(&schema_str)?;
    debug!("schema json: {:#?}", schema_json);
    let schema: Schema = schema_json.into();
    debug!("schema: {:#?}", schema);

    let store = Store::new(root, schema);

    match args.command {
        Command::Get { path } => {
            let path = parse_path(&path);
            let value = store.get(&path)?;
            let value_json: json::Value = value.into();
            let value_string = json::to_string_pretty(&value_json)?;
            println!("{}", value_string);
        }
        Command::Set { path, value } => {
            let path = parse_path(&path);
            let value_str: String = match value {
                Some(value_string) => value_string,
                None => {
                    let mut string = String::new();
                    let stdin = io::stdin();
                    let mut handle = stdin.lock();
                    handle.read_to_string(&mut string)?;
                    string
                }
            };
            let value_json: json::Value = json::from_str(&value_str)?;
            let value: Value = value_json.into();
            store.set(&path, &value)?;
        }
    }

    Ok(())
}

fn parse_path(path: &String) -> Vec<&str> {
    if path != "" {
        path.split('/').collect()
    } else {
        Vec::new()
    }
}
