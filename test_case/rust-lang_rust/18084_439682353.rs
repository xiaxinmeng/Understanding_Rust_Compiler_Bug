rust
fn main() {
    use inner::A;
    mod inner {
        pub struct A;
    }
}
