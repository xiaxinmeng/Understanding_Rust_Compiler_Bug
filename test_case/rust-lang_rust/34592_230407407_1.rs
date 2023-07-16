 Rust
extern crate test;

fn main() {
    assert!(test::decode::<()>().is_err());
}
