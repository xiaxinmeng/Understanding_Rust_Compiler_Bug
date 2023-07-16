
    pub struct A(Alpha);
}

crate b {
    pub use a::A as B;

    /// some useful fn that creates an A without exposing Alpha
    pub fn b() -> B {
        B(a::Alpha)
    }
}
