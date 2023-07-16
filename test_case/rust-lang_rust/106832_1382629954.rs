rust
pub trait Trait {
    type Assoc<A>;
}

fn test<T: Trait>()
where
    T::Assoc<u8>: Sized,
{
    constrain::<T, _>(1i32);
    // Registers `T::Assoc<_>: Sized`, which is eagerly satisfied by `T::Assoc<u8>: Sized`.
    // That means we infer the argument type to be u8.
}

fn constrain<T: Trait, A>(_: A)
where
    T::Assoc<A>: Sized,
{}
