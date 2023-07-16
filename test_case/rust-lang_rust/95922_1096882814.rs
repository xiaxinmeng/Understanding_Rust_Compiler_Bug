rust
#![feature(generic_associated_types)]
#![feature(type_alias_impl_trait)]
use core::ops::Deref;

trait Service3<Req> {
    type Output<'a> where Req: 'a;
    fn call(req: &mut Req) -> Self::Output<'_>;
}

struct WebReq<'s, S>(&'s S);

impl<'s, S> Service3<WebReq<'s, S>> for u8 {
    type Output<'a> = impl Deref where WebReq<'s, S>: 'a;
    fn call<'a>(req: &'a mut WebReq<'s, S>) -> Self::Output<'a> {
        req
    }
}

