rust
trait Sample: Copy + Clone + PartialOrd + PartialEq {
    const EQUILIBRIUM: Self;
}

// This trait now has a const generic parameter.
trait Frame<const N: usize>: Copy + Clone + PartialEq {
    type Sample: Sample;

    const NUM_CHANNELS: usize;
    const EQUILIBRIUM: Self;

    fn to_array(self) -> [Self::Sample; N];
}

impl<S, const N: usize> Frame<N> for [S; N]
where
    S: Sample,
{
    type Sample = S;

    const NUM_CHANNELS: usize = N;
    const EQUILIBRIUM: Self = [S::EQUILIBRIUM; N];

    fn to_array(self) -> [Self::Sample; N] {
        self
    }
}

// Now, I have to include a `const N: usize` bound, even though this new method
// doesn't directly use its value. The value of `N` is only used to ensure that
// the two frames have the same number of channels. This kind of const generic
// pollution is especially bad when returning values parameterized by types that
// have const generics.
fn pair_test<A, B, F, const N: usize>(frame_a: A, frame_b: B, mut test_func: F) -> bool
where
    A: Frame<N>,
    B: Frame<N>, // Same constant number of channels.
    F: FnMut(&A::Sample, &B::Sample) -> bool,
{
    todo!("pretend this is a calculation that expects two arrays of the same size")
}
