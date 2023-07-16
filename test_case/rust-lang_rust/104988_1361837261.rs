rust
trait Foo { type Bar<T>; }

struct Tester;
impl Foo for Tester {
    type Bar<T> = Vec<T>;
}

struct Testy {
  rep: <Tester as Foo>::Bar<Testy>
}

