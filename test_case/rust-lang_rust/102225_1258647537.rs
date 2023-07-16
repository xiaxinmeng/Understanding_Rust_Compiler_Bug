rust
pub struct Iter<'a, T: 'a> {
    ptr: NonNull<T>,
    end: *const T, // If T is a ZST, this is actually ptr+len.  This encoding is picked so that
    // ptr == end is a quick test for the Iterator being empty, that works
    // for both ZST and non-ZST.
    _marker: PhantomData<&'a T>,
}
