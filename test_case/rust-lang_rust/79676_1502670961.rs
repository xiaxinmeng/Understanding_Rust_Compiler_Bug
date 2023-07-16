
/// Split backtrace into frames, remove frames from Rust internals.
/// Otherwise, backtraces are mostly the panic system's internals and won't fit.
/// This is very dependent on the format of backtrace output.
/// It looks for the number: at the beginning of each frame.
///
/// This is a workaround for https://github.com/rust-lang/rust/issues/79676
fn trim_backtrace(s: &str) -> Vec<&str> {
    let splitter = Regex::new(r"(^|\n)\s+\d+:").unwrap();
    splitter.split(s).filter(|frame| !frame.contains("/rustc/") && !frame.is_empty()).collect()   // split frames
}

