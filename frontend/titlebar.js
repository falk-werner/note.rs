import { getCurrentWebviewWindow } from '@tauri-apps/api/webviewWindow'
const appWindow = getCurrentWebviewWindow()

const init_titlebar = function(onclose) {
    const settings = document.querySelector("#titlebar-settings");
    settings.addEventListener('click', () => {
        document.querySelector("#main").classList.add("hidden");
        document.querySelector("#settings").classList.remove("hidden");
        document.querySelector("#info").classList.add("hidden");
    });
    const info = document.querySelector("#titlebar-info");
    info.addEventListener('click', () => {
        document.querySelector("#main").classList.add("hidden");
        document.querySelector("#settings").classList.add("hidden");
        document.querySelector("#info").classList.remove("hidden");
    });

    const minimize = document.querySelector('#titlebar-minimize');
    minimize.addEventListener('click', () => appWindow.minimize());

    const maximize = document.querySelector('#titlebar-maximize');
    maximize.addEventListener('click', () => appWindow.toggleMaximize());

    const close = document.querySelector('#titlebar-close');
    close.addEventListener('click', async () => {
        try {
            await onclose();
        }
        catch (ex) {
            console.error("onclose handler failed", ex);
        }
        appWindow.close();
    });

    const nav_main = document.querySelectorAll(".nav-main");
    nav_main.forEach((item) => {
        item.addEventListener('click', () => {
            document.querySelector("#main").classList.remove("hidden");
            document.querySelector("#settings").classList.add("hidden");    
            document.querySelector("#info").classList.add("hidden");    
        });
    });

    // disable context menu
    //document.addEventListener('contextmenu', event => event.preventDefault());
}

export { init_titlebar }
