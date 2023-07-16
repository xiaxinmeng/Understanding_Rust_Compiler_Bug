rust
error: internal compiler error: src\librustc_metadata\cstore_impl.rs:131: get_optimized_mir: missing MIR for `DefId(8/0:
243 ~ compiler_builtins[9532]::int[0]::shift[0]::__ashlti3[0])`

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.24.0-dev running on x86_64-pc-windows-msvc

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', src\librustc_errors\lib.rs:501:8
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys::windows::backtrace::unwind_backtrace
             at .\src\libstd\sys\windows\backtrace\mod.rs:65
   1: std::sys_common::backtrace::print
             at .\src\libstd\sys_common\backtrace.rs:56
   2: std::panicking::default_hook::{{closure}}
             at .\src\libstd\panicking.rs:381
   3: std::panicking::default_hook
             at .\src\libstd\panicking.rs:391
   4: std::panicking::rust_panic_with_hook
             at .\src\libstd\panicking.rs:577
   5: std::panicking::begin_panic<rustc_errors::ExplicitBug>
             at .\src\libstd\panicking.rs:538
   6: rustc_errors::Handler::bug
             at .\src\librustc_errors\lib.rs:501
   7: std::thread::local::LocalKey<core::cell::Cell<core::option::Option<(const rustc::ty::context::tls::ThreadLocalGlob
alCtxt*, const rustc::ty::context::tls::ThreadLocalInterners*)>>>::with<core::cell::Cell<core::option::Option<(const rus
tc::ty::context::tls::ThreadLocalGlobalCtxt*, const rustc::ty::context::tls::ThreadLocalInterners*)>>,closure,!>
             at .\src\libstd\thread\local.rs:288
   8: rustc::ty::context::tls::with_opt<closure,!>
             at .\src\librustc\ty\context.rs:1478
   9: rustc::session::opt_span_bug_fmt<syntax_pos::span_encoding::Span>
             at .\src\librustc\session\mod.rs:1043
  10: rustc::session::bug_fmt
             at .\src\librustc\session\mod.rs:1027
  11: rustc_metadata::cstore_impl::provide_extern::optimized_mir<rustc::hir::def_id::DefId>
             at .\src\librustc_metadata\cstore_impl.rs:71
  12: rustc::ty::maps::queries::optimized_mir::compute_result
             at .\src\librustc\ty\maps\plumbing.rs:383
  13: rustc::dep_graph::graph::DepGraph::with_task_impl<rustc::ty::context::TyCtxt,rustc::hir::def_id::DefId,rustc::mir:
:Mir*,rustc::ich::hcx::StableHashingContext>
             at .\src\librustc\dep_graph\graph.rs:273
  14: rustc_errors::Handler::track_diagnostics<closure,(rustc::mir::Mir*, rustc::dep_graph::graph::DepNodeIndex)>
             at .\src\librustc_errors\lib.rs:564
  15: rustc::ty::context::TyCtxt::cycle_check<closure,((rustc::mir::Mir*, rustc::dep_graph::graph::DepNodeIndex), alloc:
:vec::Vec<rustc_errors::diagnostic::Diagnostic>)>
             at .\src\librustc\ty\maps\plumbing.rs:121
  16: rustc::ty::maps::queries::optimized_mir::force
             at .\src\librustc\ty\maps\plumbing.rs:484
  17: rustc::ty::maps::queries::optimized_mir::try_get
             at .\src\librustc\ty\maps\plumbing.rs:524
  18: rustc::ty::maps::TyCtxtAt::optimized_mir
             at .\src\librustc\ty\maps\plumbing.rs:565
  19: rustc::ty::context::TyCtxt::instance_mir
             at .\src\librustc\ty\mod.rs:2334
  20: rustc_trans::mir::constant::MirConstContext::trans_def
             at .\src\librustc_trans\mir\constant.rs:302
  21: rustc_trans::mir::constant::MirConstContext::trans
             at .\src\librustc_trans\mir\constant.rs:415
  22: rustc_trans::mir::constant::MirConstContext::trans_def
             at .\src\librustc_trans\mir\constant.rs:303
  23: rustc_trans::mir::constant::MirConstContext::trans
             at .\src\librustc_trans\mir\constant.rs:415
  24: rustc_trans::mir::MirContext::trans_constant
             at .\src\librustc_trans\mir\constant.rs:1064
  25: rustc_trans::mir::MirContext::trans_operand
             at .\src\librustc_trans\mir\operand.rs:317
  26: rustc_trans::mir::MirContext::trans_rvalue_operand
             at .\src\librustc_trans\mir\rvalue.rs:473
  27: rustc_trans::mir::trans_mir
             at .\src\librustc_trans\mir\mod.rs:320
  28: rustc_trans::base::trans_instance
             at .\src\librustc_trans\base.rs:505
  29: rustc_trans::trans_item::TransItemExt::define<rustc::middle::trans::TransItem>
             at .\src\librustc_trans\trans_item.rs:73
  30: rustc_trans::base::compile_codegen_unit
             at .\src\librustc_trans\base.rs:1161
  31: rustc::ty::maps::queries::compile_codegen_unit::compute_result
             at .\src\librustc\ty\maps\plumbing.rs:383
  32: rustc::dep_graph::graph::DepGraph::with_task_impl<rustc::ty::context::TyCtxt,syntax_pos::symbol::InternedString,ru
stc::middle::trans::Stats,rustc::ich::hcx::StableHashingContext>
             at .\src\librustc\dep_graph\graph.rs:273
  33: rustc_errors::Handler::track_diagnostics<closure,(rustc::middle::trans::Stats, rustc::dep_graph::graph::DepNodeInd
ex)>
             at .\src\librustc_errors\lib.rs:564
  34: rustc::ty::context::TyCtxt::cycle_check<closure,((rustc::middle::trans::Stats, rustc::dep_graph::graph::DepNodeInd
ex), alloc::vec::Vec<rustc_errors::diagnostic::Diagnostic>)>
             at .\src\librustc\ty\maps\plumbing.rs:121
  35: rustc::ty::maps::queries::compile_codegen_unit::force
             at .\src\librustc\ty\maps\plumbing.rs:484
  36: rustc::ty::maps::queries::compile_codegen_unit::try_get
             at .\src\librustc\ty\maps\plumbing.rs:524
  37: rustc::ty::maps::TyCtxtAt::compile_codegen_unit
             at .\src\librustc\ty\maps\plumbing.rs:565
  38: rustc::ty::context::TyCtxt::compile_codegen_unit
             at .\src\librustc\ty\maps\plumbing.rs:558
  39: rustc_trans::base::trans_crate
             at .\src\librustc_trans\base.rs:888
  40: rustc_trans::{{impl}}::trans_crate
             at .\src\librustc_trans\lib.rs:181
  41: rustc_driver::driver::phase_4_translate_to_llvm<rustc_trans::LlvmTransCrate>
             at .\src\librustc_driver\driver.rs:1113
  42: rustc_driver::driver::compile_input::{{closure}}
             at .\src\librustc_driver\driver.rs:249
  43: std::thread::local::LocalKey<core::cell::Cell<core::option::Option<(const rustc::ty::context::tls::ThreadLocalGlob
alCtxt*, const rustc::ty::context::tls::ThreadLocalInterners*)>>>::with<core::cell::Cell<core::option::Option<(const rus
tc::ty::context::tls::ThreadLocalGlobalCtxt*, const rustc::ty::context::tls::ThreadLocalInterners*)>>,closure,core::resu
lt::Result<core::result::Result<(rustc::session::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslat
ion, rustc::dep_graph::graph::DepGraph), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
             at .\src\libstd\thread\local.rs:288
  44: std::thread::local::LocalKey<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> co
re::result::Result<(), core::fmt::Error>>>::with<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::For
matter*) -> core::result::Result<(), core::fmt::Error>>,closure,core::result::Result<core::result::Result<(rustc::sessio
n::config::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation, rustc::dep_graph::graph::DepGraph), rustc
::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
             at .\src\libstd\thread\local.rs:288
  45: rustc::ty::context::TyCtxt::create_and_enter<closure,core::result::Result<core::result::Result<(rustc::session::co
nfig::OutputFilenames, rustc_trans::back::write::OngoingCrateTranslation, rustc::dep_graph::graph::DepGraph), rustc::ses
sion::CompileIncomplete>, rustc::session::CompileIncomplete>>
             at .\src\librustc\ty\context.rs:1069
  46: rustc_driver::driver::compile_input
             at .\src\librustc_driver\driver.rs:216
  47: rustc_driver::run_compiler
             at .\src\librustc_driver\lib.rs:253
