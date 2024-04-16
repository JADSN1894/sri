use wasm_bindgen::prelude::*;

// #[wasm_bindgen(start)]
// pub fn entrypoint() -> Result<(), JsValue> {
//     Ok(())
// }

#[wasm_bindgen]
pub fn sri(input: String) -> String {
    String::from(input)
}
