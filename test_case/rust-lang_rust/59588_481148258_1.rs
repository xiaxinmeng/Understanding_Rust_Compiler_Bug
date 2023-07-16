rust
pub enum CastKind {
    Misc,

    /// Converts unique, zero-sized type for a fn to fn()
    ReifyFnPointer,

    /// Converts non capturing closure to fn() or unsafe fn().
    /// It cannot convert a closure that requires unsafe.
    ClosureFnPointer(hir::Unsafety),

    /// Converts safe fn() to unsafe fn()
    UnsafeFnPointer,

    /// Coerces *mut T to *const T, preserving T.
    MutToConstPointer,

    /// "Unsize" -- convert a thin-or-fat pointer to a fat pointer.
    /// codegen must figure out the details once full monomorphization
    /// is known. For example, this could be used to cast from a
    /// `&[i32;N]` to a `&[i32]`, or a `Box<T>` to a `Box<dyn Trait>`
    /// (presuming `T: Trait`).
    Unsize,
}
