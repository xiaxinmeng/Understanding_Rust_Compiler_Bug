rust
if cfg!(feature = "panic_immediate_abort") {
    unsafe { ::intrinsics::abort() }
}
