rust
use std::borrow::Borrow;

pub trait ConcatBase<Item: ?Sized, const N: usize> {
    type Output;

    fn my_concat(slice: &Self) -> Self::Output;
}

impl<T: Copy + Clone, V: Borrow<[T]>, const N: usize> ConcatBase<T, N> for [V] {
    type Output = [T; N];

    fn my_concat(slice: &Self) -> [T; N] {
        let slice_ref = slice[0].borrow();
        assert!(slice_ref.len() > 0, "Insufficient input!");
        let mut res = [slice_ref[0]; N];
        let mut i = 0;
        for v in slice {
            for el in v.borrow() {
                if i == N {
                    panic!("Insufficient output capacity!")
                }
                res[i] = *el;
                i += 1;
            }
        }
        res
    }
}

pub trait SliceConcatImpl {
    fn my_concat<Item: ?Sized, const N: usize>(&self) -> Self::Output
    where
        Self: ConcatBase<Item, N>,
    {
        ConcatBase::my_concat(self)
    }
}

impl<T> SliceConcatImpl for [T] {}

pub fn test(a: [u8; 32], b: [u8; 32]) -> [u8; 64] {
    [a, b].my_concat()
}
