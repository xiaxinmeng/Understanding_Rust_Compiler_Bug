rust
pub enum Adjust<'tcx> {
    /// Go from ! to any type.
    NeverToAny,

    /// Go from a fn-item type to a fn-pointer type.
    ReifyFnPointer,

    /// Go from a safe fn pointer to an unsafe fn pointer.
    UnsafeFnPointer,

    /// Go from a non-capturing closure to an fn pointer or an unsafe fn pointer.
    /// It cannot convert a closure that requires unsafe.
    ClosureFnPointer(hir::Unsafety),

    /// Go from a mut raw pointer to a const raw pointer.
    MutToConstPointer,

    /// Dereference once, producing a place.
    Deref(Option<OverloadedDeref<'tcx>>),

    /// Take the address and produce either a `&` or `*` pointer.
    Borrow(AutoBorrow<'tcx>),

    /// Unsize a pointer/reference value, e.g., `&[T; n]` to
    /// `&[T]`. Note that the source could be a thin or fat pointer.
    /// This will do things like convert thin pointers to fat
    /// pointers, or convert structs containing thin pointers to
    /// structs containing fat pointers, or convert between fat
    /// pointers. We don't store the details of how the transform is
    /// done (in fact, we don't know that, because it might depend on
    /// the precise type parameters). We just store the target
    /// type. Codegen backends and miri figure out what has to be done
    /// based on the precise source/target type at hand.
    Unsize,
}
