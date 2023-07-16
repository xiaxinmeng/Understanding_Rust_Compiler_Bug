rust
/// When adjusting a receiver we often want to do one of
///
/// - Add a `&` (or `&mut`), converting the recevier from `T` to `&T` (or `&mut T`)
/// - If the receiver has type `*mut T`, convert it to `*const T`
///
/// This type tells us which one to do.
///
/// Note that in principle we could do both at the same time. For example, when the receiver has
/// type `T`, we could autoref it to `&T`, then convert to `*const T`. Or, when it has type `*mut
/// T`, we could convert it to `*const T`, then autoref to `&*const T`. However, currently we do
/// (at most) one of these. Either the type has `T` and we convert it to `&T` (or with `mut`), or
/// it has type `*mut T` and we convert it to `*const T`.
#[derive(Debug, PartialEq, Clone)]
pub enum AutorefOrPtrAdjustment<'tcx> {
    /// Receiver has type `T`, add `&` or `&mut` (it `T` is `mut`), and maybe also "unsize" it.
    /// Unsizing is used to convert a `[T; N]` to `[T]`, which only makes sense when autorefing.
    Autoref {
        mutbl: hir::Mutability,

        /// Indicates that the source expression should be "unsized" to a target type. This should
        /// probably eventually go away in favor of just coercing method receivers.
        unsize: Option<Ty<'tcx>>,
    },
    /// Receiver has type `*mut T`, convert to `*const T`
    ToConstPtr,
}
