
2020-03-02T16:32:10.3818914Z error: internal compiler error: src/librustc_codegen_ssa/mir/block.rs:520: `miri_start_panic` should never end up in compiled code
2020-03-02T16:32:10.3819454Z 
2020-03-02T16:32:10.3821168Z thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:885:9
2020-03-02T16:32:10.3821537Z stack backtrace:
2020-03-02T16:32:10.5535378Z    0: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
2020-03-02T16:32:10.5537236Z    1: core::fmt::write
2020-03-02T16:32:10.5537847Z    2: std::io::Write::write_fmt
2020-03-02T16:32:10.5538155Z    3: std::panicking::default_hook::{{closure}}
2020-03-02T16:32:10.5538893Z    4: std::panicking::default_hook
2020-03-02T16:32:10.5542130Z    5: rustc_driver::report_ice
2020-03-02T16:32:10.5546238Z    6: std::panicking::rust_panic_with_hook
2020-03-02T16:32:10.5550337Z    7: std::panicking::begin_panic
2020-03-02T16:32:10.5554388Z    8: rustc_errors::HandlerInner::bug
2020-03-02T16:32:10.5558305Z    9: rustc_errors::Handler::bug
2020-03-02T16:32:10.5562860Z   10: rustc::util::bug::opt_span_bug_fmt::{{closure}}
2020-03-02T16:32:10.5568446Z   11: rustc::ty::context::tls::with_opt::{{closure}}
2020-03-02T16:32:10.5572721Z   12: rustc::ty::context::tls::with_opt
2020-03-02T16:32:10.5613294Z   13: rustc::util::bug::opt_span_bug_fmt
2020-03-02T16:32:10.5613625Z   14: rustc::util::bug::bug_fmt
2020-03-02T16:32:10.5614074Z   15: rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_call_terminator
2020-03-02T16:32:10.5614657Z   16: rustc_codegen_ssa::mir::block::<impl rustc_codegen_ssa::mir::FunctionCx<Bx>>::codegen_block
2020-03-02T16:32:10.5615072Z   17: rustc_codegen_ssa::mir::codegen_mir
2020-03-02T16:32:10.5615488Z   18: <rustc::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define
2020-03-02T16:32:10.5615939Z   19: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
2020-03-02T16:32:10.5616625Z   20: rustc::dep_graph::graph::DepGraph::with_task
2020-03-02T16:32:10.5617160Z   21: rustc_codegen_llvm::base::compile_codegen_unit
2020-03-02T16:32:10.5617525Z   22: rustc_codegen_ssa::base::codegen_crate
2020-03-02T16:32:10.5620002Z   23: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
2020-03-02T16:32:10.5624143Z   24: rustc_interface::passes::start_codegen
2020-03-02T16:32:10.5628697Z   25: rustc::ty::context::tls::enter_global
2020-03-02T16:32:10.5632880Z   26: rustc_interface::queries::Queries::ongoing_codegen
2020-03-02T16:32:10.5637110Z   27: rustc_interface::interface::run_compiler_in_existing_thread_pool
2020-03-02T16:32:10.5641459Z   28: rustc_ast::attr::with_globals
2020-03-02T16:32:10.5646064Z note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
2020-03-02T16:32:10.5646360Z 
2020-03-02T16:32:10.5654225Z note: the compiler unexpectedly panicked. this is a bug.
2020-03-02T16:32:10.5654691Z 
2020-03-02T16:32:10.5661245Z note: we would appreciate a bug report: ***/blob/master/CONTRIBUTING.md#bug-reports
2020-03-02T16:32:10.5661564Z 
2020-03-02T16:32:10.5665967Z note: rustc 1.43.0-nightly (2a4746ca1 2020-03-02) running on x86_64-unknown-linux-gnu
2020-03-02T16:32:10.5666295Z 
2020-03-02T16:32:10.5673655Z note: compiler flags: -Z always-encode-mir -Z mir-emit-retag -Z mir-opt-level=0 -Z force-unstable-if-unmarked -C opt-level=3 -C debug-assertions=on -C debug-assertions=y --crate-type lib
2020-03-02T16:32:10.5674148Z 
2020-03-02T16:32:10.5686968Z note: some of the compiler flags provided by cargo are hidden
