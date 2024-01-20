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

    async #add(name) {
        const text = await this.#provider.read(name);
        const tags = await this.#provider.read_tags(name);
        const note = new Note(name, this.#provider, this, this.#editor);
        this.#notes[name] = note;
        if (!this.#active_note) {
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
        this.#add(name);
    }
}

export { NoteList }
