use figment::{
    providers::{Format, Toml},
    value::Map,
    Figment,
};
use serde::{Deserialize, Serialize};

use clap::Parser;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub clients_mac_address: Map<String, String>,
}

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    /// config file path e.g. config.toml
    #[arg(short, long, default_value_t = String::from("config.toml"))]
    pub cfg_file_path: String,

    /// clinet name to wake up
    #[arg(short, long)]
    pub name: String,
}

pub fn get_mac_to_boot() -> (String, String) {
    let cli = Cli::parse();

    let cfg = match Figment::new()
        .merge(Toml::file(&cli.cfg_file_path))
        .extract::<AppConfig>()
    {
        Ok(cfg) => cfg,
        Err(err) => panic!("Error parsing cfg file {}, {}", cli.cfg_file_path, err),
    };
    let mac_address = cfg.clients_mac_address[&cli.name].clone();
    (cli.name, mac_address)
}
