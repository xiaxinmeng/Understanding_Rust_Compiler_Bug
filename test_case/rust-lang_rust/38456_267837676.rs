
$ RUST_BACKTRACE=1 rustc src/main.rs
error: internal compiler error: ../src/librustc_typeck/check/mod.rs:4396: unexpected definition: PrimTy(TyStr)

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:423
stack backtrace:
   1:        0x1137ddf7a - std::sys::imp::backtrace::tracing::imp::write::hbea47d9dd19b523c
   2:        0x1137eb26f - std::panicking::default_hook::{{closure}}::h6875a2976258b020
   3:        0x1137eae1d - std::panicking::default_hook::h88ffbc5922643264
   4:        0x1137eb736 - std::panicking::rust_panic_with_hook::hc790e47d4ecc86cd
   5:        0x1135b583a - std::panicking::begin_panic::h264cdc75d51b518b
   6:        0x1135c84c4 - rustc_errors::Handler::bug::h620f7270292f0095
   7:        0x110193c5c - rustc::session::opt_span_bug_fmt::{{closure}}::h4a9b70c3df8b4b3a
   8:        0x110193a69 - rustc::session::opt_span_bug_fmt::h7d83586c6e2c7ae6
   9:        0x1101935ea - rustc::session::bug_fmt::he2d2f00a4afa9d1e
  10:        0x10f9d378d - rustc_typeck::check::FnCtxt::instantiate_value_path::h4229ac945aebfa55
  11:        0x10f9c1c31 - rustc_typeck::check::FnCtxt::check_expr_kind::hfeeca7568cc99087
  12:        0x10f9c086d - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h5d7f5141d7386bb8
  13:        0x10f9c466a - rustc_typeck::check::FnCtxt::check_expr_kind::hfeeca7568cc99087
  14:        0x10f9c086d - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h5d7f5141d7386bb8
  15:        0x10f9c466a - rustc_typeck::check::FnCtxt::check_expr_kind::hfeeca7568cc99087
  16:        0x10f9c086d - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h5d7f5141d7386bb8
  17:        0x10f8f6302 - core::ops::impls::<impl core::ops::FnOnce<A> for &'a mut F>::call_once::h5619f9c64db6fa06
  18:        0x10f8dc276 - <rustc_data_structures::accumulate_vec::AccumulateVec<A> as core::iter::traits::FromIterator<<A as rustc_data_structures::array_vec::Array>::Element>>::from_iter::h1e1c356f47e244c5
  19:        0x10f942106 - <T as rustc::ty::context::InternIteratorElement<T, R>>::intern_with::he42f6186b2ee1d49
  20:        0x10f9c2e73 - rustc_typeck::check::FnCtxt::check_expr_kind::hfeeca7568cc99087
  21:        0x10f9c086d - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h5d7f5141d7386bb8
  22:        0x10f95090a - rustc_typeck::check::_match::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_match::h54f69213176d4c8f
  23:        0x10f9c1092 - rustc_typeck::check::FnCtxt::check_expr_kind::hfeeca7568cc99087
  24:        0x10f9c086d - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h5d7f5141d7386bb8
  25:        0x10f9c466a - rustc_typeck::check::FnCtxt::check_expr_kind::hfeeca7568cc99087
  26:        0x10f9c086d - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h5d7f5141d7386bb8
  27:        0x10f9b6261 - rustc_typeck::check::FnCtxt::check_argument_types::he03b7446217ca4c2
  28:        0x10f993b11 - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::confirm_builtin_call::hf6c4edce4c70573d
  29:        0x10f99310d - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_call::h762a25d0c9a1f5b4
  30:        0x10f9c19ab - rustc_typeck::check::FnCtxt::check_expr_kind::hfeeca7568cc99087
  31:        0x10f9c086d - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h5d7f5141d7386bb8
  32:        0x10f9b6261 - rustc_typeck::check::FnCtxt::check_argument_types::he03b7446217ca4c2
  33:        0x10f993b11 - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::confirm_builtin_call::hf6c4edce4c70573d
  34:        0x10f99310d - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_call::h762a25d0c9a1f5b4
  35:        0x10f9c19ab - rustc_typeck::check::FnCtxt::check_expr_kind::hfeeca7568cc99087
  36:        0x10f9c086d - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h5d7f5141d7386bb8
  37:        0x10f9d24ae - rustc_typeck::check::FnCtxt::check_block_with_expected::he90dd72b6855e5e4
  38:        0x10f9c0d93 - rustc_typeck::check::FnCtxt::check_expr_kind::hfeeca7568cc99087
  39:        0x10f9c086d - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h5d7f5141d7386bb8
  40:        0x10f9a7663 - rustc_typeck::check::check_fn::h96bd864b4d4fc711
  41:        0x10f9a61f7 - rustc_typeck::check::check_bare_fn::h87386ea43f4e272e
  42:        0x10f9a9193 - rustc_typeck::check::check_item_body::h1e039ede2f960fb9
  43:        0x10f9a3dad - rustc_typeck::check::check_item_bodies::hd22adb7703cc56d7
  44:        0x10fa119de - rustc_typeck::check_crate::h92f6bdfe03b6afba
  45:        0x10efb64d0 - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::hd97ec26bf3a98036
  46:        0x10ef9e404 - rustc_driver::driver::phase_3_run_analysis_passes::h6d446abb74c09795
  47:        0x10ef8e86a - rustc_driver::driver::compile_input::h8e119234b60571d5
  48:        0x10efd5a4b - rustc_driver::run_compiler::h57c4f233cd1a0c04
  49:        0x10eef4088 - std::panicking::try::do_call::hf679f17bf3b43b0b
  50:        0x1137edcea - __rust_maybe_catch_panic
  51:        0x10ef1718f - <F as alloc::boxed::FnBox<A>>::call_box::h21b5b32059700da6
  52:        0x1137ea3b4 - std::sys::imp::thread::Thread::new::thread_start::h8084b1107992ae5b
  53:     0x7fffbcf63aaa - _pthread_body
  54:     0x7fffbcf639f6 - _pthread_start
