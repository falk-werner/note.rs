import { Note } from "./note.js"

class NoteList {

    #provider
    #element;
    #editor;
    #notes;
    #active_note;

    constructor(provider, element, editor) {
        this.#provider = provider;
        this.#element = element;
        this.#editor = editor;
        this.#active_note = null;

        this.#update();
    }

    async #update() {
        this.#element.innerHTML = "";
        this.#notes = new Map();

        const notes = await this.#provider.list();
        for (const name of notes) {
            await this.#add(name);
        }
    }

    async #add(name, activate) {
        const tags = await this.#provider.read_tags(name);
        const note = new Note(name, tags, this.#provider, this, this.#editor);
        this.#notes.set(name, note);
        if ((!this.#active_note) || (activate)) {
            this.activate(note);
        }
    }

    get element() {
        return this.#element;
    }

    activate(note) {
        if (this.#active_note) {
            this.#active_note.deactivate();            
        }
        this.#active_note = note;
        this.#active_note.activate();
    }

    async add_new() {
        const name = await this.#provider.create();
        this.#add(name, true);
    }

    rename(old_name, new_name) {
        if (this.#notes.has(old_name)) {
            const note = this.#notes.get(old_name);
            this.#notes.delete(old_name);
            this.#notes.set(new_name, note);
        }
    }

    remove(note) {
        this.#notes.delete(note.name);
        if (this.#active_note == note) {
            this.#active_note = null;
            if (this.#notes.size > 0) {
                this.activate(this.#notes.values().next().value);
            }
        }
    }
}

export { NoteList }
