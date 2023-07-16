rs
trait Dummy<'a> {}
impl<'a> Dummy<'a> for () {}

fn lives_as_long<'a, T>() where T: 'a, {}

fn test<'a, T>()
where
    &'a (): Sized, // any predicate containing `'a`
    (): Dummy<'a>, // <- this is new; added for every early-bound lifetime parameter
{
    || {
        lives_as_long::<'a, T>();
    };
}
