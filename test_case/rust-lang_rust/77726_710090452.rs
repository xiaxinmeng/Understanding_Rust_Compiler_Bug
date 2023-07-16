rust
impl<T: ?Sized> Pin<Box<T>> {
    fn leak(this: Pin<Box<T>>) -> Pin<&'static mut T> { /* .. */ }
}

// equivalent expressions:
let x = Pin::leak(Box::pin(expr));
let x = Pin::static_mut(Box::leak(Box::new(expr)));
