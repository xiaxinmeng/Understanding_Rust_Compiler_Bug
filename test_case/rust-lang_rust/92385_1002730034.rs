rust
pub const fn ok(self) -> Option<T>
where
    E: ~const Drop,
{
    match self {
        Ok(x) => Some(x),
        Err(_x) => None,
    }
}

pub const fn err(self) -> Option<E>
where
    T: ~const Drop,
{
    match self {
        Ok(_x) => None,
        Err(x) => Some(x),
    }
}
