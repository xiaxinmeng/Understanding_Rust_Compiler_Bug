Rust
pub struct Foo;
fn m_use() {
    mod foo { // If we moved this module out of the macro, 
        fn f() {}
    }
    impl ::Foo {
        pub fn f() {}
    }
}
foo::f(); // this does not resolve
::Foo::f(); // but this does
