rust
pub trait Trait<'p> {
    type Type;
}
impl<'p> Trait<'p> for str {
    type Type = &'p str;
}

pub trait Trait2 {
    fn foo<'p>(v: Self::Type)
    where
        Self: Trait<'p>;
}
impl Trait2 for str {
    fn foo<'p: 'p>(v: <Self as Trait<'p>>::Type)
    // where
    //     Self: Trait<'p>,
    {
        let _s: &str = unsafe { std::mem::transmute(v) };
    }
}
