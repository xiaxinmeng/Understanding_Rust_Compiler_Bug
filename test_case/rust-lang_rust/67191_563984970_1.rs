rust
const FOO: ! = panic!();
fn main() {
    let _ = FOO;
    unsafe {
        *(5 as *mut i32) = 49;
    }
}
