rust
// Or `Dyn: Pointee + ?Sized`, but we'll tolerant non `dyn` unsized types. 
pub struct InlinedDyn<Dyn: Pointee<Metadata = ptr::DynMetadata<Dyn>> + ?Sized> {
    data: usize, // Or anything if you like, or even with a const generic size.
    metadata: ptr::DynMetadata<Dyn>,
    _marker: PhantomData<Dyn>,
}
impl<Dyn: Pointee<Metadata = ptr::DynMetadata<Dyn>> + ?Sized> InlinedDyn<Dyn> {
    pub fn new<T: Unsize<Dyn>>(value: T) -> Self;
}
impl<Dyn: Pointee<Metadata = ptr::DynMetadata<Dyn>> + ?Sized> ops::Deref for InlinedDyn<Dyn> {
    type Target = Dyn;
}
impl<Dyn: Pointee<Metadata = ptr::DynMetadata<Dyn>> + ?Sized> ops::DerefMut for InlinedDyn<Dyn> {}
