rust
#![feature(type_alias_impl_trait)]

#[pin_project::pin_project]
pub struct Send {
    #[pin]
    __: tait::Send,
}

mod tait {
    pub(crate) type Send = impl ::core::future::Future<Output = ()>;

    pub(crate) fn _def() -> Send {
        async {}
    }
}
