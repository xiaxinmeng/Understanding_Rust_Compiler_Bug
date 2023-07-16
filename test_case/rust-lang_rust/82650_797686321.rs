rust
use std::marker::PhantomData;
use std::ops::{Add, Mul};

pub trait Allocator<N> {
    type Buffer;
}
pub struct DefaultAllocator;
impl<N> Allocator<N> for DefaultAllocator {
    type Buffer = ();
}

pub type Owned<N> = <DefaultAllocator as Allocator<N>>::Buffer;

#[derive(Copy, Clone)]
pub struct Matrix<N> {
    _phantom: PhantomData<N>,
}

pub type Vector4<N> = Matrix<Owned<N>>;

#[derive(Copy, Clone)]
pub struct Quaternion<N = ()> {
    pub coords: Vector4<N>,
}
impl<N> From<Vector4<N>> for Quaternion<N> {
    fn from(_coords: Vector4<N>) -> Self {
        unimplemented!()
    }
}
impl<N> Add for Quaternion<N>
where
    DefaultAllocator: Allocator<N>,
{
    type Output = Quaternion<N>;
    fn add(self, _rhs: Quaternion<N>) -> Self::Output {
        Quaternion::from(self.coords)
    }
}

pub struct DualQuaternion {
    pub real: Quaternion,
    pub dual: Quaternion,
}
impl<'a> Mul<()> for &'a DualQuaternion {
    type Output = ();
    fn mul(self, _rhs: ()) -> () {
        let _ = self.real + self.dual;
    }
}
