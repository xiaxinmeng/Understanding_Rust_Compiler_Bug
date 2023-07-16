rust
#![deny(improper_ctypes_definitions, improper_ctypes)]
fn main() {
    let _: extern "C" fn([u8]) = todo!();
}
