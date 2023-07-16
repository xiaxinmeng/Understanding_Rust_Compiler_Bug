 rust
mod foo {
    mod bar {
        #[test] fn bar_test() {}
    }
    #[test] fn foo_test() {}
}
#[test] fn global_test() {}
