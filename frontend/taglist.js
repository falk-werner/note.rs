
class TagList {

    #element
    #provider
    #tags
    #change_handler

    constructor(element, provider) {
        this.#element = element;
        this.#provider = provider;        
        this.#tags = new Map();
        this.#change_handler = null;

        this.update();
    }

    get active_tags() {
        const result = [];
        for(const [name, active] of this.#tags.entries()) {
            if (active) {
                result.push(name);
            }
        }
        return result;
    }

    set change_handler(new_handler) {
        this.#change_handler = new_handler;
    }

    async update() {
       const new_tags = new Map();
        const notes = await this.#provider.list();
        for(const note of notes) {
            const tags = await this.#provider.read_tags(note);
            for(let tag of tags) {
                tag = tag.toLowerCase();
                if (!new_tags.has(tag)) {
                    const active = this.#tags.has(tag) ? this.#tags.get(tag) : false;
                    new_tags.set(tag, active);
                }
            }
        }
        this.#tags = new_tags;

        this.#element.innerHTML = "";
        const tags = [...this.#tags.entries()].sort();
        for(const [name, active] of tags) {
            const item = document.createElement("div");
            this.#element.appendChild(item);
            item.textContent = name;            
            if (active) {
                item.classList.add("active");
            }
            item.addEventListener("click", () => {
                this.toggle(item, name);
            },false);
        }
        
        this.#fire();
    }

    toggle(item, tag) {
        if (this.#tags.has(tag)) {
            const active = !this.#tags.get(tag);
            this.#tags.set(tag, active);

            if (active) {
                item.classList.add("active");
            }
            else {
                item.classList.remove("active");
            }
        }

        this.#fire();
    }

    #fire() {
        if (this.#change_handler) {
            this.#change_handler();
        }
    }
}

export { TagList }