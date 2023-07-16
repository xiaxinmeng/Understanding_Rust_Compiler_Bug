rs
fn test<'a, T: Display>(x: T)
where
    &'a (): Sized, // any predicate containing `'a`
{
    let mut r: Option<Box<dyn Display + '_>> = None; // <- inferred short lifetime for '_
    let f = |x: T| {
        r = Some(lives_as_long::<'a, T>(x));
    };
}
