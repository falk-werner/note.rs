
import "lineicons/web-font/lineicons.css"

import { slider_attach } from "./slider.js"
import { init_titlebar } from "./titlebar.js"
import { init_settings } from "./settings.js"
import { FakeNoteProvider } from "./fakenoteprovider.js"
import { NoteList } from "./notelist.js"
import { Editor } from "./editor.js"

init_titlebar();
init_settings();
slider_attach(document.querySelector("#slider"));

const editor = new Editor();

const noteProvider = new FakeNoteProvider();
const notelist_element = document.querySelector("#notelist");
const notelist = new NoteList(noteProvider, notelist_element, editor);
document.querySelector("#add-note").addEventListener("click", async () => {
  notelist.add_new();
})




