rust
pub trait AAAA {}
pub trait B {}
pub trait C {}
pub type T<P: AAAA
+ B + C> = P;
