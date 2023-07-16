rust
use std::any::Any;
use std::fmt::Debug;
use std::marker::PhantomData;

#[repr(packed)]
struct Foo(Vector3<f32>);

fn main() {
    // std::mem::size_of::<Foo>();
}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Matrix<N: Scalar, R: Dim, C: Dim, S> {
    pub data: S,
    _phantoms: PhantomData<(N, R, C)>,
}

type MatrixMN<N, R, C> = Matrix<N, R, C, Owned<N, R, C>>;
type VectorN<N, D> = MatrixMN<N, D, U1>;
type Vector3<N> = VectorN<N, U3>;

pub trait Dim: Any + Debug + Copy + PartialEq + Send + Sync {}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct U1;
impl Dim for U1 {}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct U3;
impl Dim for U3 {}

pub trait Scalar: Copy + PartialEq + Debug + Any {}
impl<T: Copy + PartialEq + Debug + Any> Scalar for T {}

pub unsafe trait Storage<N: Scalar, R: Dim, C: Dim = U1>: Debug + Sized {
    type RStride: Dim;
    type CStride: Dim;
}
pub unsafe trait StorageMut<N: Scalar, R: Dim, C: Dim = U1>: Storage<N, R, C> {}

pub unsafe trait ContiguousStorage<N: Scalar, R: Dim, C: Dim = U1>:
    Storage<N, R, C>
{
}

pub unsafe trait ContiguousStorageMut<N: Scalar, R: Dim, C: Dim = U1>:
    ContiguousStorage<N, R, C> + StorageMut<N, R, C>
{
}

pub trait Allocator<N: Scalar, R: Dim, C: Dim = U1>: Any + Sized {
    type Buffer: ContiguousStorageMut<N, R, C> + Clone;
}

pub struct DefaultAllocator;
pub type Owned<N, R, C = U1> = <DefaultAllocator as Allocator<N, R, C>>::Buffer;

