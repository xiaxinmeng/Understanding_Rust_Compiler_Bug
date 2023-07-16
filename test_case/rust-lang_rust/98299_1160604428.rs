rust
use std::convert::TryFrom;

pub fn test_usage(p: ()) {
    SmallCString::try_from(p).map(|cstr| cstr);
}

pub struct SmallCString<const N: usize> {}

impl<const N: usize> TryFrom<()> for SmallCString<N> {
    type Error = ();

    fn try_from(path: ()) -> Result<Self, Self::Error> {
        unimplemented!();
    }
}
