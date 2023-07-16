rust
macro_rules! m {
    ($stmt:stmt) => { #[allow(bad_style)] $stmt }
}

fn main() {
    m!(;);
}
