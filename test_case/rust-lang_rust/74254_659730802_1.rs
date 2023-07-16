rust
#![feature(make_array)]
struct ArrayWithDefault<T, const N: usize>([T; N]);

impl<T: Default, const N: usize> Default for ArrayWithDefault {
    fn default() -> Self {
        Self(core::array::make(Default::default))
    }
}
