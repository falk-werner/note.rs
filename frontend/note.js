class Note {
    #name;
    #list_item;

    constructor(name, notelist) {
        this.#name = name;
        this.#create_listentry(notelist);
    }


    get name() {
        return this.#name;
    }

    #create_listentry(notelist) {
        this.#list_item = document.createElement("li");
        notelist.element.appendChild(this.#list_item);

        this.#list_item.textContent = this.#name;
        this.#list_item.addEventListener('click', async () => {
            console.log("ToDo: activate");
        }, false);
    }
}

export { Note }
