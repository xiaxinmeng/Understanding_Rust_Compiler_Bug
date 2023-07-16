 Rust
#[feature(macro_rules)];
fn main() {
    assert!({ macro_rules! bar( () => (5)) });
}
