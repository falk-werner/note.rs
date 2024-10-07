import { core } from "@tauri-apps/api";
import { SettingsProvider } from "./settingsprovider";

class TauriSettingsProvider extends SettingsProvider {

    async readAll() {
        return await core.invoke("read_all_settings");
    }

    async write(name, value) {
        return await core.invoke("write_setting", {
            name: name,
            value: value
        });
    }

}

export { TauriSettingsProvider }
