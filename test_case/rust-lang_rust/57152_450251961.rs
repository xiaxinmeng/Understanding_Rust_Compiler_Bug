
extern crate wasm_bindgen;
use wasm_bindgen::prelude::*;

#[derive(Debug)]
struct Test {
    a: bool,
    b: bool,
    c: bool,
    d: bool, // Removing "d" generates the correct code
}

impl Test {
    fn from_byte(byte: u8) -> Test {
        Test {
            a: (byte & 1) == 1,
            b: (byte & 2) == 2,
            c: (byte & 4) == 4,
            d: (byte & 8) == 8,
        }
    }
}


#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen]
pub fn run() {

    let list = vec![Test::from_byte(2)];

    log("&Iterator Start");
    for item in &list {
        // This gets printed as "true"
        log(&format!("{}", item.a))
    }
    log("&Iterator End");

    log("Into Iterator Start");
    for item in list {
        // This gets skipped completely
        log(&format!("{}", item.a))
    }
    log("Into Iterator End");

}
