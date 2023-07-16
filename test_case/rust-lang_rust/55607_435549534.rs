rust
    const unsafe fn foo() {}
    const unsafe fn foo() {
        bar(); // <-- ERROR! You must write `unsafe { bar(); }`.
    }
    