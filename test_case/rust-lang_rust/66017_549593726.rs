rust
trait IntoIteratorExt: IntoIterator + Sized {
    fn into_iter_first(self) -> Option<Self::Item> {
        self.into_iter().next()
    }
}

impl<T: IntoIterator> IntoIteratorExt for T {}

fn foo(value: Option<&i32>) {
    println!("{}", value.unwrap());
}

fn main() {
    foo([1,2,3].into_iter_first());
}
