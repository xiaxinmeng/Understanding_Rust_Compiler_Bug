rust
trait A {
  type C;
}

trait B: A {
  type C;
}

struct S(Box<dyn B<C=()>>);
