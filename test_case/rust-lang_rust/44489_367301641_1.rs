rust
#[panic_implementation]
fn my_panic_impl(pi: &PanicInfo) -> ! {
    abort_with_line(pi.location(),map(|l| l.line()));
}
