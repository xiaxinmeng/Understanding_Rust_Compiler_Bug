rust
#![feature(exhaustive_patterns)]
#![feature(never_type)]

pub trait Mode {
    type Known;
    type Unknown;
}

pub enum Value<T, M>
where
    M: Mode,
{
    Known(T, M::Known),
    Unknown(M::Unknown),
}

pub struct KnownOnly;

impl Mode for KnownOnly {
    type Known = ();
    type Unknown = !;
}

fn test(value: Value<bool, KnownOnly>) -> bool {
    let Value::Known(value, _) = value;
    value
}
