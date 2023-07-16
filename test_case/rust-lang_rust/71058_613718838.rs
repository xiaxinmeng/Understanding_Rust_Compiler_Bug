rust
#![feature(optin_builtin_traits)]

pub auto trait MyUnpin {}
impl<'a, T: ?Sized + 'a> MyUnpin for &'a T {}

auto trait MySend {}

struct ImplementFuture<T>(T);
impl<T> core::future::Future for ImplementFuture<T> {
    type Output = T;
    fn poll(self: core::pin::Pin<&mut Self>, _: &mut core::task::Context) -> core::task::Poll<T> {
        unimplemented!()
    }
}


trait WithAssoc {
    type Assoc;
}
impl<T: MyUnpin> WithAssoc for T {
    type Assoc = ();
}
struct UseAssocOf<T: WithAssoc>(T::Assoc);


fn foo<'a>() -> Box<dyn MySend + 'a>
where
    &'a (): MySend + 'a,
{
    let x: ImplementFuture<UseAssocOf<&'a &'a ()>> = unimplemented!();
    #[allow(unreachable_code)]
    Box::new(async { x.await })
}

