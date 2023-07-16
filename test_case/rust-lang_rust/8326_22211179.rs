 rust
pub trait Times {
    fn times(self) -> SomeIteratorOverUnit;
}

impl Times for uint {
    fn times(self) -> SomeIteratorOverUnit {
        range(0, self).transform(|_| ())
    }
}
