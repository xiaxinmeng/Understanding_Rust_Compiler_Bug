 rust
mod foo {
    trait Bar {
        fn baz() {}
    }

    impl Bar for i32 {}
}

fn main() {
    <i32 as ::foo::Bar>::baz(); //~ERROR method `baz` is inaccessible
}
