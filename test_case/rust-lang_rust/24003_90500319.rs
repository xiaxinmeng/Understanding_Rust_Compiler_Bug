 rust
macro_rules! format_template {
    ($tmp:tt) => (format!($tmp, a=123, b=456))
}
fn main() {
    format_template!("{a}{b}");
    format_template!("{a}");
    format_template!("{a}{b}");
    format_template!("{a}");
}
