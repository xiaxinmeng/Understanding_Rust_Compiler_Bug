rust
pub trait Identity {
    type Identity;
}

impl<T> Identity for T {
    type Identity = Self;
}

pub type Foo = u8;

pub union Bar {
    pub a: <Foo as Identity>::Identity,
    pub b: u8,
}
