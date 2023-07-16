rust
struct WithLt<'lt> (
    &'lt (),
);

impl<'db> WithLt<'db> {
    fn check<'lt> (f: impl FnOnce(*mut &'_ &'lt ()))
    where
        'static : 'lt, // make it turbofishable
    {
        Self::check
            // ::<'lt> /* compilation passes if uncommented */
        (move |s| f(s))
    }
}
