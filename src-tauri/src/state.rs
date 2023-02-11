use serde::{Deserialize, Serialize};
use tauri::async_runtime::{Mutex, RwLock};
use tokio_modbus::client::Context;

use crate::util;

use util::config::Config;
use util::victron::CSVRecord;

pub struct State {
    pub address_mapping: Vec<CSVRecord>,
    pub modbus_connection: Mutex<Option<Context>>,
    pub settings: RwLock<Config>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub address_mapping_file: String,

    pub modbus_host: String,
    pub modbus_port: u16,
    pub modbus_refresh_interval: u32,
}
