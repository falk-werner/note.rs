class NotImplementedDialog {

    #dialog

    constructor() {
        this.#dialog = document.querySelector("#not-implemented-dialog");
        this.#dialog.querySelector("[data-button=ok]").addEventListener("click", () => {
            this.#dialog.close();
        });
    }

    showModal() {
        this.#dialog.showModal();
    }
}

export { NotImplementedDialog }