 rust
pub mod strided {
    use raw;

    pub struct Slice<'a, T>(pub raw::Slice<'a, T>);
    impl<'a, T> Copy for Slice<'a, T> {}
    pub struct SliceMut<'a, T>(pub raw::SliceMut<'a, T>);
}

pub mod raw {
    use std::marker::PhantomData;

    pub struct Slice<'a, T> {
        // represents `index`
        _marker: PhantomData<fn() -> &'a T>,

        data: *const T,
        len: usize,
    }

    impl<'a, T> Copy for Slice<'a, T> {}

    pub struct SliceMut<'a, T> {
        // represents `index_mut`
        _marker: PhantomData<fn() -> &'a mut T>,

        data: *const T,
        len: usize,
    }
}

pub struct MutRow<'a, T>(strided::SliceMut<'a, T>);
pub struct Row<'a, T>(strided::Slice<'a, T>);
impl<'a, T> Copy for Row<'a, T> {}
pub struct Scaled<T, M>(T, M);

impl<'a, 'b, T> AddAssign<Scaled<T, Row<'a, T>>> for MutRow<'b, T> {
    fn add_assign(&mut self, rhs: &Scaled<T, Row<T>>) {
        let MutRow(strided::SliceMut(ref mut lhs)) = *self;
        let Scaled(ref alpha, Row(strided::Slice(rhs))) = *rhs;

        axpy(lhs, alpha, rhs)
    }
}

fn axpy<T>(_: &mut raw::SliceMut<T>, _: &T, _: raw::Slice<T>) {}

pub trait AddAssign<Rhs> {
    fn add_assign(&mut self, &Rhs);
}

fn main() {}
