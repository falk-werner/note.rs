import { appWindow } from '@tauri-apps/api/window'

function init_accelerators(editor) {
    const body = document.querySelector("body");
    body.addEventListener("keypress", (event)=> {
      if (event.ctrlKey) {
        switch (event.key) {
            case 'n':
                notelist.add_new();
                break;
            case 'd':
                editor.removeNote();
                break;
            case 's':
                editor.save();
                console.log("save");
                break;
            case 'b':
                editor.openNoteDirectory();
                break;
            case 'p':
                editor.mode = "edit";
                editor.takeScreenshot();
                break;
            case 'q':
                editor.save().then(() => {
                    appWindow.close();
                });
                break;
            case 'e':
                editor.mode = (editor.mode == "edit") ? "view" : "edit";
                break;
            case 'f':
                document.querySelector("#filter").focus();
                break;
            default:
                break;
        }
      }    
    });    
}

export { init_accelerators }
