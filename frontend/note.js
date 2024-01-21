
function arrays_equals(a, b) {
    if (a.length != b.length) { 
        return false;
    }

    for(const item of a) {
        if (!b.includes(a)) {
            return false;
        }
    }

    return true;
}

class Note {
    #name;
    #content;
    #tags;
    #provider;
    #taglist;
    #editor;
    #list_item;
    #notelist;

    constructor(name, content, tags, provider, notelist, taglist, editor) {
        this.#name = name;
        this.#content = content;
        this.#tags = tags;
        this.#provider = provider;
        this.#taglist = taglist;
        this.#editor = editor;
        this.#notelist = notelist;
        this.#create_listentry();
    }


    get name() {
        return this.#name;
    }

    get tags() {
        return this.#tags;
    }

    get content() {
        return this.#content;
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
        this.#list_item.classList.remove("active");
    }

    async save(name, content, tags) {
        if (name != this.#name) {
            await this.#provider.rename(this.#name, name);
            this.#list_item.textContent = name;
            this.#notelist.rename(this.#name, name);
            this.#name = name;
        }

        if (content != this.#content) {
            this.#content = content;
            await this.#provider.write(this.#name, content);
        }

        if (!arrays_equals(this.#tags, tags)) {
            this.#tags = tags;
            await this.#provider.write_tags(this.#name, tags);
            this.#taglist.update();    
        }
    }

    async remove() {
        await this.#provider.remove(this.#name);
        this.#list_item.remove();
        this.#editor.remove();
        this.#notelist.remove(this);
    }

    #filter_by_tags(tags) {
        if (tags.length == 0) {
            return true;
        }

        for(let tag of this.#tags) {
            tag = tag.toLowerCase();
            if (tags.includes(tag)) {
                return true;
            }
        }

        return false;
    }

    apply_filter(filter, tags) {
        const name = this.#name.toLowerCase();
        const content = this.#content.toLowerCase();
        if ( (this.#filter_by_tags(tags)) &&
             ((name.includes(filter)) || (content.includes(filter))) ) {
            this.#list_item.classList.remove('hidden');
        }
        else {
            this.#list_item.classList.add('hidden');
        }
    }
}

export { Note }
