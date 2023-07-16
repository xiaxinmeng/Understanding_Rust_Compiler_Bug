rs
fn lives_as_long<'a, T>() where T: 'a, {}

fn test<'a, T>()
where
    &'a (): Sized, // any predicate containing `'a`
{
    || {
        lives_as_long::<'a, T>();
    };
}
