
[00:47:34] error[E0061]: this function takes 3 parameters but 2 parameters were supplied
[00:47:34]     --> /checkout/src/tools/miri/src/librustc_mir/interpret/eval_context.rs:2419:45
[00:47:34]      |
[00:47:34] 2419 |     let vtbl = tcx.trans_fulfill_obligation(DUMMY_SP, ty::Binder(trait_ref));
[00:47:34]      |                                             ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ expected 3 parameters
[00:47:34] 
[00:47:35] error: aborting due to previous error
[00:47:35] 
[00:47:35] error: Could not compile `rustc_miri`.
