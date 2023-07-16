
error[E0658]: rust-call ABI is subject to change
  --> src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:12:12
   |
12 |     extern "rust-call" fn call(self, args: ()) -> () {}
   |            ^^^^^^^^^^^
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable

error[E0658]: rust-call ABI is subject to change
  --> src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:19:12
   |
19 |     extern "rust-call" fn call_once(self, args: ()) -> () {}
   |            ^^^^^^^^^^^
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable

error[E0658]: rust-call ABI is subject to change
  --> src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:26:12
   |
26 |     extern "rust-call" fn call_mut(&self, args: ()) -> () {}
   |            ^^^^^^^^^^^
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable

error[E0658]: rust-call ABI is subject to change
  --> src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:33:12
   |
33 |     extern "rust-call" fn call_once(&self, args: ()) -> () {}
   |            ^^^^^^^^^^^
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable

error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
 --> src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:9:6
  |
9 | impl Fn<()> for Foo {
  |      ^^^^^^ help: use parenthetical notation instead: `Fn() -> ()`
  |
  = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
  = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable

error[E0229]: associated type bindings are not allowed here
  --> src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:16:6
   |
16 | impl FnOnce() for Foo1 {
   |      ^^^^^^^^ associated type not allowed here

error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
  --> src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:23:6
   |
23 | impl FnMut<()> for Bar {
   |      ^^^^^^^^^ help: use parenthetical notation instead: `FnMut() -> ()`
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable

error[E0658]: the precise format of `Fn`-family traits' type parameters is subject to change
  --> src/test/ui/feature-gates/feature-gate-unboxed-closures-manual-impls.rs:30:6
   |
30 | impl FnOnce<()> for Baz {
   |      ^^^^^^^^^^ help: use parenthetical notation instead: `FnOnce() -> ()`
   |
   = note: see issue #29625 <https://github.com/rust-lang/rust/issues/29625> for more information
   = help: add `#![feature(unboxed_closures)]` to the crate attributes to enable

thread 'rustc' panicked at 'aborting after 8 errors due to `-Z treat-err-as-bug=8`', src/librustc_errors/lib.rs:930:13
stack backtrace:
   0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
   1: core::fmt::write
   2: std::io::Write::write_fmt
   3: std::sys_common::backtrace::print
   4: std::panicking::default_hook::{{closure}}
   5: std::panicking::default_hook
   6: rustc_driver::report_ice
   7: std::panicking::rust_panic_with_hook
   8: std::panicking::begin_panic
   9: rustc_errors::HandlerInner::emit_diagnostic
  10: rustc_errors::Handler::emit_diagnostic
  11: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
  12: <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_trait_ref
  13: <dyn rustc_typeck::astconv::AstConv>::ast_path_to_mono_trait_ref
  14: <dyn rustc_typeck::astconv::AstConv>::instantiate_mono_trait_ref
  15: rustc_typeck::collect::impl_trait_ref
  16: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::impl_trait_ref>::compute
  17: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  18: rustc_query_system::query::plumbing::get_query
  19: rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::trait_id_of_impl
  20: rustc_typeck::collect::codegen_fn_attrs
  21: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::codegen_fn_attrs>::compute
  22: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  23: rustc_query_system::query::plumbing::get_query
  24: rustc_passes::check_attr::CheckAttrVisitor::check_attributes
  25: <rustc_passes::check_attr::CheckAttrVisitor as rustc_hir::intravisit::Visitor>::visit_impl_item
  26: rustc_middle::hir::map::Map::visit_item_likes_in_module
  27: rustc_passes::check_attr::check_mod_attrs
  28: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::check_mod_attrs>::compute
  29: rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl
  30: rustc_query_system::query::plumbing::get_query
  31: rustc_query_system::query::plumbing::ensure_query
  32: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  33: std::panicking::try::do_call
  34: std::panicking::try::do_try
  35: std::panicking::try
  36: rustc_session::utils::<impl rustc_session::session::Session>::time
  37: rustc_interface::passes::analysis
  38: rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute
  39: rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task
  40: rustc_query_system::query::plumbing::get_query
  41: rustc_middle::ty::context::tls::enter_global
  42: rustc_interface::interface::run_compiler_in_existing_thread_pool
  43: scoped_tls::ScopedKey<T>::set
  44: rustc_ast::attr::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-dev running on x86_64-apple-darwin

note: compiler flags: -Z treat-err-as-bug=8

query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /Users/tous/Documents/Dev/rust/src/libcore/cell.rs:878:9
stack backtrace:
   0:        0x10008abde - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5ae43269bbebfacb
   1:        0x1000c032e - core::fmt::write::h07cd7fb0e37fcedf
   2:        0x100057e57 - std::io::Write::write_fmt::hee8ab4b967054396
   3:        0x10008a9f5 - std::sys_common::backtrace::print::hb0ff70fe048505fa
   4:        0x1000850de - std::panicking::default_hook::{{closure}}::hf0f43d75e4f49668
   5:        0x100084ef2 - std::panicking::default_hook::hda93d32041f4feae
   6:        0x107ba8a84 - rustc_driver::report_ice::hdc18c1e69237d9f7
   7:        0x1000857a4 - std::panicking::rust_panic_with_hook::hc8a9fbdf3a66d229
   8:        0x100085222 - rust_begin_unwind
   9:        0x1000d35ff - core::panicking::panic_fmt::h2b125370a9a0909a
  10:        0x1000d3bb5 - core::result::unwrap_failed::h10f4062c0589485e
  11:        0x10bfc7bd4 - rustc_errors::Handler::emit_diagnostic::h664a59e6bffab4a6
  12:        0x10c000d88 - rustc_errors::diagnostic_builder::DiagnosticBuilder::emit::h99314ec9b3de4aac
  13:        0x10b8e1151 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::impl_trait_ref>::handle_cycle_error::h8032eb24a78efc3e
  14:        0x10d0609d4 - rustc_data_structures::cold_path::h87fdb370d0610f68
  15:        0x10bab5f16 - rustc_query_system::query::plumbing::get_query::h4ce39f7f6cdfd44a
  16:        0x10bb5e0cd - <rustc_middle::ty::print::pretty::FmtPrinter<F> as rustc_middle::ty::print::Printer>::print_def_path::h2bc652cadaae1b7d
  17:        0x10b96cb64 - rustc_middle::ty::print::pretty::<impl rustc_middle::ty::context::TyCtxt>::def_path_str_with_substs::h52a65857dce1058b
  18:        0x10b853447 - <M as rustc_query_system::query::config::QueryDescription<CTX>>::describe::h25d51fc7bd8250b1
  19:        0x10bb315e6 - rustc_middle::ty::query::Query::describe::h45194de6b1d4b007
  20:        0x10b96cf92 - rustc_middle::ty::query::plumbing::<impl rustc_middle::ty::context::TyCtxt>::try_print_query_stack::h8e08ace965244981
  21:        0x107ba935a - rustc_driver::report_ice::hdc18c1e69237d9f7
  22:        0x1000857a4 - std::panicking::rust_panic_with_hook::hc8a9fbdf3a66d229
  23:        0x10d1b837d - std::panicking::begin_panic::h5b5cf998f660f2a3
  24:        0x10bfc81d7 - rustc_errors::HandlerInner::emit_diagnostic::hadd38bdf37f2be1f
  25:        0x10bfc7b94 - rustc_errors::Handler::emit_diagnostic::h664a59e6bffab4a6
  26:        0x10c000d88 - rustc_errors::diagnostic_builder::DiagnosticBuilder::emit::h99314ec9b3de4aac
  27:        0x10a23cd8f - <dyn rustc_typeck::astconv::AstConv>::create_substs_for_ast_trait_ref::h38e2a6333fca2349
  28:        0x10a23c8a1 - <dyn rustc_typeck::astconv::AstConv>::ast_path_to_mono_trait_ref::h56faf73a380b6ccc
  29:        0x10a23b5b2 - <dyn rustc_typeck::astconv::AstConv>::instantiate_mono_trait_ref::h407e2de81f0e13e2
  30:        0x10a41f723 - rustc_typeck::collect::impl_trait_ref::hb596d1b5a03054ec
  31:        0x10bb210fb - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::impl_trait_ref>::compute::hd405fb153988501d
  32:        0x10b88dc6a - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h61b153a8adc16827
  33:        0x10bab613b - rustc_query_system::query::plumbing::get_query::h4ce39f7f6cdfd44a
  34:        0x10b97c4bb - rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::trait_id_of_impl::he9e13fb946656500
  35:        0x10a42233e - rustc_typeck::collect::codegen_fn_attrs::h9c342f9d78bbea16
  36:        0x10a70ea43 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::codegen_fn_attrs>::compute::ha03557901ca21363
  37:        0x10a741c92 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hd0968caf61fde38a
  38:        0x10a71691d - rustc_query_system::query::plumbing::get_query::h4446211b5ebc08ec
  39:        0x10a66f63e - rustc_passes::check_attr::CheckAttrVisitor::check_attributes::h94f3bac521f4c938
  40:        0x10a67044e - <rustc_passes::check_attr::CheckAttrVisitor as rustc_hir::intravisit::Visitor>::visit_impl_item::h61e3a5bfd7c90165
  41:        0x10a662ea8 - rustc_middle::hir::map::Map::visit_item_likes_in_module::hd1290d1486a633c8
  42:        0x10a6706e5 - rustc_passes::check_attr::check_mod_attrs::hcbfa413588a4dae3
  43:        0x107cac46b - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::check_mod_attrs>::compute::h14889cee594534a7
  44:        0x107d56bf0 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h786530be38e80ce4
  45:        0x107cb4dce - rustc_query_system::query::plumbing::get_query::h33f5e25a46624303
  46:        0x107cae3d8 - rustc_query_system::query::plumbing::ensure_query::h8c8f096c3ace4b22
  47:        0x107c98adb - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::hf5e0c9fbe1cd4134
  48:        0x107d26070 - std::panicking::try::do_call::h984c505755627d84
  49:        0x10008514a - std::panicking::try::do_try::hdeb1365c30635857
  50:        0x107d25973 - std::panicking::try::hd35d6ac7bdc22bc3
  51:        0x107d160c8 - rustc_session::utils::<impl rustc_session::session::Session>::time::h1f99590337fb9778
  52:        0x107d6c6d1 - rustc_interface::passes::analysis::h582f524545c4d1fc
  53:        0x107b9c502 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::analysis>::compute::h29930638ff0775c6
  54:        0x107a66920 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::hdbe931e5aca38514
  55:        0x107b9d275 - rustc_query_system::query::plumbing::get_query::h159786a1ca42ccb6
  56:        0x107a802c5 - rustc_middle::ty::context::tls::enter_global::h93266dc302006862
  57:        0x107a829c3 - rustc_interface::interface::run_compiler_in_existing_thread_pool::h1cb1e9809e4832cc
  58:        0x107a806a7 - scoped_tls::ScopedKey<T>::set::h1ec9af245babe67e
  59:        0x107a83e25 - rustc_ast::attr::with_globals::ha8ba069d970440a3
  60:        0x107b7a236 - std::sys_common::backtrace::__rust_begin_short_backtrace::hfd22e98f92f23dcf
  61:        0x107b9b297 - std::panicking::try::do_call::h2ea2509554d7adc1
  62:        0x10008514a - std::panicking::try::do_try::hdeb1365c30635857
  63:        0x107b9b177 - std::panicking::try::hf50c50bca14e6a14
  64:        0x107bb947c - core::ops::function::FnOnce::call_once{{vtable.shim}}::h2a7836e1f3c11aef
  65:        0x10006501e - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hfd2401395bcceb10
  66:        0x100064fa6 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h0f0ed35928d86a2b
  67:        0x10006ccba - std::sys::unix::thread::Thread::new::thread_start::hf06c42123263313f
  68:     0x7fff64d28d76 - _ZL12preoptimized

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.44.0-dev running on x86_64-apple-darwin

note: compiler flags: -Z treat-err-as-bug=8

query stack during panic:
thread panicked while processing panic. aborting.
[1]    86494 illegal hardware instruction  RUST_BACKTRACE=1 rustc +stage1 -Ztreat-err-as-bug=8
