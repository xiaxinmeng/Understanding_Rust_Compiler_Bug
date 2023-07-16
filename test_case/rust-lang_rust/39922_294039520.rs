rust
struct S;
impl S {
    fn f(&self) {
        m!(); // If `self` were unhygienic, this would resolve
    }
}

macro m() {
    self // the resolution of `self` here would depend on where the macro is used (i.e. not hygienic)
}
