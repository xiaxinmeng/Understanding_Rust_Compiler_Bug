rust
union PtrRepr<T: Pointee + ?Sized> {
    const_ptr: *const T,
    mut_ptr: *mut T,
    components: <T as Pointee>::Metadata
}

pub trait Pointee {
    type Metadata;
}
