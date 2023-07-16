 rust
macro_rules! outer {
    ($b:tt) => {
        macro_rules! inner {
            ($b) => { }
        }
    }
}

outer!($a); // OK
outer!($b); // hangs the compiler
