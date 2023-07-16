rust
pub struct Generic<T: Clone>(T);
pub struct Normal(i32);

fn main () {
    let generic = Generic(10);
    let normal = Normal(10);
    generic;
    normal;
}
