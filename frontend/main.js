
import "lineicons/web-font/lineicons.css"
import { EditorView, basicSetup } from "codemirror"
import { Compartment, Text } from "@codemirror/state"
import { markdown } from "@codemirror/lang-markdown"
import { marked } from "marked"

import { slider_attach } from "./slider.js"
import { init_titlebar } from "./titlebar.js"
import { init_settings } from "./settings.js"

init_titlebar();
init_settings();
slider_attach(document.querySelector("#slider"));


const language = new Compartment();
const editor_element = document.querySelector("#editor");
const editor = new EditorView({
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

