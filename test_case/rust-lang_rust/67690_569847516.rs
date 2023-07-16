rust
#![no_std]
mod a {
    pub trait A {}
}
pub trait B {}
pub trait C {}
pub type T<P: a::A + B + C> = P;
