rs
fn test<'a, T: Display>(x: T)
where
    &'a (): Sized, // any predicate containing `'a`
{
    let mut r: Option<Box<dyn Display + 'a>> = None; // <- explicit lifetime 'a from the function signature
    let f = |x: T| {
        r = Some(lives_as_long::<'a, T>(x));
    };
}
