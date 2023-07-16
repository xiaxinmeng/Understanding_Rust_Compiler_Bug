 rust
trait Foo<T, E> { fn run(&self) -> Result<T, E>; }

impl Foo<int, Void> for PureIntCalculation {
    fn run(&self) -> Result<int, Void> { ... }
}

impl Foo<int, IoError> for ImpureIntCalc { ... }
