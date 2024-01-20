
import { EditorView, basicSetup } from "codemirror"
import { Compartment, Text } from "@codemirror/state"
import { markdown } from "@codemirror/lang-markdown"
import { marked } from "marked"

import { RemoveDialog } from "./removedialog.js"
import { NotImplementedDialog } from "./notimplementeddialog.js"

class Editor {

    #title
    #tags
    #active_note
    #editor
    #remove_dialog

    constructor() {
        this.#active_note = null;
        this.#title = document.querySelector("#editor-title");
        this.#tags = document.querySelector("#editor-tags");

        document.querySelector("#editor-save").addEventListener("click", () => {
            this.#save();
        });

        this.#remove_dialog = new RemoveDialog(document.querySelector("#remove-dialog"));
        document.querySelector("#editor-remove").addEventListener("click", () => { 
            this.#remove_dialog.show_modal(this.#active_note); 
        });

        const notImplementedDialog = new NotImplementedDialog();
        document.querySelector("#editor-screenshot").addEventListener("click", () => {
            notImplementedDialog.showModal();
        });
        document.querySelector("#editor-open-folder").addEventListener("click", () => {
            notImplementedDialog.showModal();
        });
        

        const language = new Compartment();
        const editor_element = document.querySelector("#editor");
        this.#editor = new EditorView({
            extensions: [
                basicSetup,
                language.of(markdown())
            ],
            doc: "",
            parent: editor_element
        });

        /*
        editor.dom.addEventListener('input', async () => {
        const text = editor.state.doc.toString();
        const html = marked.parse(text, {
            pedantic: false,
            gfm: true
        });
        document.querySelector("#view").innerHTML = html;
        });
        */

    }

    async set_note(note) {
        this.#save();
        this.#active_note = note;
        this.#title.value = note.name;
        this.#tags.value = note.tags.join(" ");
        
        this.#set_content(await note.get_content());
    }

    #save() {
        if (this.#active_note) {
            this.#active_note.save(
                this.#title.value,
                this.#editor.state.doc.toString(),
                this.#tags.value.split(" "));
        }
    }

    #set_content(content) {
        this.#editor.dispatch({changes: [{
            from: 0,
            to: this.#editor.state.doc.length,
            insert: content
        }]});
    }

    remove() {
        this.#title.value = "";
        this.#tags.value = "";
        this.#set_content("");
        this.#active_note = null;
    }
}

export { Editor }
