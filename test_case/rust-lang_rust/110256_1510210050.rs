rust
fn foo(_b: &[u8]) {}

fn main() {
    let b = Vec::<u8>::new();
    let b = std::borrow::Cow::Borrowed(&b);
    foo(&b);
}
