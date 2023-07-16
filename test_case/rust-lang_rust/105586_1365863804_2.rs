rust
#[no_mangle]
extern "C" fn nop<T>(val: T) -> T {
    val
}
