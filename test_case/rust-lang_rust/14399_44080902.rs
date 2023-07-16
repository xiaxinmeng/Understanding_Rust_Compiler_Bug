 rust
impl Property for Vec3 {
    fn get_property(&self, _: &str) -> PropertyType { Float(0.0) }
}
impl Property for Quat {
    fn get_property(&self, _: &str) -> PropertyType { Struct(self.v.clone()) }
}
#[deriving(Clone)]
struct Vec3;
struct Quat {
    v: Box<Vec3>,
}
enum PropertyType { Float(f64), Struct(Box<Property>), }
trait Property {
    fn get_property(&self, &str) -> PropertyType;
}

fn main() { }
