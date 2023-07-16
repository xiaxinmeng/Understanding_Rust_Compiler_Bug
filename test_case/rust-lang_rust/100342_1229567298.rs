rust
fn main() {
    let _x: char = unsafe { std::mem::uninitialized() };
}
