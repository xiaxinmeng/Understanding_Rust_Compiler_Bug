rust
type F<'a> = fn(&'a ());
macro_rules! output {
    ($t: ty) => {
        struct S<'a> {
            r: &'a VARIABLE,
            o: Box<F<'a>>,
        }

        struct SAsyncSendTryBuilder<
            'a,
            RBuilder_: for<'this> ::core::ops::FnOnce(
                &'this Box<F<'a>>,
            ) -> ::core::pin::Pin<
                Box<
                    dyn ::core::future::Future<
                            Output = ::core::result::Result<&'this VARIABLE, Error_>,
                        > + ::core::marker::Send
                        + 'this,
                >,
            >,
            Error_,
        > {
            o: Box<F<'a>>,
            r_builder: RBuilder_,
        }
        impl<
                'a,
                RBuilder_: for<'this> ::core::ops::FnOnce(
                    &'this Box<F<'a>>,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                                Output = ::core::result::Result<&'this VARIABLE, Error_>,
                            > + ::core::marker::Send
                            + 'this,
                    >,
                >,
                Error_,
            > SAsyncSendTryBuilder<'a, RBuilder_, Error_>
        {
            async fn try_build_or_recover(
                self,
            ) -> ::core::result::Result<S<'a>, (Error_, Heads<'a>)> {
                S::try_new_or_recover_async_send(self.o, self.r_builder).await
            }
        }
        struct Heads<'a> {
            o: Box<F<'a>>,
            _consume_template_lifetime_a: ::core::marker::PhantomData<&'a ()>,
        }
        impl<'a> S<'a> {
            async fn try_new_or_recover_async_send<Error_>(
                o: Box<F<'a>>,
                r_builder: impl for<'this> ::core::ops::FnOnce(
                    &'this Box<F<'a>>,
                ) -> ::core::pin::Pin<
                    Box<
                        dyn ::core::future::Future<
                                Output = ::core::result::Result<&'this VARIABLE, Error_>,
                            > + ::core::marker::Send
                            + 'this,
                    >,
                >,
            ) -> ::core::result::Result<S<'a>, (Error_, Heads<'a>)> {
                todo!()
            }
        }
    };
}

output!(str);
fn main() {}
