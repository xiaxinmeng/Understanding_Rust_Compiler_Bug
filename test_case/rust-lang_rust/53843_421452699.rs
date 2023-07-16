rust
use std::ops::Deref;

pub struct Pin<P>(P);

impl<P> Deref for Pin<P> where
    P: Deref,
{
    type Target = P::Target;
    fn deref(&self) -> &P::Target {
        &*self.0
    }
}

impl<'a, F> Pin<&'a mut F> {
    fn poll(self) {}
}

fn test(pin: Pin<&mut ()>) {
    pin.poll()
}
