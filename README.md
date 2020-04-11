# Chip8 Emulator with Rust, WASM & Vue.js

## About

A very basic Chip8 Emulator written in Rust. The emulator is embedded in a webpage with Web Assembly (WASM). The webpage is built with Vue.js.

## 🚴 Usage

### Rust & Webassembly

Install wasm-pack from https://rustwasm.github.io/wasm-pack/installer/

Install dependencies:

```
cargo build
```

Compile code and generate WASM code

```
wasm-pack build
```

Test Rust code: 

```
cargo build
```

### Vue.js

From `web/`

```
yarn install
yarn serve
```


### 🔬 Test in Headless Browsers with `wasm-pack test`

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with `wasm-pack publish`

```
wasm-pack publish
```

## Inspiration

http://www.multigesture.net/articles/how-to-write-an-emulator-chip-8-interpreter/

https://github.com/ColinEberhardt/wasm-rust-chip8/blob/master/src/cpu.rs

http://devernay.free.fr/hacks/chip8/C8TECH10.HTM

http://emulator101.com/chip-8-instruction-set.html

https://github.com/learnopengles/chip8-rust

Opcode

https://github.com/trapexit/chip-8_documentation

https://rustwasm.github.io/book/

https://github.com/dmatlack/chip8/tree/master/roms

## Fonts
https://fonts.google.com/specimen/VT323?category=Monospace
https://www.fontspace.com/category/terminal

## Todo

- Add global scss styles
- Move all chip8 dependencies to store
- Style drag & drop for ROMs