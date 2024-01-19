import { Note } from "./note.js"

class NoteList {

    #provider
    #element;
    #notes;

    constructor(provider, element) {
        this.#provider = provider;
        this.#element = element;

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
        const note = new Note(name, this);
    }

    get element() {
        return this.#element;
    }

    async add_new() {
        const name = await this.#provider.create();
        this.#add(name);
    }
}

export { NoteList }
