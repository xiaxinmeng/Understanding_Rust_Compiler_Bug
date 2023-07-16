rust
fn main() {
    trait T {}
    impl<'a> T for u8 {}
}
