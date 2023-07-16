rust
fn abort(_ecx: &mut InterpCx<'mir, 'tcx, Self>, msg: String) -> InterpResult<'tcx, !> {
    Err(ConstEvalErrKind::Abort(msg))
}
