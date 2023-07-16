
   8: core::panicking::panic_fmt
             at libcore/panicking.rs:77
   9: core::option::expect_failed
             at libcore/option.rs:1008
  10: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_pat_tuple_struct
  11: rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_pat_walk
  12: rustc_typeck::check::FnCtxt::check_decl_local
  13: rustc_typeck::check::FnCtxt::check_block_with_expected
  14: rustc_typeck::check::FnCtxt::check_expr_kind
  15: rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_needs
  16: rustc_typeck::check::FnCtxt::check_return_expr
  17: rustc_typeck::check::check_fn
