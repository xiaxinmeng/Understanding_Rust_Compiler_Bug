
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/option.rs:362

stack backtrace:
   1:      0x360164ec0b9 - sys::backtrace::write::h0bd0381a7c6d019fBYr
   2:      0x360164f3e46 - panicking::on_panic::h953134dfac303ab1pow
   3:      0x360164b5ba2 - rt::unwind::begin_unwind_inner::hd1b458e2307f3d05z3v
   4:      0x360164b6967 - rt::unwind::begin_unwind_fmt::h08053ed3951de550F2v
   5:      0x360164f3a26 - rust_begin_unwind
   6:      0x36016540084 - panicking::panic_fmt::hf5d8479bf55b3fa2NKy
   7:      0x3601653ec64 - panicking::panic::h824ffaba30018d8bkJy
   8:      0x360145fd848 - middle::ty::TraitRef<'tcx>::self_ty::h551bfe30dbbedbc9xr3
   9:      0x36014627172 - middle::traits::select::SelectionContext<'cx, 'tcx>::candidate_from_obligation::hf8ada0827e1be47bteR
  10:      0x360144ff3ed - middle::traits::select::SelectionContext<'cx, 'tcx>::select::h377fd22a5284042fsSQ
  11:      0x360144f973a - middle::const_eval::resolve_trait_associated_const::h9711fd07dfd9af7cQMi
  12:      0x360144acdf8 - middle::const_eval::eval_const_expr_partial::hb8dc4544c452182dcki
  13:      0x36015ce0c35 - astconv::ast_ty_to_ty::h71cca4fc3e245676nSv
  14:      0x36015ce0dac - astconv::ast_ty_to_ty::h71cca4fc3e245676nSv
  15:      0x36015d31a4d - astconv::convert_ty_with_lifetime_elision::ha9a5c36346b84adeBQu
  16:      0x36015d3b8d6 - astconv::ty_of_method_or_bare_fn::h225734b640577932p8v
  17:      0x36015d58b6e - collect::convert_method::hd7d7ee3346f6b3e6zlx
  18:      0x36015d43f38 - collect::convert_item::hd27f0bc8c9c983e37Ax
  19:      0x36015d8baf6 - check_crate::closure.38750
  20:      0x36015d89abb - check_crate::h8249ae25e4dcf889tCC
  21:      0x36016a2f728 - driver::phase_3_run_analysis_passes::h5acb8c02893af881tGa
  22:      0x36016a10fac - driver::compile_input::h2d4d7c29076e9ffaQba
  23:      0x36016aca2f1 - run_compiler::h39664fd458db49c365b
  24:      0x36016ac7b42 - boxed::F.FnBox<A>::call_box::h8536945657960310400
  25:      0x36016ac70e9 - rt::unwind::try::try_fn::h4867300882306443046
  26:      0x36016567788 - rust_try_inner
  27:      0x36016567775 - rust_try
  28:      0x36016ac7390 - boxed::F.FnBox<A>::call_box::h18199782063315395191
  29:      0x360164f2be1 - sys::thread::Thread::new::thread_start::h92b0e1bfe7b243d409u
  30:      0x3601059c68b - <unknown>
  31:      0x3601615032c - clone
  32:                0x0 - <unknown>
