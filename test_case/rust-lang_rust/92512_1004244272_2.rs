rust
#![feature(core_intrinsics, const_ptr_offset_from, const_ptr_offset)]

use core::intrinsics::*;

#[inline(always)]
pub(crate) const fn distance_between<T>(dest: *const T, origin: *const T) -> usize {
    // Safety: this function is used strictly with linear inputs where `dest` is known to come after
    // `origin`.
    match size_of::<T>() {
        // If `T` is a ZST, we cannot use typed pointers as it would either return an incorrect value
        // or in some cases segfault.
        0 => unsafe { ptr_offset_from(dest as *const u8, origin as *const u8) as usize },
        // For all other sizes of `T`, however typed pointers are just fine.
        _ => unsafe { ptr_offset_from(dest, origin) as usize },
    }
}

struct S;

const X: [S; 4] = [S, S, S, S];
const Y: *const S = X.as_ptr();
// The call to `distance_betweeen` will return 0 *unless* we set this pointer
// up as a u8. Also, const eval fails and aborts the compilation if regular
// `add` is used below instead of `wrapping_add`.
const Z: *const S = (Y as *const u8).wrapping_add(2) as *const S;
const W: usize = distance_between(Z, Y);

fn main() {
    // Prints "2".
    println!("{}", W);
}
