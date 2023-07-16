rust
#[panic_implementation]
fn my_panic_impl(pi: &PanicInfo) -> ! {
    abort()
}
