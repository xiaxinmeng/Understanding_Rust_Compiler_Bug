
struct Foo<A> {
  x: Option<~Foo<(A, A)>>
}
fn main() { let y: Foo<int> = Foo { x: None }; }
