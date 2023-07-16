plain
    Checking cargo_metadata v0.12.0
    Checking rustfix v0.5.1
    Checking rustc-workspace-hack v1.0.0 (/checkout/src/tools/rustc-workspace-hack)
    Checking clippy_lints v0.1.56 (/checkout/src/tools/clippy/clippy_lints)
error[E0050]: method `call_return_effect` has 6 parameters but the declaration in trait `call_return_effect` has 4
    |
501 | /         &self,
501 | /         &self,
502 | |         _in_out: &mut impl GenKill<Self::Idx>,
503 | |         _block: mir::BasicBlock,
504 | |         _func: &mir::Operand<'tcx>,
505 | |         _args: &[mir::Operand<'tcx>],
506 | |         _return_place: mir::Place<'tcx>,
    |
    |
    = note: `call_return_effect` from trait: `fn(&Self, &mut impl GenKill<Self::Idx>, BasicBlock, CallReturnPlaces<'_, 'tcx>)`
For more information about this error, try `rustc --explain E0050`.
error: could not compile `clippy_lints` due to previous error
Build completed unsuccessfully in 0:03:48
