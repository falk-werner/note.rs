
import { EditorView, basicSetup } from "codemirror"
import { Compartment, Text } from "@codemirror/state"
import { markdown } from "@codemirror/lang-markdown"
import { marked } from "marked"

import { RemoveDialog } from "./removedialog.js"
import { ErrorDialog } from "./errordialog.js"
import { MdRenderer } from "./mdrenderer.js"

class Editor {

    #title
    #tags
    #active_note
    #editor
    #content_element
    #view
    #remove_dialog
    #mode
    #mode_change_handler

    constructor() {
        this.#active_note = null;
        this.#title = document.querySelector("#editor-title");
        this.#tags = document.querySelector("#editor-tags");
        this.#view = document.querySelector("#view");
        this.#content_element = document.querySelector("#content");
        this.#mode_change_handler = () => {};

        document.querySelector("#editor-save").addEventListener("click", () => {
            this.save();
        });

        this.#remove_dialog = new RemoveDialog(document.querySelector("#remove-dialog"));
        document.querySelector("#editor-remove").addEventListener("click", () => {
            this.removeNote();
        });

        const errorDialog = new ErrorDialog();
        document.querySelector("#editor-screenshot").addEventListener("click", async () => {
            this.takeScreenshot();
        });

        document.querySelector("#editor-open-folder").addEventListener("click", async () => {
            this.openNoteDirectory();
        });
        

        const language = new Compartment();
        const editor_element = document.querySelector("#editor");
        this.#editor = new EditorView({
            extensions: [
                basicSetup,
                EditorView.lineWrapping,
                language.of(markdown())
            ],
            doc: "",
            parent: editor_element
        });

        this.mode = "view";
    }

    get mode() {
        return this.#mode;
    }

    set mode(new_mode) {
        switch (new_mode) {
            case "edit":
                this.#mode = "edit";
                this.#view.classList.add("hidden");
                this.#content_element.classList.remove("hidden");
                break;
            case "view":
                // fall-through
            default:
                this.#mode = "view";
                this.#view.classList.remove("hidden");
                this.#content_element.classList.add("hidden");

                const renderer = new MdRenderer(this.#active_note);
                const content = this.#editor.state.doc.toString();
                const html = marked.parse(content, {
                    pedantic: false,
                    gfm: true,
                    renderer: renderer
                });
                this.#view.innerHTML = html;        
                break;
        }

        this.#mode_change_handler();
    }

    set mode_change_handler(handler) {
        this.#mode_change_handler = handler;
    }

    async set_note(note) {
        await this.save();
        this.#active_note = note;
        this.#title.value = note.name;
        this.#tags.value = note.tags.join(" ");
        
        this.#set_content(note.content);
    }

    async save() {
        if (this.#active_note) {
            await this.#active_note.save(
                this.#title.value,
                this.#editor.state.doc.toString(),
                this.#tags.value.split(" ").filter((item) => item != ""));
        }
    }

    #set_content(content) {
        this.#editor.dispatch({changes: [{
            from: 0,
            to: this.#editor.state.doc.length,
            insert: content
        }]});

        const renderer = new MdRenderer(this.#active_note);
        const html = marked.parse(content, {
            pedantic: false,
            gfm: true,
            renderer: renderer
        });
        this.#view.innerHTML = html;
    }

    remove() {
        this.#title.value = "";
        this.#tags.value = "";
        this.#set_content("");
        this.#active_note = null;
    }

    async takeScreenshot() {
        try {
            const filename = await this.#active_note.take_screenshot();
            this.#editor.dispatch(this.#editor.state.replaceSelection(`![screenshot](${filename})\n`));
        }
        catch (ex) {
            errorDialog.showModal("Failed to take screenshot. Check if gnome-screenshot is installed.");
        }
    }

    async openNoteDirectory() {
        try {
            await this.#active_note.open_note_directory();
        }
        catch (ex) {
            errorDialog.showModal("Failed to open folder");
        }
    }

    removeNote() {
        this.#remove_dialog.show_modal(this.#active_note); 
    }
}

export { Editor }
