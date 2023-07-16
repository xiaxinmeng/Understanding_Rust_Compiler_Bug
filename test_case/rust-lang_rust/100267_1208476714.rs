rust
#![feature(type_alias_impl_trait)]

type Fut<'a> = impl Sized;

fn call<'cx, 's>() -> Fut<'cx>
where
    's: 'cx,
{
    || {}
}
