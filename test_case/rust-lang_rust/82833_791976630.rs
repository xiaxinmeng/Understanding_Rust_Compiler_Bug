rust
// compile-flags: -Cinline-threshold=0 -Clink-dead-code -g --crate-type=cdylib --emit=link
#[no_mangle]
#[inline(never)]
pub extern "C" fn banana() -> String {
    let mut bytes = [0u8; 4];
    '😃'.encode_utf8(&mut bytes).into()
}
