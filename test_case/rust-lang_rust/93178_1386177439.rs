rust
use std::future::Future;
use std::ops::Deref;
use std::pin::Pin;
use std::task::{Context, Poll};

pub struct Pinner<'a, T> {
    pub unsafe_pointer: &'a mut T,
}

impl<'a, T> Pinner<'a, T> {
    pub fn as_mut(&mut self) -> Pin<&mut T> {
        // SAFETY: as long as Pinner is only ever constructed via the pin!()
        // macro and the unsafe_pointer field is never directly accessed,
        // then the value is safe to pin here. this is because the macro
        // ensures the input is turned into a borrowed anonymous temporary,
        // preventing any further access to the original value after Pinner
        // is constructed, and Pinner has no methods that enable moving out
        // of the reference. the word "unsafe" is used in the field name to
        // discourage direct access. this is the best we can do, since the
        // field must be public for the macro to work.
        unsafe { Pin::new_unchecked(self.unsafe_pointer) }
    }

    pub fn set(&mut self, value: T) {
        self.as_mut().set(value)
    }
}

impl<'a, T> Pinner<'a, Option<T>> {
    pub fn as_pin_mut(&mut self) -> Option<Pin<&mut T>> {
        self.as_mut().as_pin_mut()
    }
}

impl<'a, T> Deref for Pinner<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.unsafe_pointer
    }
}

impl<'a, T> Future for Pinner<'a, T>
where
    T: Future,
{
    type Output = T::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        T::poll(Pin::into_inner(self).as_mut(), cx)
    }
}

#[macro_export]
macro_rules! pin {
    ($x:expr) => {
        crate::Pinner {
            unsafe_pointer: &mut { $x },
        }
    };
}
