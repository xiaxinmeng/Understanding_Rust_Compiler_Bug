rust
#[cfg(parallel_compiler)]
#[allow(rustc::usage_of_qualified_ty)]
pub fn check_for_send(tcx: ty::TyCtxt<'static>) -> impl Send {
    tcx
}
