
#[allow(default_methods)];

trait A<T> {
    fn g(&self, x: T) -> T { x }
}

impl A<int> for int { }

fn f<T, V: A<T>>(i: V, j: T) -> T {
    i.g(j)
}

fn main () {
    assert!(f(0, 2) == 2);
}
