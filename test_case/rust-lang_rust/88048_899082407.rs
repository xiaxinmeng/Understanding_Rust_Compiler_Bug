rust
pub trait Kind where for<'a> &'a Self::Values: IntoIterator<Item=&'a Self> {
    type Values: IntoIterator<Item=Self>;
}

pub trait OverridableKind: Kind {
}
