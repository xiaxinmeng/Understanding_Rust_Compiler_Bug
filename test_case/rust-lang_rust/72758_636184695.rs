rust
#[no_mangle]
pub extern "C" fn wasm_sin(key: i32) -> i32 {
    (key as f32).sin() as i32
}
