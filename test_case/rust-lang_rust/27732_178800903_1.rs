 rust
#[coerce_unsized(T)]
pub struct Allocated<'a, 'b, T: 'b + ?Sized> {
    reference: &'b mut T,
    phantom: PhantomData<&'a ()>
}
