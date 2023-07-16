rust
/// Prints a "backtrace" of some `Error`.
pub fn log_backtrace(e: &Error) {
    error!("Error: {}", e);

    for cause in e.iter().skip(1) {
        error!("\tCaused By: {}", cause);
    }
}
