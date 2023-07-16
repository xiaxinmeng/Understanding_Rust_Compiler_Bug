
trait SomeBound {}

trait A {
    type Type;
}

trait B: A where Self::Type: SomeBound {
}

trait C: B {}

fn main() {}
