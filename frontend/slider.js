const slider_attach = function (element) {
    const body = document.querySelector("body");
    const left = element.previousElementSibling;
    const right = element.nextElementSibling;
    const minWidth = left.clientWidth;

    const resize = (event) => {        
        left.style.minWidth = `${Math.max(event.x, minWidth)}px`;
    };

    element.addEventListener("mousedown", () => {
        left.style.userSelect = "none";
        left.style.webkitUserSelect = "none";
        right.style.webkitUserSelect = "none";
        right.style.userSelect = "none";
        body.style.cursor = "col-resize";

        document.addEventListener("mousemove", resize, false);
        document.addEventListener("mouseup", () => {
            left.style.userSelect = null;
            left.style.webkitUerSelect = null;
            right.style.userSelect = null;
            right.style.webkitUserSelect = null;
            body.style.cursor = null;

            document.removeEventListener("mousemove", resize, false);
        }, false);
    }, false);

};

export { slider_attach };
