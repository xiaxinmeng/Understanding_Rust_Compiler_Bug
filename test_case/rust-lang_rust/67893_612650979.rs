rust
fn g(_: impl Send) {}

fn main() {
    g(my_crate::run())
}
