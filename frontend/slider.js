const slider_attach = function (element) {
    const target_name = element.getAttribute('data-slider-target');
    const target = document.querySelector(`#${target_name}`);

    const resize = (event) => {
        target.style.flexBasis = `${event.x}px`;
    };

    element.addEventListener("mousedown", () => {
        document.addEventListener("mousemove", resize, false);
        document.addEventListener("mouseup", () => {
            document.removeEventListener("mousemove", resize, false);
        }, false);
    }, false);

};

export { slider_attach };
