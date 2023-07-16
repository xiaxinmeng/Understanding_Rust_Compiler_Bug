rust
struct WithLt<'lt> (
    &'lt (),
);

impl<'db> WithLt<'db> {
    fn check (_: impl FnOnce(*mut &'_ &'db ()))
    {}
}

fn _for<'db> (
    f: impl FnOnce(*mut &'_ &'db ()),
)
{
    WithLt::<'db>::check(move |s| f(s))
}
