rust
pub enum Void {}
#[no_mangle]
pub fn bar(v: Void) -> usize {
    v as usize
}
