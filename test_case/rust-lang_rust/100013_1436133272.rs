
trait Tr {
        type A<'a, 'b: 'a>;
}

struct S<T> {
        a: T,
}

// bad
impl<T: Tr> S<T>
where
        for<'a, 'b> T: Tr<A<'a, 'b> = ()>,
        for<'c, 'd> T::A<'c, 'd>: Default,
{
}
