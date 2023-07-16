rust
impl<'a> Trait1 for &'a str
where
    &'a str: Trait2 // <- the caller bounds
{
    fn foo(_: <&'a str as Trait2>::Assoc) {}
}
