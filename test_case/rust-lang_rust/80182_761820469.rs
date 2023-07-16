
#[cfg(unix)]
fn print_stack_trace(_: libc::c_int) {
    let backtrace = std::backtrace::Backtrace::capture();
    println!("{}", backtrace);
}
