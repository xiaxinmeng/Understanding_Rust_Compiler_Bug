rust
#![feature(try_trait_v2)]
struct Flip;
impl<T> std::ops::FromResidual<Flip> for Option<T>
{
    fn from_residual(residual: Flip) -> Self {
        None
    }
}
