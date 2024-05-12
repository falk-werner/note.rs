[![Build](https://github.com/falk-werner/note.rs/actions/workflows/build.yaml/badge.svg)](https://github.com/falk-werner/note.rs/actions/workflows/build.yaml)
[![Clippy](https://github.com/falk-werner/note.rs/actions/workflows/clippy.yaml/badge.svg)](https://github.com/falk-werner/note.rs/actions/workflows/clippy.yaml)
[![Test](https://github.com/falk-werner/note.rs/actions/workflows/test.yaml/badge.svg)](https://github.com/falk-werner/note.rs/actions/workflows/test.yaml)

# note.rs

Yet another note taking tool.

## Build and run
This project is build with [rust](https://www.rust-lang.org/) and [npm](https://www.npmjs.com/).
Please see to have it installed beforehand.

Additionally, running `note.rs` will create a __GUI__ and it needs a display for this.
See to have a setup which enables this.

### Install dependencies
`note.rs` uses [tauri](https://tauri.app/) and [vite](https://vitejs.dev/).

How to setup __Ubuntu 20.04__ with tauri and vite:
1. Install tauri's dependencies
    ```bash
    sudo apt-get update
    sudo apt-get install -y \
        libwebkit2gtk-4.0-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev
    ```
2. Install tauri crates
    ```bash
    cargo install tauri-cli
    ```
3. Download npm packages including vite
    ```bash
    npm install
    ```

### Run in dev mode
```bash
cargo tauri dev
```

## Accelerators

| Key Binding | Description |
| ----------- | ----------- |
| Ctrl + n    | Create a new note. |
| Ctrl + d    | Delete the selected note. |
| Ctrl + s    | Sync changes *(note that changes are also saved automatically)*. |
| Ctrl + b    | Browse attachments of the selected note. |
| Ctrl + p    | Take screenshot. |
| Ctrl + q    | Save and exit *(note that Alt + F4 will exit **without** saving)*. |
| Ctrl + e    | Toggle between view mode and edit mode. |
| Ctrl + f    | Set focus of search *(note that Ctrl + f will open find menu while editing)*. |

## References

- [tauri](https://tauri.app/)
- [vite](https://vitejs.dev/)
- [npm](https://www.npmjs.com/)
- [codemirror](https://codemirror.net/)
- [marked](https://marked.js.org/)