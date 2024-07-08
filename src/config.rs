use serde::{Deserialize, Serialize};

use clap::Parser;
use toml::Table;

#[derive(Debug, Serialize, Deserialize)]
pub struct AppConfig {
    pub clients_mac_address: Table,
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

    let cfg =
        toml::from_str::<AppConfig>(&std::fs::read_to_string(&cli.cfg_file_path).unwrap()).unwrap();

    let mac_address = cfg.clients_mac_address[&cli.name]
        .as_str()
        .unwrap()
        .to_string();
    (cli.name, mac_address)
}
