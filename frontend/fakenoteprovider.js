import { NoteProvider } from "./noteprovider";

class Note {
    name;
    content;
    tags;

    constructor(name, content) {
        this.name = name;
        this.content = content || "";
        this.tags = [];
    }
}

class FakeNoteProvider extends NoteProvider {

    #notes;

    constructor() {
        super();
        this.#notes = new Map([
            ["Sample", new Note("Sample", "# Sample Note\n")]
        ]);
    }

    #get_note(name) {
        const note = this.#notes.get(name);
        if (!note) { throw new Error(`unknown note \"${name}\"`); }
        return note;
    }

    async list() {
        return [...this.#notes.keys()];
    }

    async read(name) {
        const note = this.#get_note(name);
        return note.content;
    }

    async create() {
        let name = "Untitled";
        let count = 0;
        while (this.#notes.has(name)) {
            count++;
            name = `Untitled ${count}`;
        }

        this.#notes.set(name, new Note(name));
        return name;
    }

    async rename(old_name, new_name) {
        const note = this.#get_note(old_name);
        if (this.#notes.has(new_name)) {
            throw new Error(`failed to rename to an existing note \"${new_name}\"`);
        }

        this.#notes.delete(old_name);
        note.name = new_name;
        this.#notes.set(note.name, note);
    }

    async write(name, content) {
        const note = this.#get_note(name);
        note.content = content;
    }

    async remove(name) {
        if (this.#notes.has(name)) {
            this.#notes.delete(name);
        }
    }

    async read_tags(name) {
        const note = this.#get_note(name);
        return note.tags;
    }

    async write_tags(name, tags) {
        const note = this.#get_note(name);
        note.tags = tags;
    }
}

export { FakeNoteProvider }