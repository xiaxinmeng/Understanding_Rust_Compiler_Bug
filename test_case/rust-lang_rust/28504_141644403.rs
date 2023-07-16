 rust
pub use inner::Bar;

mod inner {
    trait Foo {
        fn foo(&self) { println!("foo"); }
    }
    pub trait Bar: Foo {
        fn bar(&self) { println!("bar"); }
    }
    impl Foo for i32 {}
    impl Bar for i32 {}
}

fn main() {
    //0i32.foo(); // Can't call this
    0i32.bar();
}
