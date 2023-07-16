
fn main() {
    let id: &Mat2<float> = &Matrix::identity();
}

pub trait Index<Index,Result> { }
pub trait Dimensional<T>: Index<uint, T> { }

pub struct Mat2<T> { x: () }
pub struct Vec2<T> { x: () }

impl<T> Dimensional<Vec2<T>> for Mat2<T> { }
impl<T> Index<uint, Vec2<T>> for Mat2<T> { }

impl<T> Dimensional<T> for Vec2<T> { }
impl<T> Index<uint, T> for Vec2<T> { }

pub trait Matrix<T,V>: Dimensional<V> {
    fn identity() -> Self;
}

impl<T> Matrix<T, Vec2<T>> for Mat2<T> {
    fn identity() -> Mat2<T> { fail!() }
}
