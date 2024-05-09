class ErrorDialog {

    #dialog
    #message

    constructor() {
        this.#dialog = document.querySelector("#error-dialog");
        this.#message = this.#dialog.firstElementChild;
        this.#dialog.querySelector("[data-button=ok]").addEventListener("click", () => {
            this.#dialog.close();
        });
    }

    showModal(message) {
        this.#message.textContent = message;
        this.#dialog.showModal();
    }
}

export { ErrorDialog }