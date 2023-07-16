rust
// run-pass
#![feature(const_generics)]
#![feature(lazy_normalization_consts)]
#![allow(incomplete_features)]

// try to devise a test for higher-ranked case
//
// e.g., for<'a> EqZero<<() as Zero<'a>::C>>
//
//

trait EqZero<const C: usize> { }

impl EqZero<0> for () { }

trait Zero<'a> {
    const C: usize;
}

impl<'a> Zero<'a> for () {
    const C: usize = 0;
}

fn test_me<T>()
where for<'a> T: EqZero<{<() as Zero<'a>>::C}>
{
}

fn main() {
    test_me::<()>();
}
