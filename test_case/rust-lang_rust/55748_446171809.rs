rust
#![feature(nll, type_ascription)]

#![allow(dead_code, unused_mut)]
type Pair<T> = (T, T);

fn static_to_a_to_static_through_tyvar<'a>(_x: &'a u32, s: &'static u32) -> &'static u32 {
    let local = 3;
    let ((mut y, mut _z),) = ((s, &local),): (Pair<&u32>,);

    // I should be able to add the call to `swap` below at whim based
    // on the above type annotation, which should coerce both `s` and
    // `x` to `&'1 u32` (and then `'1` should be inferred to be `'a`).

    // ::std::mem::swap(&mut y, &mut _z);

    // Likewise, the same rules that caused `y` and `_z` to have the
    // same `&'1 u32` type should likewise cause a borrow-check error
    // at this attempt to return a `&'static u32`.
    y
}

fn main() {
    static_to_a_to_static_through_tyvar(&3, &4);
}
