import { core } from "@tauri-apps/api";
import { NoteProvider } from "./noteprovider";

class TauriNoteProvider extends NoteProvider {
    async list() {
        return await core.invoke("list");
    }

    async read(name) {
        return await core.invoke("read", { name: name });
    }

    async create() {
        return await core.invoke("create");
    }

    async rename(old_name, new_name) {
        return await core.invoke("rename", { 
            oldName: old_name,
            newName: new_name
         });
    }

    async write(name, content) {
        await core.invoke("write", { 
            name: name,
            content: content
         });
    }

    async remove(name) {
        await core.invoke("remove", { name: name});
    }

    async read_tags(name) {
        return await core.invoke("read_tags", { name: name});
    }

    async write_tags(name, tags) {
        await core.invoke("write_tags", { 
            name: name,
            tags: tags
        });
    }

    async open_note_directory(name) {
        await core.invoke("open_note_directory", {name: name});
    }

    async take_screenshot(name) {
        return await core.invoke("take_screenshot", {name: name});
    }    
}

export { TauriNoteProvider }