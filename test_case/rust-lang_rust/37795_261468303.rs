
error: internal compiler error: ../src/librustc_metadata/decoder.rs:490: entry: id not found: DefIndex(1) in crate "serde_derive" with number 12

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', ../src/librustc_errors/lib.rs:424
stack backtrace:
   1:        0x113b564ea - std::sys::imp::backtrace::tracing::imp::write::h944c02ac40aee2d7
   2:        0x113b6377f - std::panicking::default_hook::{{closure}}::h6875a2976258b020
   3:        0x113b6332d - std::panicking::default_hook::h88ffbc5922643264
   4:        0x113b63c46 - std::panicking::rust_panic_with_hook::ha5aed1dfc0e220e3
   5:        0x1112f5f2a - std::panicking::begin_panic::h264cdc75d51b518b
   6:        0x111306d34 - rustc_errors::Handler::bug::h620f7270292f0095
   7:        0x110663ecc - rustc::session::opt_span_bug_fmt::{{closure}}::h4a9b70c3df8b4b3a
   8:        0x110663cd9 - rustc::session::opt_span_bug_fmt::h7d83586c6e2c7ae6
   9:        0x11066385a - rustc::session::bug_fmt::he2d2f00a4afa9d1e
  10:        0x10fb94463 - rustc_metadata::decoder::<impl rustc_metadata::cstore::CrateMetadata>::entry::h5e3d8114267e9116
  11:        0x10fb9ceab - rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore<'tcx> for rustc_metadata::cstore::CStore>::visibility::h2949b511f18961ee
  12:        0x10fba59a6 - rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore<'tcx> for rustc_metadata::cstore::CStore>::visible_parent_map::hab53861eb74bce8d
  13:        0x1106ad1c4 - rustc::ty::item_path::<impl rustc::ty::context::TyCtxt<'a, 'gcx, 'tcx>>::push_item_path::haa7f7a9afda5f72a
  14:        0x1106f92d5 - rustc::util::ppaux::parameterized::h7bcfb1fd3d8a947a
  15:        0x113ba7cb5 - core::fmt::write::h01739b8f12f355f9
  16:        0x113ba8b6d - core::fmt::Formatter::write_fmt::h50f274f007b40806
  17:        0x110701cf2 - rustc::util::ppaux::<impl core::fmt::Display for rustc::ty::TraitPredicate<'tcx>>::fmt::he0dbf908fd919098
  18:        0x113ba7cb5 - core::fmt::write::h01739b8f12f355f9
  19:        0x113ba8b6d - core::fmt::Formatter::write_fmt::h50f274f007b40806
  20:        0x1106fe8e0 - rustc::util::ppaux::<impl core::fmt::Display for rustc::ty::sty::Binder<rustc::ty::TraitPredicate<'tcx>>>::fmt::hb2f823bd10699038
  21:        0x113ba7cb5 - core::fmt::write::h01739b8f12f355f9
  22:        0x113ba8b6d - core::fmt::Formatter::write_fmt::h50f274f007b40806
  23:        0x110701fbd - rustc::util::ppaux::<impl core::fmt::Display for rustc::ty::Predicate<'tcx>>::fmt::hcaf28c3b7b8ea3ed
  24:        0x113ba7cb5 - core::fmt::write::h01739b8f12f355f9
  25:        0x113b66f6e - collections::fmt::format::h2e5dd97da45bd3ab
  26:        0x11066c014 - rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_selection_error::h962cd4a7e26d176d
  27:        0x1106669fb - rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_fulfillment_error::hb3520d160f1dddc3
  28:        0x1106654b1 - rustc::traits::error_reporting::<impl rustc::infer::InferCtxt<'a, 'gcx, 'tcx>>::report_fulfillment_errors::h9a88e2fc91b0b056
  29:        0x10f94b2be - rustc_typeck::check::FnCtxt::select_obligations_where_possible::hb71c39774d7865bc
  30:        0x10f94c391 - rustc_typeck::check::FnCtxt::check_argument_types::he326f7b617348eec
  31:        0x10f91b6dc - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::confirm_builtin_call::h2ace35e908f32dc7
  32:        0x10f91ac14 - rustc_typeck::check::callee::<impl rustc_typeck::check::FnCtxt<'a, 'gcx, 'tcx>>::check_call::h1477c9c452056cce
  33:        0x10f952619 - rustc_typeck::check::FnCtxt::check_expr_kind::h7d21f5f45aa93926
  34:        0x10f9519cd - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h2b84bdb152466f80
  35:        0x10f9521bb - rustc_typeck::check::FnCtxt::check_expr_kind::h7d21f5f45aa93926
  36:        0x10f9519cd - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h2b84bdb152466f80
  37:        0x10f9601db - rustc_typeck::check::FnCtxt::check_decl_initializer::hcde9e1bad2c91b30
  38:        0x10f9602e3 - rustc_typeck::check::FnCtxt::check_decl_local::h6cf1db2e01e7cb8e
  39:        0x10f9605dc - rustc_typeck::check::FnCtxt::check_stmt::hd3d24c9e1f029860
  40:        0x10f9608fe - rustc_typeck::check::FnCtxt::check_block_with_expected::he5a0b03e6951b34f
  41:        0x10f951e97 - rustc_typeck::check::FnCtxt::check_expr_kind::h7d21f5f45aa93926
  42:        0x10f9519cd - rustc_typeck::check::FnCtxt::check_expr_with_expectation_and_lvalue_pref::h2b84bdb152466f80
  43:        0x10f9321b5 - rustc_typeck::check::check_fn::h96bd864b4d4fc711
  44:        0x10f930cf7 - rustc_typeck::check::check_bare_fn::heb8c1cdb84569c5e
  45:        0x10f933d75 - rustc_typeck::check::check_item_body::h1e039ede2f960fb9
  46:        0x10f92ecf5 - rustc_typeck::check::check_item_bodies::hd22adb7703cc56d7
  47:        0x10f99c208 - rustc_typeck::check_crate::h4045752b69a796e8
  48:        0x10f4495ec - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h97a3a12d948df547
  49:        0x10f4467bd - rustc_driver::driver::phase_3_run_analysis_passes::hb0ad9de18d423e67
  50:        0x10f42e005 - rustc_driver::driver::compile_input::h8e119234b60571d5
  51:        0x10f474099 - rustc_driver::run_compiler::hbdfc4f84e2e0f4b9
  52:        0x10f392d98 - std::panicking::try::do_call::hf679f17bf3b43b0b
  53:        0x113b661fa - __rust_maybe_catch_panic
  54:        0x10f3b6eef - <F as alloc::boxed::FnBox<A>>::call_box::h506fb5d7b8891cd4
  55:        0x113b628d4 - std::sys::imp::thread::Thread::new::thread_start::h8084b1107992ae5b
  56:     0x7fffe9907aaa - _pthread_body
  57:     0x7fffe99079f6 - _pthread_start
