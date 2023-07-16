rust
fn abort(_ecx: &mut InterpCx<'mir, 'tcx, Self>) -> InterpResult<'tcx, !> {
    throw_unsup_format!("aborting execution is not supported")
}
