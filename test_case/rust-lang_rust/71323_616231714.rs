
error[E0053]: method `visit_union` has an incompatible type for trait
   --> src/tools/miri/src/helpers.rs:344:13
    |
344 |             fn visit_union(&mut self, v: MPlaceTy<'tcx, Tag>, fields: usize) -> InterpResult<'tcx> {
    |             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected struct `std::num::NonZeroUsize`, found `usize`
    |
    = note: expected fn pointer `fn(&mut helpers::EvalContextExt::visit_freeze_sensitive::UnsafeCellVisitor<'ecx, 'mir, 'tcx, F>, rustc_mir::interpret::MPlaceTy<'_, _>, std::num::NonZeroUsize) -> std::result::Result<_, _>`
               found fn pointer `fn(&mut helpers::EvalContextExt::visit_freeze_sensitive::UnsafeCellVisitor<'ecx, 'mir, 'tcx, F>, rustc_mir::interpret::MPlaceTy<'tcx, _>, usize) -> std::result::Result<_, _>`

error: aborting due to previous error

For more information about this error, try `rustc --explain E0053`.
error: could not compile `miri`.
