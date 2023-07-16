
#[allow(default_methods)];

trait A<T> {
    fn g(&self, x: T) -> T { x }
}

impl A<int> for int { }

fn main () {
    assert!(0i.g(2i) == 2i);
}
