 rust
fn main() {
    let foo: &mut [u8] = &mut [1,2,3];
    foo as *mut [u8] as *mut u8;
}
