 rust
macro_rules! method {
    () => { fn f() {} }
}

struct S;
impl S {
    #[allow_internal_unstable]
    method! {}
}
