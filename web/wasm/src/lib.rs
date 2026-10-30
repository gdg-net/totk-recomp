use wasm_bindgen::prelude::*;
#[wasm_bindgen]
pub fn runtime_version() -> String { "bootstrap-0.1.0".to_owned() }
#[wasm_bindgen(start)]
pub fn start() {
    // Browser-side runtime initialization will be added here.
}