 rust
trait Foo { fn foo(&self, x: &Self); }
trait Bar<A: Foo> { fn bar(&self) -> A; }
struct Wrap<T>(T);

fn test<A, B: Bar<A>>(wrap: Wrap<B>, x: &A) {
    wrap.0.bar().foo(x);
}

fn main() {}
