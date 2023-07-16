rust

trait Sample: Copy + Clone + PartialOrd + PartialEq {
    const EQUILIBRIUM: Self;
}

trait Frame: Copy + Clone + PartialEq {
    type Sample: Sample;

    const NUM_CHANNELS: usize;
    const EQUILIBRIUM: Self;

    // Returns a fixed array of of `Self::Sample`s of size `Self::NUM_CHANNELS`.
    fn to_array(self) -> [Self::Sample; { Self::NUM_CHANNELS }];                                    // !!!!!!!!
}

// All fixed-size arrays of samples are frames.
impl<S, const N: usize> Frame for [S; N]
where
    S: Sample,
{
    type Sample = S;

    const NUM_CHANNELS: usize = N;
    const EQUILIBRIUM: Self = [S::EQUILIBRIUM; Self::NUM_CHANNELS];                                 // !!!!!!!!

    // Already an array, just return `self`.
    // I would hope [S; N] and [S; Self::NUM_CHANNELS] are realized to be the same type!
    fn to_array(self) -> [Self::Sample; { Self::NUM_CHANNELS }] {                                   // !!!!!!!!
        self
    }
}

// Standalone function to show example of const generic bound restrictions.
fn pair_test<A, B, F>(frame_a: A, frame_b: B, mut test_func: F) -> bool
where
    A: Frame,
    // Same constant number of channels.
    B: Frame<const NUM_CHANNELS = A::NUM_CHANNELS>,                                                 // !!!!!!!!
    F: FnMut(&A::Sample, &B::Sample) -> bool,
{
    todo!("pretend this is a calculation that expects two arrays of the same size")
}
