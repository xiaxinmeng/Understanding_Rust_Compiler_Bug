rust
#![feature(core_intrinsics, const_fn, const_panic)]

use core::intrinsics::assume;
use std::num::{NonZeroU8, NonZeroU16};


trait HasNonZero {
    type NonZero;
    fn from_nonzero(v: Self::NonZero) -> Self;
}
impl HasNonZero for u8 {
    type NonZero = NonZeroU8;
    fn from_nonzero(v: Self::NonZero) -> Self { v.get() }
}
impl HasNonZero for u16 {
    type NonZero = NonZeroU16;
    fn from_nonzero(v: Self::NonZero) -> Self { v.get() }
}
//...

//- - - - - - - - - - - - - - - - - - - -

// To be used at compile-time, it raises a panic. TODO until Option::unwrap becomes const.

const fn nonzero_u8(x: u8) -> NonZeroU8 {
    match x {
        0 => panic!("nonzero: x == 0"),
        _ => unsafe { NonZeroU8::new_unchecked(x) },
    }
}

const fn nonzero_u16(x: u16) -> NonZeroU16 {
    match x {
        0 => panic!("nonzero: x == 0"),
        _ => unsafe { NonZeroU16::new_unchecked(x) },
    }
}
//...

//- - - - - - - - - - - - - - - - - - - -

trait NarrowRem<Out> where Out: HasNonZero {
    fn narrowing_rem(&self, den: Out::NonZero) -> Out;
}
impl NarrowRem<u8> for u32 {
    #[inline(always)]
    fn narrowing_rem(&self, den: <u8 as HasNonZero>::NonZero) -> u8 {
        let divisor = Self::from(u8::from_nonzero(den));
        unsafe { assume(divisor != 0); } // TODO, will be unnecessary.
        (*self % divisor) as u8
    }
}
impl NarrowRem<u8> for usize {
    #[inline(always)]
    fn narrowing_rem(&self, den: <u8 as HasNonZero>::NonZero) -> u8 {
        let divisor = Self::from(u8::from_nonzero(den));
        unsafe { assume(divisor != 0); } // TODO, will be unnecessary.
        (*self % divisor) as u8
    }
}
//...
