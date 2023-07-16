rust
#![feature(nll, type_ascription)]

#![allow(dead_code, unused_mut)]
type PairUncoupled<'a, 'b, T> = (&'a T, &'b T);
type PairCoupledRegions<'a, T> = (&'a T, &'a T);
type PairCoupledTypes<T> = (T, T);

fn uncoupled<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
    let ((mut y, mut _z),) = ((s, &_x),): (PairUncoupled<u32>,); // ok
    y
}

fn coupled_regions<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
    let ((mut y, mut _z),) = ((s, &_x),): (PairCoupledRegions<u32>,); //~ ERROR
    y
}

fn coupled_types<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
    let ((mut y, mut _z),) = ((s, &_x),): (PairCoupledTypes<&u32>,); //~ ERROR
    y
}

fn coupled_wilds<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
    let ((mut y, mut _z),) = ((s, &_x),): (PairCoupledTypes<_>,); //~ ERROR
    // ::std::mem::swap(&mut y, &mut _z);
    y
}

fn main() {
    uncoupled(&3, &4);
    coupled_regions(&3, &4);
    coupled_types(&3, &4);
    coupled_wilds(&3, &4);
}
