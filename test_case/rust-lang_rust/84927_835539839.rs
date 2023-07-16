rust
ty::Instance::resolve_for_vtable(
    tcx,
    ty::ParamEnv::reveal_all(),
    def_id,
    substs,
)
.unwrap()
