rust
struct Foo<T>
where
    for<'a> T: PartialEq<&'a T>,
{
    x: T,
}

struct Bar<T>
where
    T: for<'a> PartialEq<&'a T>,
{
    x: T,
}
