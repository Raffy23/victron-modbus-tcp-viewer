use log::info;
use serde::{de, Deserialize, Deserializer, Serialize};
use std::fs::OpenOptions;
use std::io::{BufRead, BufReader};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct CSVRecord {
    pub dbus_service_name: String,
    pub description: String,
    #[serde(rename = "Address")]
    pub address: u16,
    #[serde(rename = "Type")]
    pub typ: String,
    #[serde(rename = "Scalefactor")]
    #[serde(deserialize_with = "deserialize_float")]
    pub scaling: f32,
    #[serde(rename = "Range")]
    pub range: String,
    pub dbus_obj_path: String,
    #[serde(deserialize_with = "deserialize_bool")]
    pub writable: bool,
    pub dbus_unit: String,
    #[serde(rename = "Remarks")]
    pub remarks: String,
}

pub fn load_register_table(file_path: String) -> Vec<CSVRecord> {
    info!("Loading modbus registers from {}", file_path);

    let file = OpenOptions::new()
        .read(true)
        .open(&file_path)
        .expect("Unable to open modbus register list csv");

    let csv_content = BufReader::new(file)
        .lines()
        .skip(1)
        .map(|x| x.unwrap())
        .collect::<Vec<String>>()
        .join("\n");

    let mut csv_reader = csv::ReaderBuilder::new().from_reader(csv_content.as_bytes());
    let records: Vec<CSVRecord> = csv_reader
        .deserialize::<CSVRecord>()
        .map(|x| x.unwrap())
        .collect::<Vec<CSVRecord>>()
        .try_into()
        .unwrap();

    records
}

fn deserialize_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    match s {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["yes", "no"])),
    }
}

fn deserialize_float<'de, D>(deserializer: D) -> Result<f32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;
    if s.is_empty() {
        return Ok(0.0);
    }

    s.replace(',', ".")
        .parse()
        .map_err(|_| de::Error::unknown_variant(s, &["####,###"]))
}
