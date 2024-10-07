import { marked } from "marked"

class MdRenderer extends marked.Renderer{

    #note

    constructor(note) {
        super();
        this.#note = note
    }

    image(context) {
        if (!context.href.includes("://")) {
            context.href = `local://notes/${this.#note.name}/${context.href}`;
        }
        
        return super.image(context);
    }
}

export { MdRenderer }
