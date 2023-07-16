
trait A {
    fn foo();
}

trait B: A {
}

impl int: A {
    fn foo() {}
}

impl int: B {
}

fn main() {
    5.foo();
    (5 as B).foo();
}
