rust
> pub struct IamSend<F: Future> {
>     f: F,
> }
> impl<F: Future> IamSend<F> {
>     pub unsafe fn new(f: F) -> Self {
>         IamSend { f }
>     }
> }
> unsafe impl<F: Future> Send for IamSend<F> {}
> impl<F: Future> Future for IamSend<F> {
>     type Output = F::Output;
>     fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output> {
>         unsafe { self.map_unchecked_mut(|s| &mut s.f).poll(cx) }
>     }
> }
> 