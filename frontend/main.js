
import "lineicons/web-font/lineicons.css"

import { slider_attach } from "./slider.js"
import { init_titlebar } from "./titlebar.js"
import { init_settings } from "./settings.js"
import { FakeNoteProvider } from "./fakenoteprovider.js"
import { NoteList } from "./notelist.js"
import { Editor } from "./editor.js"
import { TagList } from "./taglist.js"

init_titlebar();
init_settings();
slider_attach(document.querySelector("#slider"));

const editor = new Editor();

const noteProvider = new FakeNoteProvider();

const taglist_elemnt = document.querySelector("#taglist");
const taglist = new TagList(taglist_elemnt, noteProvider);

const notelist_element = document.querySelector("#notelist");
const filter_element = document.querySelector('#filter');
const notelist = new NoteList(noteProvider, notelist_element, filter_element, taglist, editor);
document.querySelector("#add-note").addEventListener("click", async () => {
  notelist.add_new();
})




