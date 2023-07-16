rust
trait Foo {
    type Error;
}

trait Bar {
    type Error;
}

trait Cake : Foo + Bar {}

struct S {
    f: Box<Cake<Error = i32>>,
}

fn main() {
}
