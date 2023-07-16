 rust
trait A {
    fn as_A(&self) -> &A {self}
}
impl<T> A for T {}

trait B: A {
    fn as_B(&self) -> &B {self}
}
impl<T> B for T{}

fn main() {
    let i = 5;
    let a: &A = i.as_A();
    let b: &B = a.as_B();

}
