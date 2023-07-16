rust
#![feature(
  const_eval_select,
  core_intrinsics,
  const_ptr_offset,
  const_ptr_offset_from,
  inline_const
)]

use core::hint::unreachable_unchecked;
use core::intrinsics::*;

#[inline(always)]
pub(crate) fn runtime_zst_handler<T>(dest: *const T, origin: *const T) -> usize {
  (dest as usize).wrapping_sub(origin as usize)
}

#[inline(always)]
pub(crate) const fn compiletime_zst_handler<T>(_dest: *const T, _origin: *const T) -> usize {
  match size_of::<T>() {
    0 => panic!("`distance_between` is only currently usefully `const` for non-ZSTs!"),
    _ => unsafe { unreachable_unchecked() },
  }
}

#[inline(always)]
pub(crate) const fn distance_between<T>(dest: *const T, origin: *const T) -> usize {
  // Safety: this function is used strictly with linear inputs where `dest` is known to come after
  // `origin`.
  match size_of::<T>() {
    0 => unsafe {
      const_eval_select((dest, origin), compiletime_zst_handler, runtime_zst_handler)
    },
    _ => unsafe { ptr_offset_from(dest, origin) as usize },
  }
}

struct S;

const X: [S; 4] = [S, S, S, S];
const Y: *const S = X.as_ptr();
const Z: *const S = (Y as *const u8).wrapping_add(2) as *const S;

// The next line does the panic at compile time if uncommented.
// const W: usize = distance_between(Z, Y);

pub fn main() {
  // This one works properly.
  println!("{}", distance_between(Z, Y));
}
