Victron ModbusTCP Viewer
========================

This repository contains a  [Tauri App](https://tauri.app/) that can be used to visualize different registers of the Modbus TCP Server of the Victron Cerbo GX device.

## Building
* Make sure Rust (1.59+) and Node (v20+) is installed
* Run `npm ci` to install dependencies
* Run `npm run tauri build` for a production build
* Copy `./src-tauri/target/release/victron-modbus-tcp-viewer` or the generated bundle to the target directory 


## Configuration
To be able to use the viewer application the following files are needed in the same directory as the executable:

* *CCGX-Modbus-TCP-register-list-2.90.csv*: This CSV contains the the contents of the "Field List" Sheet of the .xslx file that can be downloaded from the [official website](https://www.victronenergy.com/download-document/6195/CCGX-Modbus-TCP-register-list-2.90.xlsx). Make sure to export the Sheet as UTF-8. 

* *config.toml*: Configuration file of the viewer must contain the following structure:
  ```toml
  ip = '<IP of the Victron Cerbo GX device>'
  port = 502
  register_list = './CCGX-Modbus-TCP-register-list-2.90.csv'
  refresh_interval = 1000

  [id_mapping]
  'com.victronenergy.system'     = 100
  'com.victronenergy.hub4'       = 100
  'com.victronenergy.vebus'      = <ID of the vebus device>
  'com.victronenergy.battery'    = <ID of the battery device>
  'com.victronenergy.pvinverter' = <ID of the pvinverter device>
  <...>
  ```

  Note: The ID of the device can be queried from the Victron Cerbo GX device by going to it's webinterface and navigating to Settings > Services > Modbus TCP > Devices

## Development
The development server can be launched with `npm run tauri dev`. It may take some time until the Tauri App is started and connects to the webpack dev server.
