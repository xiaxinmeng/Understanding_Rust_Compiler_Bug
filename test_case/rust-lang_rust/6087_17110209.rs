 rust
trait BaseVec<T> {
    fn from_value(value: T) -> Self;
}

struct Vec3<T> { x: T, y: T, z: T }

impl<T: Copy> BaseVec<T> for Vec3<T> {
    fn from_value(value: T) -> Vec3<T> { Vec3 { x: value, y: value, z: value } }
}

type Vec3f = Vec3<float>;

impl Vec3f {
    fn from_value(value: float) -> Vec3f { BaseVec::from_value(value) }
}

fn main() {
    let v = Vec3f::from_value(3.0);
}
