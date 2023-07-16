rust
mod foo {
    fn f() {}
    m!(f); // Due to hygiene, this `f` reliably resolves to the above `f` ...
}

macro m($f:ident) {
    mod bar {
        fn g() { $f } // ... no matter where it ends up being used in the macro definition.
        // fn $f() {} // (unless the macro definition explicitly shadows it like this)
    }
}
