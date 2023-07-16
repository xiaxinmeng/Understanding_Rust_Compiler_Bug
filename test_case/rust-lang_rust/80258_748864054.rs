rust
struct Test {}

fn main() {
    let ptr: *mut Test = std::ptr::null_mut();
    let ptr_: *const Test = ptr;
}
