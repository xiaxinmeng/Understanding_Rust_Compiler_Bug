rust
mod foo {
    fn f() {}
    mod bar {
        fn g() { f() } // `f` doesn't resolve here
    }
}
