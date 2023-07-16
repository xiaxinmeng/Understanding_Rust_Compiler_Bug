rust
#![feature(specialization)]
#![feature(never_type)]

trait Unto<T> {
    fn unto(self) -> T;
}

impl<T> Unto<T> for T {
    default fn unto(self) -> T {
        self
    }
}

impl<T> Unto<T> for ! {
    default fn unto(self) -> T {
        self
    }
}

impl Unto<!> for ! {
    fn unto(self) -> ! {
        self
    }
}
