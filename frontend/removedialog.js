
class RemoveDialog {

    #dialog
    #text
    #note

    constructor(dialog) {
        this.#dialog = dialog;
        this.#text = dialog.querySelector("[data-text]");
        this.#note = null;

        dialog.querySelector("[data-button=yes]").addEventListener("click", async () => {
            if (this.#note) {
                this.#note.remove();
            }
            this.#dialog.close()
        })

        dialog.querySelector("[data-button=no]").addEventListener("click", () => {
            this.#dialog.close()
        })
    }

    show_modal(note) {
        if (note) {
            this.#note = note;
            this.#text.textContent = `Do you really want to remove note \"${note.name}\"?`;
            this.#dialog.showModal();
            this.#dialog.querySelector("[data-button=no]").focus();    
        }
    }
}

export { RemoveDialog }