rust
use std::borrow::Cow;

fn test<'a>()
where
    Cow<'a, u8>: Sized,
{
    let _: Option<Cow<'_, _>> = None;
    //^ the type is inferred to be Option<Cow<'_, u8>>
}
