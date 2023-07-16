rust
unsafe fn unsf() {}

// This has some safety requirements...
unsafe fn foo() {
    // ...which might be used in the closure here
    let x: Foo = || unsf();
}
