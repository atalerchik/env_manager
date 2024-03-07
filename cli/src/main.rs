mod config;

use clap::{App, Arg};
use config::AppConfig;
use std::path::Path;

fn main() {
    let _matches = App::new("My Env Manager")
        .version("1.0")
        .author("Alexey Talerchik")
        .about("Manages .env files securely")
        .arg(Arg::with_name("config")
             .short('c')
             .long("config")
             .value_name("FILE")
             .help("Sets a custom config file")
             .takes_value(true))
        .get_matches();

    let config_path = _matches.value_of("config").unwrap_or("config/default.json");
    let config = AppConfig::new(Path::new(config_path)).expect("Failed to load config");

    println!("Using env files path: {}", config.get_env_files_path());
    println!("Using GPG key ID: {}", config.get_gpg_key_id());
}

