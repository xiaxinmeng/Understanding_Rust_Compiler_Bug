
RUST_BACKTRACE=1 cargo build --verbose
       Fresh lazy_static v0.2.1
   Compiling asm v0.0.1 (file:///home/lsdev/asm)
     Running `rustc src/lib.rs --crate-name asm --crate-type lib -g --out-dir /home/lsdev/asm/target/debug --emit=dep-info,link -L dependency=/home/lsdev/asm/target/debug -L dependency=/home/lsdev/asm/target/debug/deps --extern lazy_static=/home/lsdev/asm/target/debug/deps/liblazy_static-a81b08a56ec46bff.rlib`
error: internal compiler error: unexpected panic
note: the compiler unexpectedly panicked. this is a bug.
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
note: run with `RUST_BACKTRACE=1` for a backtrace
thread 'rustc' panicked at 'assertion failed: self.appropriate_rvalue_mode(bcx.ccx()) == ByValue', src/librustc_trans/datum.rs:789
stack backtrace:
   1:     0x7f22f42d6140 - std::sys::backtrace::tracing::imp::write::hf68f1a220b61702c
   2:     0x7f22f42e386b - std::panicking::default_hook::_$u7b$$u7b$closure$u7d$$u7d$::hb638acea7c29901b
   3:     0x7f22f42e340c - std::panicking::default_hook::h508c3dab3df347d6
   4:     0x7f22f42a844f - std::sys_common::unwind::begin_unwind_inner::h17f9e42de6d55309
   5:     0x7f22f320789f - std::sys_common::unwind::begin_unwind::hedd66fca20b8a1c3
   6:     0x7f22f3307548 - _<datum..Datum<'tcx, K>>::to_llscalarish::h90da49dfa16703f5
   7:     0x7f22f3307378 - core::ops::impls::_<impl std..ops..FnOnce<A> for &'a mut F>::call_once::hb0aa51af534f50de
   8:     0x7f22f3307185 - _<std..vec..Vec<T> as std..iter..FromIterator<T>>::from_iter::hea83d25a10a342bb
   9:     0x7f22f32febc1 - rustc_trans::expr::trans_rvalue_stmt_unadjusted::he6f7be9fa97e15d9
  10:     0x7f22f3275b1f - rustc_trans::expr::trans_into::h67fb303b64e53185
  11:     0x7f22f32d9d1b - rustc_trans::controlflow::trans_stmt_semi::h6dfb389a66ac41c8
  12:     0x7f22f326d2f0 - rustc_trans::controlflow::trans_block::hb0ab7613146bf62c
  13:     0x7f22f32fc530 - rustc_trans::expr::trans_rvalue_dps_unadjusted::hdf00eddfdf40ffbd
  14:     0x7f22f3275b44 - rustc_trans::expr::trans_into::h67fb303b64e53185
  15:     0x7f22f326d61d - rustc_trans::controlflow::trans_block::hb0ab7613146bf62c
  16:     0x7f22f326bb65 - rustc_trans::base::trans_closure::hb994e1a0888be25b
  17:     0x7f22f326dd45 - rustc_trans::base::trans_fn::h88500350326131d7
  18:     0x7f22f3278a69 - rustc_trans::base::trans_item::hda6e7b5dd0b4b42f
  19:     0x7f22f3290d4b - _<base..TransItemsWithinModVisitor<'a, 'tcx> as rustc..hir..intravisit..Visitor<'v>>::visit_item::ha66039a6625cc579
  20:     0x7f22f327f1db - rustc_trans::base::trans_crate::hd61624dbab389bc0
  21:     0x7f22f4846b3f - rustc_driver::driver::phase_4_translate_to_llvm::h02f3cb83bf92a921
  22:     0x7f22f484523b - rustc_driver::driver::compile_input::_$u7b$$u7b$closure$u7d$$u7d$::h828c3d094bf44949
  23:     0x7f22f4841bb0 - rustc_driver::driver::phase_3_run_analysis_passes::_$u7b$$u7b$closure$u7d$$u7d$::he8d0a22aa8076fed
  24:     0x7f22f483b68b - rustc::ty::context::TyCtxt::create_and_enter::h980e1fdb00693075
  25:     0x7f22f483813e - rustc_driver::driver::phase_3_run_analysis_passes::h611c3bbb7fd33763
  26:     0x7f22f480a99f - rustc_driver::driver::compile_input::hd56cf2fd4d4af18d
  27:     0x7f22f47f0f04 - rustc_driver::run_compiler::h0185bb759e59b39b
  28:     0x7f22f47ee361 - std::sys_common::unwind::try::try_fn::hdf9deeebe718955f
  29:     0x7f22f42d38db - __rust_try
  30:     0x7f22f42d386d - std::sys_common::unwind::inner_try::h74a189ca1fbe8e07
  31:     0x7f22f47eebaa - _<F as std..boxed..FnBox<A>>::call_box::hb28490749f4628cc
  32:     0x7f22f42e1a04 - std::sys::thread::Thread::new::thread_start::h40a33956f4cb7596
  33:     0x7f22ece526a9 - start_thread
  34:     0x7f22f3f35e9c - clone
  35:                0x0 - <unknown>

error: Could not compile `asm`.

Caused by:
  Process didn't exit successfully: `rustc src/lib.rs --crate-name asm --crate-type lib -g --out-dir /home/lsdev/asm/target/debug --emit=dep-info,link -L dependency=/home/lsdev/asm/target/debug -L dependency=/home/lsdev/asm/target/debug/deps --extern lazy_static=/home/lsdev/asm/target/debug/deps/liblazy_static-a81b08a56ec46bff.rlib` (exit code: 101)
