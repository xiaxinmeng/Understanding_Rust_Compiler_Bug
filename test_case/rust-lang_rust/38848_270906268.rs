
// Cast to `usize` and compare
fn main() {
    let a: extern "fastcall" fn() = panic!();
    let b: extern "fastcall" fn() = panic!();
    a as usize == b as usize;
}
