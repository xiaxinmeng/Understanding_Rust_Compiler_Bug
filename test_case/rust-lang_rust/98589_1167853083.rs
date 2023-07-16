rs
use std::fmt::Display;

fn lives_as_long<'a, T: Display>(x: T) -> Box<dyn Display + 'a>
where
    T: 'a,
{
    Box::new(x)
}

fn test<'a, T: Display>(x: T)
where
    &'a (): Sized, // any predicate containing `'a`
{
    let f = |x: T| -> Box<dyn Display + 'a> { lives_as_long::<'a, T>(x) };
    let r: Box<dyn Display + '_> = f(x); // <- inferred short lifetime for '_
}
