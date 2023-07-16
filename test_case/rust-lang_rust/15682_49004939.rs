 rust
macro_rules! self_call(
    ($self_:ident, $name:ident) => ($self_.$name())
)

// ...

    fn x(&self) {
        self_call!(self, y)
    }
