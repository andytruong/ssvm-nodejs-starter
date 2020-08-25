extern crate ferris_says;

use ferris_says::say as raw_say;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
pub fn say(s: &str) -> String {
    let mut vec = Vec::new();
    raw_say(s.as_bytes(), 24, &mut vec).unwrap();

    return String::from_utf8(vec).expect("Fount invalid UTF-8");
}
