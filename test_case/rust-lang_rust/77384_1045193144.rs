rust
fn extract_backtrace(error: &dyn Error) -> Option<&Backtrace> {
    error.backtrace()
}
