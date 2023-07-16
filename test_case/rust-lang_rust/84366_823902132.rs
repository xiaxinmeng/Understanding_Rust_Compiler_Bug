rust
fn make_static_displayable<'a>(s: &'a str) -> Box<dyn fmt::Display> {
    //let f = || -> &'a str { "" };
    fn f<'a>() -> &'a str { "" }
    // problem is: the type of `f::<'a>` is 'static
    static_transfers_to_associated(&f::<'a>, s)
}
