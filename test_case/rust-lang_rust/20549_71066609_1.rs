
$ RUST_BACKTRACE=1 rustc bug.rs
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: http://doc.rust-lang.org/complement-bugreport.html
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /home/rustbuild/src/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libcore/option.rs:361

stack backtrace:
   1:     0x7fd06dc3fa20 - sys::backtrace::write::h8a4dc9e73e8a435dvRt
   2:     0x7fd06dc618c0 - failure::on_fail::h2ba8afb30d5ff67304z
   3:     0x7fd06dbd0180 - rt::unwind::begin_unwind_inner::h312ad0c8f7e45209eKz
   4:     0x7fd06dbd0cb0 - rt::unwind::begin_unwind_fmt::hb26174a8353d5ac1LIz
   5:     0x7fd06dc61720 - rust_begin_unwind
   6:     0x7fd06dcaa270 - panicking::panic_fmt::hcf948883f7aad095Prm
   7:     0x7fd06dca74c0 - panicking::panic::hc4268c2fb464581aSpm
   8:     0x7fd06bd8bb70 - middle::fast_reject::simplify_type::h7569aab642affa6aQ1n
   9:     0x7fd06beaa810 - middle::traits::select::SelectionContext<'cx, 'tcx>::match_impl::h8c336050cd45a989NXR
  10:     0x7fd06beb4850 - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates_from_impls::unboxed_closure.76632
  11:     0x7fd06beb18a0 - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates_from_impls::h90169ef31b5ba3a3PIQ
  12:     0x7fd06beae4f0 - middle::traits::select::SelectionContext<'cx, 'tcx>::assemble_candidates::h6f75fd8f92a0efc0JdQ
  13:     0x7fd06bea1740 - middle::traits::select::SelectionContext<'cx, 'tcx>::candidate_from_obligation::h4cfba6bf61198452RSP
  14:     0x7fd06be89fe0 - middle::traits::select::SelectionContext<'cx, 'tcx>::select::heded53003941f711TxP
  15:     0x7fd06be86fd0 - middle::traits::fulfill::FulfillmentContext<'tcx>::select::ha822caaaaad61b90zGN
  16:     0x7fd06be86900 - middle::traits::fulfill::FulfillmentContext<'tcx>::select_where_possible::h02cae374cd3da6feFFN
  17:     0x7fd06bd63830 - middle::traits::fulfill::FulfillmentContext<'tcx>::select_all_or_error::h52641b14bbae077dGDN
  18:     0x7fd06bec1910 - middle::traits::evaluate_builtin_bound::hcf040ef39f6235b1HmT
  19:     0x7fd06bec21f0 - middle::traits::type_known_to_meet_builtin_bound::haca8abb2f46fa2902rT
  20:     0x7fd06cc7eb50 - check::FnCtxt<'a, 'tcx>.mc..Typer<'tcx>::type_moves_by_default::hab346d0d97101078DCk
  21:     0x7fd06bd521e0 - middle::expr_use_visitor::copy_or_move::hf546009675e54e40aIn
  22:     0x7fd06cc7daf0 - middle::mem_categorization::MemCategorizationContext<'t, TYPER>::cat_pattern_::h17264932787913456602
  23:     0x7fd06cc7d8a0 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::determine_pat_move_mode::h3374001298130170265
  24:     0x7fd06cc7d7f0 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_irrefutable_pat::h2621705422205095360
  25:     0x7fd06cc81ec0 - middle::expr_use_visitor::ExprUseVisitor<'d, 't, 'tcx, TYPER>::walk_block::h2009545953353444490
  26:     0x7fd06cc7b000 - check::upvar::AdjustBorrowKind<'a, 'tcx>::analyze_fn::hefaddfb947087b55Szi
  27:     0x7fd06ccb0190 - check::check_bare_fn::hb4168c50feefe4f5fNk
  28:     0x7fd06cca7d80 - check::check_item::hd2a7f68ea80a9d49S5k
  29:     0x7fd06cd758a0 - check_crate::unboxed_closure.31151
  30:     0x7fd06cd703d0 - check_crate::h75580db133241186Bpy
  31:     0x7fd06e195470 - driver::phase_3_run_analysis_passes::h3ddac035093704e2WFa
  32:     0x7fd06e182c90 - driver::compile_input::h8f87e6545ca866f1Cba
  33:     0x7fd06e24d8e0 - run_compiler::h01aa999b613651f1l9b
  34:     0x7fd06e24c050 - thunk::F.Invoke<A, R>::invoke::h5072366889563560254
  35:     0x7fd06e24afb0 - rt::unwind::try::try_fn::h15081826542860147634
  36:     0x7fd06dcc8930 - rust_try_inner
  37:     0x7fd06dcc8920 - rust_try
  38:     0x7fd06e24b260 - thunk::F.Invoke<A, R>::invoke::h301471058200503190
  39:     0x7fd06dc4f550 - sys::thread::thread_start::h204ed77b5f1055c7QGw
  40:     0x7fd067e510c0 - start_thread
  41:     0x7fd06d870fd9 - __clone
  42:                0x0 - <unknown>

$ RUST_BACKTRACE=1 rustc --version --verbose bug.rs
rustc 1.0.0-nightly (29bd9a06e 2015-01-20 23:03:09 +0000)
binary: rustc
commit-hash: 29bd9a06efd2f8c8a7b1102e2203cc0e6ae2dcba
commit-date: 2015-01-20 23:03:09 +0000
host: x86_64-unknown-linux-gnu
release: 1.0.0-nightly
