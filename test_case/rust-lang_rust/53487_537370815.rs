rust
let was_captured = match backtrace.status() {
    BacktraceStatus::Captured => true,
    _ => false,
};
