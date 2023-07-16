rust
pub
unsafe
auto trait TransitivelyImmutableWhenAliased {}

impl<T : ?Sized> !TransitivelyImmutableWhenAliased for UnsafeCell<T> {}

// Same as with `Send` and `Sync`, we conservatively drop the trait when raw pointers are involved,
// in case the pointers are type-erase
// (_e.g._, a `*const ()` representing a `*const UnsafeCell<Something>`)
impl<T : ?Sized> !TransitivelyImmutableWhenAliased for *const T {}
impl<T : ?Sized> !TransitivelyImmutableWhenAliased for *mut T {}

unsafe impl<T : ?Sized> TransitivelyImmutableWhenAliased for Box<T> where
    T : TransitivelyImmutableWhenAliased,
{}

impl<T : ?Sized> Cell<T> {
    fn with<R, F> (self: &'_ Self, f: for<'any> fn(&'any T) -> R) -> R
    where
        T : TransitivelyImmutableWhenAliased,
