rust
use std::{convert::TryFrom, num::NonZeroI8};

pub fn into_nzi8<T>(x: T) -> Option<NonZeroI8>
where
    i8: TryFrom<T>,
{
    i8::try_from(x).ok().and_then(NonZeroI8::new)
}
