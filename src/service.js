import { invoke } from '@tauri-apps/api'

export default {
  connectModbus() {
    return invoke('connect_modbus');
  },

  listRegisters() {
    return invoke('list_registers');
  },

  getRegisterValues(query) {
    return invoke('get_register_values', { query });
  },

  getSettings() {
    return invoke('get_settings');
  },

  setSettings(settings) {
    return invoke('set_settings', { settings });
  },

  availableDevices() {
    return invoke('available_devices');
  }
}
