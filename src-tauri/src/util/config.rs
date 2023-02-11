use std::collections::HashMap;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Config {
    pub ip: String,
    pub port: Option<u16>,
    pub refresh_interval: u32,

    pub register_list: String,

    pub id_mapping: HashMap<String, u8>,
}
