
enum Foo<A> { Empty,
              Deep(~Foo<(A,A)>) }
fn main() { let y: Foo<int> = Empty; }
