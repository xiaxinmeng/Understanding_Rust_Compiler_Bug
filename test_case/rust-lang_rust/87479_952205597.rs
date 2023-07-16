rust
pub trait FnOnceWithPinnedArg<Arg> {
    type Output;
    type Fut<'a>: Future<Output = Self::Output>
    where
        Arg: 'a;
    fn call<'a>(self, arg: Pin<&'a mut Arg>) -> Self::Fut<'a>;
}

impl<Arg, F, Output> FnOnceWithPinnedArg<Arg> for F
where
    F: for<'a> FnOnce<(Pin<&'a mut Arg>,), Output: Future<Output = Output>>,
{
    type Output = Output;
    type Fut<'a>
    where
        Arg: 'a,
    = <F as FnOnce<(Pin<&'a mut Arg>,)>>::Output;
    fn call<'a>(self, arg: Pin<&'a mut Arg>) -> Self::Fut<'a> {
        (self)(arg)
    }
}
