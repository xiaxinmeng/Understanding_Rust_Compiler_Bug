`rust
#[inline(always)]
fn err_exit(_: ()) -> ! {
    std::process::exit(1);
}
