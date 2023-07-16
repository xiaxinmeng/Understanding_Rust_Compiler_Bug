rust
trait Foo {
    type F;
}

impl<T> Foo for T {
    type F = T;
}

trait Bar: Foo<F = u32> {}
impl<T: Foo<F = u32>> Bar for T {}

fn main() {
    let y: i32 = 7;
    let z: Box<dyn Bar<F = i32>> = Box::new(y);
}
