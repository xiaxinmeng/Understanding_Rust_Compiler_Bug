
#![feature(conservative_impl_trait)]
#![allow(unconditional_recursion)] 
pub fn x() -> impl std::fmt::Display
{
    x()
}
