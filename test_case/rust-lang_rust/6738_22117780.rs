 rust
struct Vec3<T> {
    x: T,
    y: T,
    z: T
}
impl<T: Add<T,T>+Clone> Vec3<T> {
    fn add(&mut self,v: Vec3<T>){
        self.x += v.x.clone();
        self.y += v.y.clone();
        self.z += v.z.clone();
    }
}
fn main() {
    let mut v1 : Vec3<int> = Vec3{x: 1,y: 1,z: 1,};
    let v2 : Vec3<int> = Vec3{x: 1,y: 1,z: 1,};
    v1.add(v2);
}
