#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use clap::Parser;
use std::fs;
use tauri::async_runtime::{Mutex, RwLock};
use util::config::Config;

mod commands;
mod state;
mod util;

/// Victron Modbus TCP Viewer
#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Path to the Modbus TCP register csv file
    #[arg(short, long, default_value_t = ("./config.toml".into()))]
    config_file: String,
}

fn main() {
    pretty_env_logger::init();
    let args = Args::parse();

    let cfg_str = fs::read_to_string(args.config_file).unwrap();
    let cfg: Config = toml::from_str(&cfg_str).unwrap();

    // TODO: lazy load after the app has been loaded so we can display error if
    //       the file is missing or malformed.
    let records = util::victron::load_register_table(cfg.register_list.clone());

    tauri::Builder::default()
        .manage(state::State {
            address_mapping: records,
            modbus_connection: Mutex::new(None),
            settings: RwLock::new(cfg),
        })
        .invoke_handler(tauri::generate_handler![
            commands::connect_modbus,
            commands::list_registers,
            commands::get_register_value,
            commands::get_register_values,
            commands::get_settings,
            commands::set_settings,
            commands::available_devices,
        ])
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
