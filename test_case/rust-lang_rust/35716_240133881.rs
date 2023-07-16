 rust
fn foo() {
    unsafe {
        static FOO: Foo = /* is this considered to be in an unsafe block? */;

        fn bar() {
            /* is this considered to be in an unsafe block? */;
        }
    }
}

unsafe fn unsafe_foo() {
    static FOO: Foo = /* is this considered to be in an unsafe block? */;

    fn bar() {
        /* is this considered to be in an unsafe block? */;
    }
}
