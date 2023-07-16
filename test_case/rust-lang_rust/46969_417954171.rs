rust
use std::marker::PhantomData;

pub trait FromPest: Sized {
    type Rule;
    const RULE: Self::Rule;
}

impl<T> FromPest for PhantomData<T>
where
    T: FromPest,
{
    type Rule = T::Rule;
    const RULE: T::Rule = T::RULE;
}
