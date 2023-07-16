`
error: internal compiler error: compiler/rustc_symbol_mangling/src/v0.rs:537:17: symbol_names: unsupported constant of type `&str` (Const { ty: &str, val: Value(Slice { data: Allocation { bytes: [105, 110, 116], relocations: Relocations(SortedMap { data: [] }), init_mask: InitMask { blocks: [7], len: Size { raw: 3 } }, size: Size { raw: 3 }, align: Align { pow2: 0 }, mutability: Not, extra: () }, start: 0, end: 3 }) })

thread 'rustc' panicked at 'Box<Any>', compiler/rustc_errors/src/lib.rs:958:9
stack backtrace:
   0:     0x7f284253b527 - std::backtrace_rs::backtrace::libunwind::trace::h746c3e9529d524bc
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7f284253b527 - std::backtrace_rs::backtrace::trace_unsynchronized::h84373278bfb39e0c
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f284253b527 - std::sys_common::backtrace::_print_fmt::h517324efde750597
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f284253b527 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hf594ab77fac89284
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f28425ac0ec - core::fmt::write::h3868db8542c90941
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/core/src/fmt/mod.rs:1078:17
   5:     0x7f284252d3f2 - std::io::Write::write_fmt::h3f6656f045fa877f
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/std/src/io/mod.rs:1519:15
   6:     0x7f284253f215 - std::sys_common::backtrace::_print::hda7655c057c24dcc
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f284253f215 - std::sys_common::backtrace::print::h546a6c8431d46287
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f284253f215 - std::panicking::default_hook::{{closure}}::h006dd083853faf51
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/std/src/panicking.rs:208:50
   9:     0x7f284253ed6a - std::panicking::default_hook::hf0f9afb1017317fc
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/std/src/panicking.rs:225:9
  10:     0x7f2842df4cd8 - rustc_driver::report_ice::h745efc047fa5f80b
  11:     0x7f284253fb16 - std::panicking::rust_panic_with_hook::hb7a19826c029b1d6
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/std/src/panicking.rs:595:17
  12:     0x7f2845fb050d - std::panicking::begin_panic::{{closure}}::he3ac55d11a883a10
  13:     0x7f2845fb03c6 - std::sys_common::backtrace::__rust_end_short_backtrace::h4402bc3ed558879b
  14:     0x7f2845fb04af - std::panicking::begin_panic::hd2137c659c375844
  15:     0x7f2845febe2c - rustc_errors::HandlerInner::bug::hbfb11e3c8ba1475f
  16:     0x7f2845fea540 - rustc_errors::Handler::bug::ha00f48e1291906a1
  17:     0x7f28458a9744 - rustc_middle::util::bug::opt_span_bug_fmt::{{closure}}::h08c1bd52850a380c
  18:     0x7f28458a43bb - rustc_middle::ty::context::tls::with_opt::{{closure}}::h82c056b7e0ea907f
  19:     0x7f28458a4362 - rustc_middle::ty::context::tls::with_opt::h001ea91e9a6c9bc0
  20:     0x7f28458a9663 - rustc_middle::util::bug::opt_span_bug_fmt::h033dd7c3ae8939b2
  21:     0x7f28458a95d5 - rustc_middle::util::bug::bug_fmt::hededfcfaf49f2715
  22:     0x7f28449dc23a - <rustc_symbol_mangling::v0::SymbolMangler as rustc_middle::ty::print::Printer>::print_const::hb4db9f7658de19c3
  23:     0x7f28449d824c - <rustc_symbol_mangling::v0::SymbolMangler as rustc_middle::ty::print::Printer>::print_def_path::h532851c326efa497
  24:     0x7f28449d94d7 - <rustc_symbol_mangling::v0::SymbolMangler as rustc_middle::ty::print::Printer>::print_impl_path::habaffdd820484912
  25:     0x7f28449d7f4d - <rustc_symbol_mangling::v0::SymbolMangler as rustc_middle::ty::print::Printer>::print_def_path::h532851c326efa497
  26:     0x7f28449d87fc - <rustc_symbol_mangling::v0::SymbolMangler as rustc_middle::ty::print::Printer>::print_def_path::h532851c326efa497
  27:     0x7f28449d6b69 - rustc_symbol_mangling::v0::mangle::h0a8cac0b6256ab14
  28:     0x7f28449c7fda - rustc_symbol_mangling::symbol_name_provider::hbed96fd29036464b
  29:     0x7f2845c062cd - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::symbol_name>::compute::hed8d1ce9b7bdb5d0
  30:     0x7f2845e9c863 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task::h87c5e7129ab7c736
  31:     0x7f284597e0be - rustc_data_structures::stack::ensure_sufficient_stack::hd27156d5d08320de
  32:     0x7f2845cd560f - rustc_query_system::query::plumbing::get_query_impl::hae92309e9ab35d1a
  33:     0x7f2845beb9dd - rustc_middle::mir::mono::MonoItem::symbol_name::h4e1d33cdccb70a70
  34:     0x7f28442c79b1 - <alloc::vec::Vec<T> as alloc::vec::SpecFromIter<T,I>>::from_iter::h156d84d0271736c5
  35:     0x7f284454d4b3 - rustc_mir::monomorphize::partitioning::assert_symbols_are_distinct::h310c033f832b90f1
  36:     0x7f28445e0625 - rustc_data_structures::sync::join::h984f166973574fde
  37:     0x7f284454dac6 - rustc_mir::monomorphize::partitioning::collect_and_partition_mono_items::h4f6973e15951cf11
  38:     0x7f28432765d2 - rustc_middle::ty::query::<impl rustc_query_system::query::config::QueryAccessors<rustc_middle::ty::context::TyCtxt> for rustc_middle::ty::query::queries::collect_and_partition_mono_items>::compute::hc67938a3c3133947
  39:     0x7f28431b179e - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::h2ca6615b86509346
  40:     0x7f28431e7f19 - rustc_data_structures::stack::ensure_sufficient_stack::h4535a53c7273efc1
  41:     0x7f284312593c - rustc_query_system::query::plumbing::get_query_impl::hecc9be70aeb29bd5
  42:     0x7f28432903cc - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::h84729c6a9074acd9
  43:     0x7f284301684e - rustc_session::utils::<impl rustc_session::session::Session>::time::he9da96abd57eec9b
  44:     0x7f28430580bc - rustc_interface::passes::QueryContext::enter::hf96a740ef8e4de64
  45:     0x7f28430b2383 - rustc_interface::queries::Queries::ongoing_codegen::h8469c6700fd7a0f4
  46:     0x7f2842d9dae9 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hed31e089d4b6eb55
  47:     0x7f2842e1e557 - rustc_span::with_source_map::h179240a3f032f8c8
  48:     0x7f2842d9f1fb - rustc_interface::interface::create_compiler_and_run::hb67b5170c4ead91b
  49:     0x7f2842e4b650 - scoped_tls::ScopedKey<T>::set::h3794ca08c112be1c
  50:     0x7f2842e52516 - std::sys_common::backtrace::__rust_begin_short_backtrace::he3bd2885a25fc3ec
  51:     0x7f2842da67ca - core::ops::function::FnOnce::call_once{{vtable.shim}}::he39fdb958d398dc2
  52:     0x7f284254f61a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hea1090dbdcecbf5a
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/alloc/src/boxed.rs:1328:9
  53:     0x7f284254f61a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h8d5723d3912bd325
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/alloc/src/boxed.rs:1328:9
  54:     0x7f284254f61a - std::sys::unix::thread::Thread::new::thread_start::hc17a425ca2995724
                               at /rustc/e15ec667cee92d47c64fc903227b2fdb81f9e530/library/std/src/sys/unix/thread.rs:71:17
  55:     0x7f28424403e9 - start_thread
  56:     0x7f284235d293 - __GI___clone
  57:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.50.0-nightly (e15ec667c 2020-12-15) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z instrument-coverage

query stack during panic:
#0 [symbol_name] computing the symbol for `<Foo as Get<"int">>::get`
#1 [collect_and_partition_mono_items] collect_and_partition_mono_items
end of query stack
error: aborting due to previous error
