rust
#![feature(const_trait_impl)]
#![allow(incomplete_features)]

use std::num::{NonZeroU8, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroUsize, NonZeroU128};

trait HasNonZero {
    type NonZero;
    fn from_nonzero(v: Self::NonZero) -> Self;
}

macro_rules! has_nonzero_helper {
    ($T:ty, $NZT:ty) => {
        impl const HasNonZero for $T {
            type NonZero = $NZT;
            fn from_nonzero(v: Self::NonZero) -> Self { v.get() }
        }
    }
}

has_nonzero_helper!(u8, NonZeroU8);
has_nonzero_helper!(u16, NonZeroU16);
has_nonzero_helper!(u32, NonZeroU32);
has_nonzero_helper!(u64, NonZeroU64);
has_nonzero_helper!(u128, NonZeroU128);
has_nonzero_helper!(usize, NonZeroUsize);


// Performs a rem followed by a safe narrowing cast:
// (T1 % (T2 as T1)) as T2  where sizeof<T2> < sizeof<T1>
trait NarrowRem<Out> where Out: HasNonZero {
    fn narrowing_rem(&self, den: Out::NonZero) -> Out;
}

macro_rules! narrow_rem_helper {
    ($TSelf:ty, $TRem:ty) => {
        impl NarrowRem<$TRem> for $TSelf {
            #[inline(always)]
            fn narrowing_rem(&self, den: <$TRem as HasNonZero>::NonZero) -> $TRem {
                (*self % <Self as HasNonZero>::NonZero::from(den)) as _
            }
        }
    }
}

narrow_rem_helper!(u16, u8);
narrow_rem_helper!(u32, u8);
narrow_rem_helper!(u64, u8);
narrow_rem_helper!(u128, u8);
narrow_rem_helper!(usize, u8);
narrow_rem_helper!(u32, u16);
narrow_rem_helper!(u64, u16);
narrow_rem_helper!(u128, u16);
narrow_rem_helper!(usize, u16);
narrow_rem_helper!(u64, u32);
narrow_rem_helper!(u128, u32);
narrow_rem_helper!(u128, u64);

fn main() {}
