import { getVersion } from '@tauri-apps/api/app';

async function init_info() {
    document.querySelector("#version").textContent = await getVersion();
}

export { init_info }
