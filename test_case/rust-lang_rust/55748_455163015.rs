rust
#![feature(nll)]

#![allow(dead_code, unused_mut)]
type PairUncoupled<'a, 'b, T> = (&'a T, &'b T);
type PairCoupledRegions<'a, T> = (&'a T, &'a T);
type PairCoupledTypes<T> = (T, T);

fn uncoupled_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
    let ((mut y, mut _z),): (PairUncoupled<u32>,) = ((s, &_x),); // ok
    // Above compiling does *not* imply below would compile.
    // ::std::mem::swap(&mut y, &mut _z);
    y
}

fn coupled_regions_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
    let ((mut y, mut _z),): (PairCoupledRegions<u32>,) = ((s, &_x),); //~ ERROR
    // Above compiling would imply below should compile.
    // ::std::mem::swap(&mut y, &mut _z);
    y
}

fn coupled_types_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
    let ((mut y, mut _z),): (PairCoupledTypes<&u32>,) = ((s, &_x),); //~ ERROR
    // Above compiling would imply below should compile.
    // ::std::mem::swap(&mut y, &mut _z);
    y
}

fn coupled_wilds_lhs<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
    let ((mut y, mut _z),): (PairCoupledTypes<_>,) = ((s, &_x),); //~ ERROR
    // Above compiling would imply below should compile.
    // ::std::mem::swap(&mut y, &mut _z);
    y
}

fn main() {
    uncoupled_lhs(&3, &4);
    coupled_regions_lhs(&3, &4);
    coupled_types_lhs(&3, &4);
    coupled_wilds_lhs(&3, &4);
}
