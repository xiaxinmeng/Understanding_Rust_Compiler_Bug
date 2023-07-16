rust
#[unstable(feature = "print_internals",
           reason = "implementation detail which may disappear or be replaced at any time",
           issue = "0")]
#[doc(hidden)]
#[cfg(not(test))]
pub fn _eprint(args: fmt::Arguments) {
    print_to(args, &LOCAL_STDERR, stderr, "stderr");
}

/// Write `args` to output stream `local_s` if possible, `global_s`
/// otherwise. `label` identifies the stream in a panic message.
///
/// This function is used to print error messages, so it takes extra
/// care to avoid causing a panic when `local_stream` is unusable.
/// For instance, if the TLS key for the local stream is
/// already destroyed, or if the local stream is locked by another
/// thread, it will just fall back to the global stream.
///
/// However, if the actual I/O causes an error, this function does panic.
fn print_to<T>(
    args: fmt::Arguments,
    local_s: &'static LocalKey<RefCell<Option<Box<dyn Write+Send>>>>,
    global_s: fn() -> T,
    label: &str,
)
where
    T: Write,
{
    let result = local_s.try_with(|s| {
        if let Ok(mut borrowed) = s.try_borrow_mut() {
            if let Some(w) = borrowed.as_mut() {
                return w.write_fmt(args);
            }
        }
        global_s().write_fmt(args)
    }).unwrap_or_else(|_| {
        global_s().write_fmt(args)
    });

    if let Err(e) = result {
        panic!("failed printing to {}: {}", label, e);
    }
}
