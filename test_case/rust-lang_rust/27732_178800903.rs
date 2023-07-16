 rust
pub struct Allocated<'a, 'b, T: 'b + ?Sized> {
    reference: &'b mut T,
    phantom: PhantomData<&'a ()>
}

impl<'a, 'b, T: ?Sized, U: ?Sized> CoerceUnsized<Allocated<'a, 'b, U>> for Allocated<'a, 'b, T>
    where U: 'b, T: 'b + Unsize<U> {}
