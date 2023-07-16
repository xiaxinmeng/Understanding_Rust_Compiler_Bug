rust
use std::borrow::Cow;

fn test<'a>()
where
    Cow<'a, u8>: Sized,
{
    let _: Option<Cow<'static, u8>> = None;
    //~^ ERROR lifetime may not live long enough
    // requires 'a == 'static
}
