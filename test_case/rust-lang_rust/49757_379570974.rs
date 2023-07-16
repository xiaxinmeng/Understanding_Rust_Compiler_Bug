rust
#[doc(primitive = "never")]
#[doc(alias = "!")]
/// blah
mod prim_never { }

#[doc(primitive = "slice")]
#[doc(alias = "[]")]
mod prim_slice { }

#[doc(alias = "-")]
pub trait Neg { }

#[doc(alias = "-")]
pub trait Sub { }
