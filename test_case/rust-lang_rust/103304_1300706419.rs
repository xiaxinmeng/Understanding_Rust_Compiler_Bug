rust
#![feature(core_intrinsics)]
#[no_mangle]
extern "C" fn penguin() -> i64 {
    let dummy = ();
    core::intrinsics::black_box(dummy);
    0
}
