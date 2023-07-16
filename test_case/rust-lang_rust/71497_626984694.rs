Rust
mod foo1 {
    #[link(kind="raw_dylib", name="foo1.dll")]
    extern {
        fn foo() -> i32;
    }
}
mod foo2 {
    #[link(kind="raw_dylib", name="foo2.dll")]
    extern {
        fn foo() -> i32;
    }
}
fn main() {
    unsafe {
        assert_eq!(foo1::foo(), 100);
        assert_eq!(foo2::foo(), 200);
    }
}
