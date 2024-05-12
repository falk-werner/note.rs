
import "lineicons/web-font/lineicons.css"

import { slider_attach } from "./slider.js"
import { init_titlebar } from "./titlebar.js"
import { init_settings } from "./settings.js"
import { init_info } from "./info.js"
import { TauriNoteProvider } from "./taurinoteprovider.js"
import { NoteList } from "./notelist.js"
import { Editor } from "./editor.js"
import { TagList } from "./taglist.js"
import { TauriSettingsProvider } from "./taurisettingsprovider.js"

init_info();
slider_attach(document.querySelector("#slider"));

const editor = new Editor();
editor.mode_change_handler = () => {
  const icon = document.querySelector("#toggle-mode > i");
  if (editor.mode == "edit") {
      icon.classList.add("lni-pencil");
      icon.classList.remove("lni-eye");
  }
  else {
      icon.classList.add("lni-eye");
      icon.classList.remove("lni-pencil");
  }
};
document.querySelector("#toggle-mode").addEventListener("click", () => {
  editor.mode = (editor.mode == "edit") ? "view" : "edit";
});


const noteProvider = new TauriNoteProvider();

const taglist_elemnt = document.querySelector("#taglist");
const taglist = new TagList(taglist_elemnt, noteProvider);

const notelist_element = document.querySelector("#notelist");
const filter_element = document.querySelector('#filter');
const notelist = new NoteList(noteProvider, notelist_element, filter_element, taglist, editor);
document.querySelector("#add-note").addEventListener("click", async () => {
  notelist.add_new();
})

init_titlebar(async () => {
  await editor.save();
});

init_settings(new TauriSettingsProvider(), () => {
  editor.remove();
  notelist.update();
  taglist.update();
});





