
pub trait Pointee {
   type Metadata;
}

// minimization
union PtrRepr<T: Pointee> {
    components: <T as Pointee>::Metadata,
}
