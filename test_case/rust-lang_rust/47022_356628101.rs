
  11: rustc::ty::util::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::dtorck_constraint_for_ty
             at src/librustc/ty/util.rs:609
  12: rustc_mir::borrow_check::nll::type_check::liveness::generate
             at src/librustc_mir/borrow_check/nll/type_check/liveness.rs:192
             at src/librustc_mir/borrow_check/nll/type_check/liveness.rs:126
             at src/librustc_mir/borrow_check/nll/type_check/liveness.rs:51
  13: rustc_mir::borrow_check::nll::type_check::type_check::{{closure}}
             at src/librustc_mir/borrow_check/nll/type_check/mod.rs:116
  14: rustc_mir::borrow_check::nll::type_check::type_check_internal
             at src/librustc_mir/borrow_check/nll/type_check/mod.rs:150
  15: rustc_mir::borrow_check::nll::type_check::type_check
             at src/librustc_mir/borrow_check/nll/type_check/mod.rs:108
