rust
struct Qux;
struct Map<T, F>(T, F);

trait Foo {
    fn map<F>(self, f: F) -> Map<Self, F> where Self: Sized, Map<Self, F>: Foo {
        Map(self, f)
    }
}
trait Bar: Foo {
    fn bar(&self) {}
}

impl Foo for Qux {}
impl Bar for Qux {}

impl<T, F> Foo for Map<T, F> where F: FnOnce() {}
impl<T, F> Bar for Map<T, F> where F: FnMut() {}

fn main() {
    fn id<F>(f: F) -> F { f }
    
    Qux.map(id(||())).bar();
    // Qux.map(||()).bar(); // doesn't work
}
