 rust
macro_rules! m {
    () => {
        struct S;
        impl S {
            fn f(&self) { // Since this binding was introduced by the macro,
                self; // it can be used here
            }
        }
    }
}
m!(); // This compiles on stable, beta, and nightly
