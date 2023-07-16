 rust
pub struct Foo;

mod bar {
    use Foo;

    impl Foo {
        fn baz(&self) {}
    }
}
fn main() {}
