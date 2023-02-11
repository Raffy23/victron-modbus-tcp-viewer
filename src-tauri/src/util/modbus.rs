use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::default;
use std::io::{Error, ErrorKind};
use tokio_modbus::client::Context;
use tokio_modbus::prelude::Reader;

pub(crate) type Address = u16;
pub(crate) type Quantity = u16;
pub(crate) type Scaling = f32;
pub(crate) type Type = String;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum ModbusValue {
    Number(f32),
    String(String),
}

#[async_trait]
pub trait ModbusReader {
    async fn read_i16(&mut self, _: Address) -> Result<i16, Error>;
    async fn read_u16(&mut self, _: Address) -> Result<u16, Error>;

    async fn read_i32(&mut self, _: Address) -> Result<i32, Error>;
    async fn read_u32(&mut self, _: Address) -> Result<u32, Error>;

    async fn read_string(&mut self, _: Address, _: Quantity) -> Result<String, Error>;

    async fn read_value(&mut self, _: Address, _: Type, _: Scaling) -> Result<ModbusValue, Error>;
}

#[async_trait]
impl ModbusReader for Context {
    async fn read_i16<'a>(&'a mut self, addr: Address) -> Result<i16, Error> {
        self.read_input_registers(addr, 1)
            .await
            .map(|words| words[0] as i16)
    }

    async fn read_u16<'a>(&'a mut self, addr: Address) -> Result<u16, Error> {
        self.read_input_registers(addr, 1)
            .await
            .map(|words| words[0] as u16)
    }

    async fn read_i32<'a>(&'a mut self, addr: Address) -> Result<i32, Error> {
        self.read_input_registers(addr, 2).await.map(|words| {
            let high = words[0] as u32;
            let low = words[1] as u32;

            (high << 16 | low) as i32
        })
    }

    async fn read_u32<'a>(&'a mut self, addr: Address) -> Result<u32, Error> {
        self.read_input_registers(addr, 2).await.map(|words| {
            let high = words[0] as u32;
            let low = words[1] as u32;

            high << 16 | low
        })
    }

    async fn read_string<'a>(
        &'a mut self,
        base_address: Address,
        size: Quantity,
    ) -> Result<String, Error> {
        self.read_input_registers(base_address, size)
            .await
            .and_then(|words| {
                let str_content = words
                    .iter()
                    .flat_map(|value| value.to_be_bytes())
                    .collect::<Vec<u8>>();

                String::from_utf8(str_content)
                    .map_err(|error| Error::new(ErrorKind::Other, error.to_string()))
            })
            .map(|string| string.trim_end_matches(char::from('\0')).to_string())
    }

    async fn read_value(
        &mut self,
        addr: Address,
        typ: Type,
        scaling: Scaling,
    ) -> Result<ModbusValue, Error> {
        match typ.as_str() {
            "int16" => self
                .read_i16(addr)
                .await
                .map(|value| ModbusValue::Number((value as f32) / scaling)),

            "uint16" => self
                .read_u16(addr)
                .await
                .map(|value| ModbusValue::Number((value as f32) / scaling)),

            "int32" => self
                .read_i32(addr)
                .await
                .map(|value| ModbusValue::Number((value as f32) / scaling)),

            "uint32" => self
                .read_u32(addr)
                .await
                .map(|value| ModbusValue::Number((value as f32) / scaling)),

            default if default.starts_with("string[") => {
                let mut size = default.strip_prefix("string[").unwrap();
                size = size.strip_suffix("]").unwrap();

                self.read_string(addr, size.parse::<u16>().unwrap())
                    .await
                    .map(|value| ModbusValue::String(value))
            }

            _default => Err(Error::new(ErrorKind::Other, "not implemented")),
        }
    }
}
