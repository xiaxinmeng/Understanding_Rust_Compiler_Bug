 rust
pub fn foo() {}

mod bar {
    extern "Rust" {
        fn foo();
    }

    pub fn bar() {
        foo();
    }
}
