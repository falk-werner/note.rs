import { tauri } from "@tauri-apps/api";
import { SettingsProvider } from "./settingsprovider";

class TauriSettingsProvider extends SettingsProvider {

    async readAll() {
        return await tauri.invoke("read_all_settings");
    }

    async write(name, value) {
        return await tauri.invoke("write_setting", {
            name: name,
            value: value
        });
    }

}

export { TauriSettingsProvider }
