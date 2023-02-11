use log::info;
use serde::{Deserialize, Serialize};
use std::net::{AddrParseError, SocketAddr};
use tokio_modbus::client::tcp;
use tokio_modbus::prelude::{Reader, Slave, SlaveContext};

use crate::state::{self, Settings};
use crate::util;
use crate::util::config::Config;
use crate::util::modbus::ModbusValue;

use util::modbus::ModbusReader;
use util::victron::CSVRecord;

#[tauri::command]
pub fn list_registers(state: tauri::State<'_, state::State>) -> Vec<CSVRecord> {
    state.address_mapping.clone()
}

#[tauri::command(async)]
pub async fn connect_modbus(state: tauri::State<'_, state::State>) -> Result<(), String> {
    let settings = state.settings.read().await;
    let mut mutext_connection = state.modbus_connection.lock().await;
    if mutext_connection.is_some() {
        mutext_connection.as_mut().unwrap().disconnect().await;
    }

    info!("Connecting to modbus server");

    let parsed_ip_address = settings.ip.parse();
    if parsed_ip_address.is_err() {
        return parsed_ip_address
            .map_err(|err: AddrParseError| err.to_string())
            .map(|_context| ());
    }

    let socket_addr = SocketAddr::new(parsed_ip_address.unwrap(), settings.port.unwrap_or(502));
    let modbus_connection = tcp::connect(socket_addr).await;

    if let Ok(modbus_connection) = modbus_connection {
        *mutext_connection = Some(modbus_connection);

        Ok(())
    } else {
        *mutext_connection = None;

        modbus_connection
            .map_err(|err| err.to_string())
            .map(|_context| ())
    }
}

#[tauri::command(async)]
pub async fn get_register_value(
    state: tauri::State<'_, state::State>,
    device: String,
    address: u16,
) -> Result<ModbusValue, String> {
    let settings = state.settings.read().await;
    let mut modbus = state.modbus_connection.lock().await;
    if modbus.is_none() {
        return Err("Modbus not connected!".to_string());
    }

    if let Some(slave_id) = settings.id_mapping.get(device.as_str()) {
        let modbus = modbus.as_mut().unwrap();

        modbus.set_slave(Slave::from(*slave_id));

        let csv_record = state
            .address_mapping
            .iter()
            .find(|item| item.dbus_service_name == device && item.address == address)
            .unwrap();

        return modbus
            .read_value(address, csv_record.typ.clone(), csv_record.scaling)
            .await
            .map_err(|_| "Unable to query modbus address".to_string());
    }

    Err("Device ID not found".to_string())
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRegisterRequest {
    pub device: String,
    pub address: u16,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GetRegisterResponse {
    pub device: String,
    pub address: u16,
    pub result: Option<ModbusValue>,
    pub error: Option<String>,
}

#[tauri::command(async)]
pub async fn get_register_values(
    state: tauri::State<'_, state::State>,
    query: Vec<GetRegisterRequest>,
) -> Result<Vec<GetRegisterResponse>, String> {
    let settings = state.settings.read().await;
    let mut modbus = state.modbus_connection.lock().await;
    if modbus.is_none() {
        return Err("Modbus not connected!".to_string());
    }

    let modbus = modbus.as_mut().unwrap();
    let mut results = Vec::new();

    for record in query {
        if let Some(slave_id) = settings.id_mapping.get(record.device.as_str()) {
            modbus.set_slave(Slave::from(*slave_id));

            let csv_record = state
                .address_mapping
                .iter()
                .find(|item| {
                    item.dbus_service_name == record.device && item.address == record.address
                })
                .unwrap();

            results.push(
                modbus
                    .read_value(record.address, csv_record.typ.clone(), csv_record.scaling)
                    .await
                    .map_or_else(
                        |err| GetRegisterResponse {
                            device: record.device.clone(),
                            address: record.address.clone(),
                            result: None,
                            error: Some(format!("read_scaled_value: {}", err.to_string()).into()),
                        },
                        |value| GetRegisterResponse {
                            device: record.device.clone(),
                            address: record.address.clone(),
                            result: Some(value),
                            error: None,
                        },
                    ),
            );
        } else {
            results.push(GetRegisterResponse {
                device: record.device,
                address: record.address,
                result: None,
                error: Some("unit_id_mapping".into()),
            });
        }
    }

    return Ok(results);
}

#[tauri::command]
pub async fn get_settings(state: tauri::State<'_, state::State>) -> Result<Settings, ()> {
    let cfg = state.settings.read().await.clone();

    return Ok(Settings {
        address_mapping_file: cfg.register_list,
        modbus_host: cfg.ip,
        modbus_port: cfg.port.unwrap_or(502),
        modbus_refresh_interval: cfg.refresh_interval,
    });
}

#[tauri::command]
pub async fn set_settings(
    state: tauri::State<'_, state::State>,
    settings: Settings,
) -> Result<(), ()> {
    let mut state_settings = state.settings.write().await;

    *state_settings = Config {
        ip: settings.modbus_host,
        port: Some(settings.modbus_port),
        id_mapping: state_settings.id_mapping.clone(),
        register_list: settings.address_mapping_file,
        refresh_interval: settings.modbus_refresh_interval,
    };

    Ok(())
}

#[tauri::command]
pub async fn available_devices(state: tauri::State<'_, state::State>) -> Result<Vec<String>, ()> {
    let state_settings = state.settings.read().await;
    let mut result = Vec::new();

    for key in state_settings.id_mapping.keys() {
        result.push(key.clone())
    }

    Ok(result)
}
