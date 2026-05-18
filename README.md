# CHIP-8 Emulator

This repository provides an implementation of CHIP-8 Emulator written in Rust.
The core CHIP-8 emulation core can found in the `chip8_core` sub-crate. It provides the
core emulation logic as library. An example Emulator with SDL2 based UI can be found
in the `chip8` sub-crate.

## WebAssembly Builds

A WebAssembly variant that relies on wasm_bindgen can be found
in `chip8_wasm` and can be build using wasm-pack (`cargo install wasm-pack`).

For running the WebAssembly variant first build the wasm binary:

```
cd chip8_wasm
wasm-pack build --target web
```

And then copy the `chip8_wasm_bg.wasm` and `chip8_wasm.js` from `chip8_wasm/pkg` into the `web` folder.
You can then run e.g. `python -m http.server` from within the `web` folder to serve the application, which can
then be accessed from the browser under `http://localhost:8000`.

## Credits

This code is based heavily on the [CHIP-8 tutorial book](https://github.com/aquova/chip8-book) and mainly adds a couple
of high level types for CPU components and decouples the opcode decoding from the opcode execution using the `OpCode`
enumeration. You can find ROMs for the emulator in CHIP-8 tutorial book repository.
