rust
#[lang = "future_trait"]
trait Future {
    type Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Self::Output>;

    fn map(self) -> Self
        where Self: Sized
    {
        loop {}
    }
}
