rust
use ::core::cell::Cell;

#[derive(Clone, Copy)]
#[repr(C)]
struct T(bool, u8, bool);

let ref mut s: [u8; 5] = [1, 2, 0, 1, 0];
let p = s.as_mut_ptr(); // SB: shared read-write over those 5 bytes (`&[Cell<u8>; 5]`)
let a: &Cell< T > = unsafe { &*p.cast() };
let b: &Cell< T > = unsafe { &*p.add(2).cast() };
// `a.2` overlaps with `b.0`
Cell::swap(a, b);
// Assuming the non-overlapping parts swap, the original 5-byte layout now is:
// `[1, 0, 0, 1, 2]`
b.get().2 // invalid `bool`
