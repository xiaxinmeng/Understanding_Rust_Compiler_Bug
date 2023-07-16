rust
#![feature(type_alias_impl_trait)]
#![feature(generic_associated_types)]

trait Service<Req> {
    type Output<'x> where Req: 'x;
    fn call<'y>(req: &'y Req) -> Self::Output<'y>;
}

impl<Req> Service<Req> for u8 {
    type Output<'b> = impl Copy where Req: 'b;
    fn call<'c>(req: &'c Req) -> Self::Output<'c> {
        req
    }
}
