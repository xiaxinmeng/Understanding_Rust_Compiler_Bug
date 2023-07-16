rust
#![feature(generic_associated_types)]
use std::marker::PhantomData;

trait AsyncFn<Arg> { type Output; }
trait RequestFamily<'a> { type Type; }
trait Service {}

struct MyFn;
impl AsyncFn<String> for MyFn { type Output = (); }

impl RequestFamily<'_> for String { type Type = String; }

struct ServiceFromAsyncFn<F, Req>(F, PhantomData<Req>);

impl<F, Req, O> Service for ServiceFromAsyncFn<F, Req>
where
    Req: for<'a> RequestFamily<'a>,
    F: AsyncFn<Req>,
    F: for<'a> AsyncFn<<Req as RequestFamily<'a>>::Type, Output = O>,
{
}

fn assert_service() -> impl Service {
    ServiceFromAsyncFn(MyFn, PhantomData)
}
