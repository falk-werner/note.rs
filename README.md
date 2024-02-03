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

## References

- [tauri](https://tauri.app/)
- [vite](https://vitejs.dev/)
- [npm](https://www.npmjs.com/)
- [codemirror](https://codemirror.net/)
- [marked](https://marked.js.org/)