import { tauri } from "@tauri-apps/api";
import { NoteProvider } from "./noteprovider";

class TauriNoteProvider extends NoteProvider {
    async list() {
        return await tauri.invoke("list");
    }

    async read(name) {
        return await tauri.invoke("read", { name: name });
    }

    async create() {
        return await tauri.invoke("create");
    }

    async rename(old_name, new_name) {
        return await tauri.invoke("rename", { 
            oldName: old_name,
            newName: new_name
         });
    }

    async write(name, content) {
        await tauri.invoke("write", { 
            name: name,
            content: content
         });
    }

    async remove(name) {
        await tauri.invoke("remove", { name: name});
    }

    async read_tags(name) {
        return await tauri.invoke("read_tags", { name: name});
    }

    async write_tags(name, tags) {
        await tauri.invoke("write_tags", { 
            name: name,
            tags: tags
        });
    }

    async open_note_directory(name) {
        await tauri.invoke("open_note_directory", {name: name});
    }

    async take_screenshot(name) {
        return await tauri.invoke("take_screenshot", {name: name});
    }    
}

export { TauriNoteProvider }