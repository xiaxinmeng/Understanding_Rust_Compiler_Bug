rust
#![allow(incomplete_features)]
// `struct ICE` below compiles well WITHOUT generic_const_exprs. But it does have an ICE with
// generic_const_exprs!
//
// The ICE is triggered even if the const generic-based struct itself (`ICE` below) is not used at
// all (not in any function signature, expression...). (And regardless of whether the type is
// public.)
#![feature(generic_const_exprs)]

trait Tr {
    const C: usize = 0;
}

struct Str {}
impl Tr for Str {}

struct ICE<const C: usize={Str::C}>
where [(); C]:,
{
    _arr: [(); C],
}
