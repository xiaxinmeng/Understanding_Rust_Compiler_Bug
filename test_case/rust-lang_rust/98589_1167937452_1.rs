rs
fn lives_as_long<'a, T>() where T: 'a, {}

fn test<'a, T>()
where
    &'a (): Sized, // any predicate containing `'a`
{
    struct Closure<'a, T>(…) where T: 'a;
    impl<'a, T> FnOnce<…> for Closure<'a, T> { … } // and also FnMut and Fn

    Closure::<'a, T>(…);
}
