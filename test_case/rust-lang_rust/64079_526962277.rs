rust
struct Foo;
fn my_fn() {
   use self::Foo;
}

mod bar {
    fn other_fn() {
        use super::Foo;
    }
}
