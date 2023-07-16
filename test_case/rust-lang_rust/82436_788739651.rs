rust
pub struct Pick<'tcx> {
    ...

    /// Indicates that we want to add an autoref (and maybe also unsize it), or if the receiver is
    /// `*mut T`, convert it to `*const T`.
    pub autoref_or_ptr_adjustment: Option<AutorefOrPtrAdjustment<'tcx>>,
}
