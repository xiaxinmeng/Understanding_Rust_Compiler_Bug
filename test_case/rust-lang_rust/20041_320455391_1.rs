
#[derive(Clone, Copy, Debug)]
pub struct Range<T: RangeImpl> {
    inner: T,
}

pub fn range<X: SampleRange, R: Rng+?Sized>(low: X, high: X, rng: &mut R) -> X {
    assert!(low < high, "distributions::range called with low >= high");
    Range { inner: X::T::new(low, high) }.sample(rng)
}
