
struct A { a: int }
impl A { fn foo(&self) {} }

struct B { b: int, A }
impl B { fn bar(&self) {} }

fn main() {
  let b = B { b: 3, A: { a: 3 } };
  b.a; // silently goes through `A`
  b.foo(); // again, goes through `A`
  b.A.a; // explicitly go through `A`
  b.A.foo();
  b.b;
  b.bar();
}
