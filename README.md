# Tauri + Vue 3

This template should help get you started developing with Tauri + Vue 3 in Vite. The template uses Vue 3 `<script setup>` SFCs, check out the [script setup docs](https://v3.vuejs.org/api/sfc-script-setup.html#sfc-script-setup) to learn more.

## Recommended IDE Setup

- [VS Code](https://code.visualstudio.com/) + [Vue - Official](https://marketplace.visualstudio.com/items?itemName=Vue.volar) + [Tauri](https://marketplace.visualstudio.com/items?itemName=tauri-apps.tauri-vscode) + [rust-analyzer](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer)


### build requirements

```zsh
Python +3.11.2
NodeJS +v20.19.2
Rust 1.97.1
```


### Developer server

```zsh
npm install
npm run tauri dev

```

### Build production

```zsh
npm run tauri build

```


# Perfect structure

```zsh

my-app/
│
├── src/                    # Vue/Tauri frontend
│   ├── assets/
│   ├── components/
│   ├── locales/
│   ├── pages/
│   ├── plugins/
│   ├── router/
│   ├── stores/
│   ├── utils/
│   ├── App.vue
│   └── main.js
│
├── src-tauri/
│   ├── src/
│   │   └── main.rs         # Минимум Rust
│   ├── icons/
│   ├── tauri.conf.json
│   └── Cargo.toml
│
├── python/
│   ├── app.py              # Точка входа
│   │
│   ├── api/
│   │   ├── music.py
│   │   ├── settings.py
│   │   └── files.py
│   │
│   ├── services/
│   │   ├── player.py
│   │   ├── downloader.py
│   │   ├── metadata.py
│   │   └── audio.py
│   │
│   ├── models/
│   │   ├── song.py
│   │   └── playlist.py
│   │
│   ├── database/
│   │   ├── db.py
│   │   └── migrations/
│   │
│   ├── utils/
│   │   ├── logger.py
│   │   ├── config.py
│   │   └── helpers.py
│   │
│   ├── workers/
│   │   ├── scanner.py
│   │   └── converter.py
│   │
│   └── requirements.txt
│
├── resources/
│
├── scripts/
│
├── package.json
├── pyproject.toml
└── README.md

```