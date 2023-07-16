 rust
#![feature(specialization)]

trait Convert {
    type Target;
    fn convert(self) -> Self::Target;
}

impl<T> Convert for T {
    default type Target = T;
    default fn convert(self) -> Self::Target { self }
}

fn main() {}
