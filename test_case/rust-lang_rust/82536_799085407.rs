plain
    Checking clippy_lints v0.1.52 (/checkout/src/tools/clippy/clippy_lints)
error[E0046]: not all trait items implemented, missing: `fake_read`
  --> src/tools/clippy/clippy_lints/src/loops/mut_range_bound.rs:83:1
   |
83 | impl<'tcx> Delegate<'tcx> for MutatePairDelegate<'_, 'tcx> {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ missing `fake_read` in implementation
   |
   = help: implement the missing item: `fn fake_read(&mut self, _: rustc_typeck::expr_use_visitor::Place<'tcx>, _: FakeReadCause, _: HirId) { todo!() }`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0046`.
error: could not compile `clippy_lints`
