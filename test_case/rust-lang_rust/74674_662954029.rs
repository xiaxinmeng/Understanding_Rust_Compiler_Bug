Rust
use core::mem::size_of;

trait SplitCheck<const N2: usize, const N3: usize> {
    type LHS;
    type RHS;

    const ASSERT: bool;
    const __ASSERT: () = [()][(!Self::ASSERT) as usize];
}

impl<T, const N: usize, const N2: usize, const N3: usize> SplitCheck<{ N2 }, { N3 }> for [T; N] {
    type LHS = [T; N2];
    type RHS = [T; N3];

    const ASSERT: bool = size_of::<Self::LHS>() + size_of::<Self::RHS>() == size_of::<Self>();
}

impl<T, const N: usize> [T; N] {
    fn split_array<const N2: usize, const N3: usize>(&self) -> (&[T; N2], &[T; N3]) {
        let _: () = <Self as SplitCheck<{ N2 }, { N3 }>>::__ASSERT;

        let (lhs, rhs) = self.split_at(N2);

        unsafe {
            let lhs_array = &(*lhs.as_ptr().cast());
            let rhs_array = &(*rhs.as_ptr().cast());

            (lhs_array, rhs_array)
        }
    }
}
