
error: internal compiler error: /rustc/30b3f35c420694a4f24e5a4df00f06073f4f3a37/compiler/rustc_codegen_ssa/src/mir/constant.rs:42:20: encountered bad ConstKind after monomorphizing: Error(DelaySpanBugEmitted(()))
  --> src/lib.rs:99:9
   |
99 |         dbg!(line!());
   |         ^^^^^^^^^^^^^
   |
   = note: this error: internal compiler error originates in the macro `dbg` (in Nightly builds, run with -Z macro-backtrace for more info)

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/30b3f35c420694a4f24e5a4df00f06073f4f3a37/compiler/rustc_errors/src/lib.rs:1106:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
   2: <rustc_errors::HandlerInner>::span_bug::<rustc_span::span_encoding::Span>
   3: <rustc_errors::Handler>::span_bug::<rustc_span::span_encoding::Span>
   4: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>
   5: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
   6: rustc_middle::util::bug::span_bug_fmt::<rustc_span::span_encoding::Span>
   7: <rustc_codegen_ssa::mir::FunctionCx<rustc_codegen_llvm::builder::Builder>>::codegen_rvalue_operand
   8: rustc_codegen_ssa::mir::codegen_mir::<rustc_codegen_llvm::builder::Builder>
   9: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
  10: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>
  11: rustc_codegen_llvm::base::compile_codegen_unit
  12: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
  13: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
  14: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
  15: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>
  16: <rustc_interface::queries::Queries>::ongoing_codegen
  17: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-nightly (30b3f35c4 2022-02-17) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
warning: `reproduce_ice` (lib test) generated 3 warnings
error: could not compile `reproduce_ice`; 3 warnings emitted
The terminal process "cargo 'test', '--package', 'reproduce_ice', '--lib', '--', 'test', '--exact', '--nocapture'" terminated with exit code: 101.
