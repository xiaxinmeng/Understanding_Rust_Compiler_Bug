rust
fn test<'a>()
where
    'a, 'a, // <- makes sure `'a` is early-bound; late-bound lifetimes are already rejected
{
    let f = |_: &'a str| {};
    f(&String::new());
}
