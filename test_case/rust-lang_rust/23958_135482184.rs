 rust
trait Collection {
    fn my_iter<'a>(&'a self) -> <&'a Self as IntoIterator>::IntoIter where &'a Self: IntoIterator {
        self.into_iter()
    }
}

impl<T> Collection for [T] { }

fn main() {
    let v = [0usize];
    let _ = v.my_iter();
}
