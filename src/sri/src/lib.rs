use std::{fs::File, io::Write};

use wasm_bindgen::prelude::*;

// #[wasm_bindgen(start)]
// pub fn entrypoint() -> Result<(), JsValue> {
//     Ok(())
// }

#[wasm_bindgen]
pub fn sri(input: String) -> String {
    let mut file = File::create("foo.txt").unwrap();
    file.write_all(b"Hello, world!").unwrap();
    String::from(input)
}
