
error: internal compiler error: /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/compiler/rustc_codegen_ssa/src/mir/constant.rs:42:20: encountered bad ConstKind after monomorphizing: Error(DelaySpanBugEmitted(()))
  --> reed-solomon-novelpoly/src/field/traits.rs:28:15
   |
28 |     let res = <MyStruct as MyTrait>::ARRAY[2]; // PANICS HERE
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/compiler/rustc_errors/src/lib.rs:1061:9
stack backtrace:
   0:     0x7fadca9c25c0 - std::backtrace_rs::backtrace::libunwind::trace::h0197874f43e7190f
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7fadca9c25c0 - std::backtrace_rs::backtrace::trace_unsynchronized::hfdfd566bbbda6f2d
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7fadca9c25c0 - std::sys_common::backtrace::_print_fmt::hb3b4707c15b032a1
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7fadca9c25c0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::had1dceac8b072f88
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7fadcaa3088c - core::fmt::write::h9a6d9c74526a6c1b
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/core/src/fmt/mod.rs:1117:17
   5:     0x7fadca9b3e15 - std::io::Write::write_fmt::h05c3bb2bc83b8ed0
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/io/mod.rs:1667:15
   6:     0x7fadca9c5b6b - std::sys_common::backtrace::_print::h10f0826b2a586b58
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7fadca9c5b6b - std::sys_common::backtrace::print::h0d4242fbd1ac3eb6
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7fadca9c5b6b - std::panicking::default_hook::{{closure}}::h275ead39fe8b8e2a
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/panicking.rs:210:50
   9:     0x7fadca9c56fb - std::panicking::default_hook::h5505d4d90f5984d9
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/panicking.rs:227:9
  10:     0x7fadcb1a8441 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::hfafa74d182a98572
  11:     0x7fadb9fd53b3 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h2b79204794dbfa84
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/alloc/src/boxed.rs:1650:9
  12:     0x7fadb9fd2ffd - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::hf32c5214921a97db
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/proc_macro/src/bridge/client.rs:320:21
  13:     0x7fadca9c6399 - std::panicking::rust_panic_with_hook::h2f8980d1d9fd4ca0
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/panicking.rs:628:17
  14:     0x7fadcb2f9c2b - std::panicking::begin_panic::{{closure}}::h0e2e1a1aa4ce315f
  15:     0x7fadcb2f9736 - std::sys_common::backtrace::__rust_end_short_backtrace::h4eb9afe6bb16aed5
  16:     0x7fadcb31c876 - std::panicking::begin_panic::h9dd4456f75ce6691
  17:     0x7fadcb31c7d6 - std::panic::panic_any::h6b06b88cda88784e
  18:     0x7fadcb31aae5 - rustc_errors::HandlerInner::span_bug::hc6acfc2c2084be90
  19:     0x7fadcb31ae80 - rustc_errors::Handler::span_bug::h762758f218baebb2
  20:     0x7fadcb312af5 - rustc_middle::ty::context::tls::with_opt::h71ce7bbce4f174ce
  21:     0x7fadcb312b90 - rustc_middle::util::bug::opt_span_bug_fmt::haf876d53e593e3d0
  22:     0x7fadcb312b5c - rustc_middle::util::bug::span_bug_fmt::h368a3256a5c02003
  23:     0x7fadcc300eb8 - rustc_codegen_ssa::mir::constant::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::eval_mir_constant::h41a46f521048047a
  24:     0x7fadcc2f1e4d - rustc_codegen_ssa::mir::codegen_mir::hbc621ab63ff51430
  25:     0x7fadcc2cfd36 - rustc_codegen_ssa::base::codegen_instance::h91346eecdc80ef31
  26:     0x7fadcc2dff96 - <rustc_middle::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define::h85759a705242d1c7
  27:     0x7fadcc2e16df - rustc_codegen_llvm::base::compile_codegen_unit::module_codegen::h0a45249604e8656d
  28:     0x7fadccd1d5a6 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h24bdf5ffdfe13e63
  29:     0x7fadccd35527 - rustc_codegen_llvm::base::compile_codegen_unit::h3e8c94a58b30e230
  30:     0x7fadccd01aec - rustc_codegen_ssa::base::codegen_crate::hdbaa5305d943869d
  31:     0x7fadccd2719a - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::hff050ff248e149ba
  32:     0x7fadccca4576 - rustc_interface::queries::Queries::ongoing_codegen::h8543ef60effd5971
  33:     0x7fadccc7bbac - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h65311acc74a98b42
  34:     0x7fadccc69cac - rustc_span::with_source_map::hd15b16ce10abe356
  35:     0x7fadccc7b33f - scoped_tls::ScopedKey<T>::set::h7c4648ae17840458
  36:     0x7fadccc6a37a - std::sys_common::backtrace::__rust_begin_short_backtrace::h471322c14806fa6a
  37:     0x7fadccc69415 - core::ops::function::FnOnce::call_once{{vtable.shim}}::ha951d63e2ad1502e
  38:     0x7fadca9d2d13 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h9c4443ed82ca1eae
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/alloc/src/boxed.rs:1636:9
  39:     0x7fadca9d2d13 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h4b334c81001270ca
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/alloc/src/boxed.rs:1636:9
  40:     0x7fadca9d2d13 - std::sys::unix::thread::Thread::new::thread_start::h092d15d6be9ed8c7
                               at /rustc/0035d9dcecee49d1f7349932bfa52c05a6f83641/library/std/src/sys/unix/thread.rs:106:17
  41:     0x7fadca90a590 - start_thread
  42:     0x7fadca81d223 - clone
  43:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.56.0-nightly (0035d9dce 2021-08-16) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
warning: `reed-solomon-novelpoly` (lib test) generated 48 warnings
error: could not compile `reed-solomon-novelpoly`; 48 warnings emitted
