rs
trait Dummy<'a> {}
impl<'a> Dummy<'a> for () {}

fn lives_as_long<'a, T>()
where
    T: 'a,
    (): Dummy<'a>, // <- added for every early-bound lifetime parameter on every function
{
}

fn test<'a, T>()
where
    &'a (): Sized, // any predicate containing `'a`
    (): Dummy<'a>, // <- added for every early-bound lifetime parameter on every function
{
    || {
        lives_as_long::<'_, T>(); // no longer 'a
    };
}
