
trait Test {
    type Assoc;
}
impl<T> Test for [T] {
    type Assoc = T;
}
fn test<T: Test<Assoc = u8>>(iter: &T) {}

fn main() {
    test(&[0i32][..])
}
