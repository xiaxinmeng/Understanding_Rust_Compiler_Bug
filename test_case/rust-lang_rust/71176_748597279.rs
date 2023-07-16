`rust
trait A {
    type B<T>;
}

trait C {
    type D: A<B = ()>;
}

fn main() {}
