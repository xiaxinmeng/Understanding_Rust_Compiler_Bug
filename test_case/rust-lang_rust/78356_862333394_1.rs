rust
#[inline]
pub fn log_fmt(x: fmt::Arguments) {
    if let Some(s) = x.as_str() {
        log_str(s); // No allocation needed. The whole str is already available. \o/
    } else {
        log_str(&x.to_string());
    }
}
