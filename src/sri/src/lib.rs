
use wasm_bindgen::{prelude::wasm_bindgen, JsValue};

#[wasm_bindgen]
pub fn sri() -> Result<String, JsValue> {

    let output = "Hello world";
    Ok(output.into())
}
