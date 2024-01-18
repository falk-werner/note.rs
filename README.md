# note.rs

Yet another note taking tool.

## Run in dev mode

```bash
cargo tauri dev
```

## Setup environment
This guide focuses on how to setup your development system based on ubuntu:jammy.

1. Install dependencies
    ```bash
    sudo apt-get update
    sudo apt-get install -y \
        libwebkit2gtk-4.0-dev \
        build-essential \
        curl \
        wget \
        file \
        libssl-dev \
        libgtk-3-dev \
        libayatana-appindicator3-dev \
        librsvg2-dev
    ```
2. Install rust
    ```bash
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```
3. Install npm (node.js)
    ```bash
    curl -fsSL https://deb.nodesource.com/setup_20.x | sudo -E bash - &&\
    sudo apt-get install -y nodejs
    ```
4. Install tauri crates for development
    ```bash
    cargo install tauri-cli
    ```
5. Download npm packages
    ```bash
    npm install
    ```

## References

- [tauri](https://tauri.app/)
- [vite](https://vitejs.dev/)
- [npm](https://www.npmjs.com/)
- [codemirror](https://codemirror.net/)
- [marked](https://marked.js.org/)