rust
#![feature(negative_impls)]
#![feature(optin_builtin_traits)]
pub trait Trait {}
pub struct Special;

impl<T> Trait for T where private::Is<T>: private::NotSpecial {}
impl Trait for Special {}

mod private {
    pub auto trait NotSpecial {}
    pub struct Is<T>(T);
    impl !NotSpecial for Is<super::Special> {}   
}
