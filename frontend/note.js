class Note {
    #name;
    #provider;
    #editor;
    #list_item;
    #notelist

    constructor(name, provider, notelist, editor) {
        this.#name = name;
        this.#provider = provider;
        this.#editor = editor;
        this.#notelist = notelist
        this.#create_listentry();
    }


    get name() {
        return this.#name;
    }

    async get_content() {
        return await this.#provider.read(this.#name);
    }

    #create_listentry() {
        this.#list_item = document.createElement("li");
        this.#notelist.element.appendChild(this.#list_item);

        this.#list_item.textContent = this.#name;
        this.#list_item.addEventListener('click', async () => {
            this.#notelist.activate(this);
        }, false);
    }

    activate() {
        this.#list_item.classList.add("active");
        this.#editor.set_note(this);

    }

    async deactivate() {
        this.save();
        this.#list_item.classList.remove("active");
    }

    async save(name, content, tags) {
        this.#provider.write(this.#name, content);
    }

    async remove() {
        await this.#provider.remove(this.#name);
        this.#list_item.remove();
        this.#editor.remove();
        this.#notelist.remove(this);
    }
}

export { Note }
