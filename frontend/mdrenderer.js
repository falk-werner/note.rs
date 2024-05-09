import { marked } from "marked"

class MdRenderer extends marked.Renderer{

    #note

    constructor(note) {
        super();
        this.#note = note
    }

    image(href, title, text) {
        if (!href.includes("://")) {
            href = `local://notes/${this.#note.name}/${href}`;
        }
        
        return super.image(href, title, text);
    }
}

export { MdRenderer }
