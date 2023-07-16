rs
#![feature(const_generics)]
#![feature(const_evaluatable_checked)]

pub struct S<T: Copy + Default, const N: usize>
where
    [T; N*2]: Sized,
{
    pub s: [T; N * 2],
}

impl<T: Default + Copy, const N: usize> S<T, N>
where
    [T; N * 2]: Sized,
{
    pub fn test() -> Self {
        S {
            s: [T::default(); N * 2],
        }
    }
}
