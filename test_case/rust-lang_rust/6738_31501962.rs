 rust
struct Foo<T> {
    x: T,
}
impl<T> Foo<T> {
    fn add(&mut self, v: Foo<T>){
        self.x += v.x;
    }
}
fn main() {}
