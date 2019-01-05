# 🦀🕸️ `wasm-game-of-life`

This project is Conway's game of life using web assembly with Rust and Node.JS

This project was created using the following tutorial:
https://rustwasm.github.io/book/game-of-life/introduction.html

## 🛠️ Setup

Dependencies: Node.JS and Rust toolchain

Install wasm-pack: https://rustwasm.github.io/wasm-pack/installer/

Update NPM: npm install npm@latest -g

## 🚴 Run the app via Rust web assembly and Node.js
```
cargo build
wasm-pack build
cd ./pkg
npm link
cd ../www
npm link wasm-game-of-life
npm run start
```
