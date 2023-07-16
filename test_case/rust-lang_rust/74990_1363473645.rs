rust
#[no_mangle]
extern "C-unwind" fn example() {
    panic!("Uh oh");
}
