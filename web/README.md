# Browser prototype

This is an initial browser-side bring-up target, not a playable port of Tears of the Kingdom.

## Included
- Static browser page suitable for Vercel.
- Rust wasm-bindgen bootstrap crate.
- WebGPU adapter and IndexedDB checks.
- Clearly-labelled placeholder canvas.
- Local file picker that only reads the selected file metadata. It does not upload, extract, or execute a game dump.

## Build the WASM bootstrap
Install Rust, wasm-pack, and the WASM target:

    rustup target add wasm32-unknown-unknown
    cargo install wasm-pack
    cd web/wasm
    wasm-pack build --target web --out-dir ../wasm

This generates the JavaScript glue and WASM binary imported by web/app.js.

## Run
After building WASM, serve the repository's web/ directory over HTTP(S). ES modules and WebAssembly should not be opened through file://.

## Limitations
The upstream runtime targets Linux x86-64 and uses Switch service HLE plus NVN/Vulkan. This bootstrap does not translate the AArch64 game, emulate Switch services, parse an XCI, implement NVN/WebGPU, or run game code. Those are substantial porting tasks. Do not add game files, keys, or copyrighted game assets to this repository.