
Assembling stage1 compiler (x86_64-unknown-linux-gnu)
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling compiler_builtins v0.1.66
   Compiling core v0.0.0 (/home/woppopo/Repo/rust/library/core)
   Compiling libc v0.2.108
   Compiling cc v1.0.69
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/home/woppopo/Repo/rust/library/std)
   Compiling unwind v0.0.0 (/home/woppopo/Repo/rust/library/unwind)
   Compiling rustc-std-workspace-core v1.99.0 (/home/woppopo/Repo/rust/library/rustc-std-workspace-core)
error: internal compiler error: compiler/rustc_codegen_llvm/src/intrinsic.rs:371:18: unknown intrinsic 'const_allocate'

thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
stack backtrace:
   0: std::panicking::begin_panic::<rustc_errors::ExplicitBug>
             at ./library/std/src/panicking.rs:609:12
   1: std::panic::panic_any::<rustc_errors::ExplicitBug>
             at ./library/std/src/panic.rs:60:5
   2: <rustc_errors::HandlerInner>::bug
             at ./compiler/rustc_errors/src/lib.rs:1169:9
   3: <rustc_errors::Handler>::bug
             at ./compiler/rustc_errors/src/lib.rs:864:9
   4: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>::{closure#0}
             at ./compiler/rustc_middle/src/util/bug.rs:34:34
   5: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>::{closure#0}
             at ./compiler/rustc_middle/src/ty/context.rs:1836:40
   6: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_opt<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>::{closure#0}, ()>
             at ./compiler/rustc_middle/src/ty/context.rs:1788:22
   7: rustc_middle::ty::context::tls::with_opt::<rustc_middle::util::bug::opt_span_bug_fmt<rustc_span::span_encoding::Span>::{closure#0}, ()>
             at ./compiler/rustc_middle/src/ty/context.rs:1836:9
   8: rustc_middle::util::bug::opt_span_bug_fmt::<rustc_span::span_encoding::Span>
             at ./compiler/rustc_middle/src/util/bug.rs:30:5
   9: rustc_middle::util::bug::bug_fmt
             at ./compiler/rustc_middle/src/util/bug.rs:14:5
  10: <rustc_codegen_llvm::builder::Builder as rustc_codegen_ssa::traits::intrinsic::IntrinsicCallMethods>::codegen_intrinsic_call
  11: <rustc_codegen_ssa::mir::FunctionCx<rustc_codegen_llvm::builder::Builder>>::codegen_intrinsic_call
             at ./compiler/rustc_codegen_ssa/src/mir/intrinsic.rs:571:17
  12: <rustc_codegen_ssa::mir::FunctionCx<rustc_codegen_llvm::builder::Builder>>::codegen_call_terminator
             at ./compiler/rustc_codegen_ssa/src/mir/block.rs:748:17
  13: <rustc_codegen_ssa::mir::FunctionCx<rustc_codegen_llvm::builder::Builder>>::codegen_terminator
             at ./compiler/rustc_codegen_ssa/src/mir/block.rs:1080:17
  14: <rustc_codegen_ssa::mir::FunctionCx<rustc_codegen_llvm::builder::Builder>>::codegen_block
             at ./compiler/rustc_codegen_ssa/src/mir/block.rs:1019:9
  15: rustc_codegen_ssa::mir::codegen_mir::<rustc_codegen_llvm::builder::Builder>
             at ./compiler/rustc_codegen_ssa/src/mir/mod.rs:246:9
  16: rustc_codegen_ssa::base::codegen_instance::<rustc_codegen_llvm::builder::Builder>
             at ./compiler/rustc_codegen_ssa/src/base.rs:361:5
  17: <rustc_middle::mir::mono::MonoItem as rustc_codegen_ssa::mono_item::MonoItemExt>::define::<rustc_codegen_llvm::builder::Builder>
             at ./compiler/rustc_codegen_ssa/src/mono_item.rs:69:17
  18: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
             at ./compiler/rustc_codegen_llvm/src/base.rs:92:17
  19: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:326:53
  20: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}::{closure#0}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:55:46
  21: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}
             at ./compiler/rustc_middle/src/ty/context.rs:1771:50
  22: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>
             at ./compiler/rustc_middle/src/ty/context.rs:1755:9
  23: rustc_middle::ty::context::tls::enter_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>
             at ./compiler/rustc_middle/src/ty/context.rs:1771:9
  24: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:55:13
  25: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}
             at ./compiler/rustc_middle/src/ty/context.rs:1799:40
  26: rustc_middle::ty::context::tls::with_context_opt::<rustc_middle::ty::context::tls::with_context<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>
             at ./compiler/rustc_middle/src/ty/context.rs:1788:22
  27: rustc_middle::ty::context::tls::with_context::<<rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>
             at ./compiler/rustc_middle/src/ty/context.rs:1799:9
  28: <rustc_middle::dep_graph::dep_node::DepKind as rustc_query_system::dep_graph::DepKind>::with_deps::<<rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>::{closure#0}, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>
             at ./compiler/rustc_middle/src/dep_graph/mod.rs:52:9
  29: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task_impl::<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:326:22
  30: <rustc_query_system::dep_graph::graph::DepGraph<rustc_middle::dep_graph::dep_node::DepKind>>::with_task::<rustc_middle::ty::context::TyCtxt, rustc_span::symbol::Symbol, rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>>
             at ./compiler/rustc_query_system/src/dep_graph/graph.rs:274:13
  31: rustc_codegen_llvm::base::compile_codegen_unit
             at ./compiler/rustc_codegen_llvm/src/base.rs:62:23
  32: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::compile_codegen_unit
             at ./compiler/rustc_codegen_llvm/src/lib.rs:118:9
  33: rustc_codegen_ssa::base::codegen_crate::<rustc_codegen_llvm::LlvmCodegenBackend>
             at ./compiler/rustc_codegen_ssa/src/base.rs:684:38
  34: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate
             at ./compiler/rustc_codegen_llvm/src/lib.rs:328:18
  35: rustc_interface::passes::start_codegen::{closure#0}
             at ./compiler/rustc_interface/src/passes.rs:1093:9
  36: <rustc_data_structures::profiling::VerboseTimingGuard>::run::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
             at ./compiler/rustc_data_structures/src/profiling.rs:644:9
  37: <rustc_session::session::Session>::time::<alloc::boxed::Box<dyn core::any::Any>, rustc_interface::passes::start_codegen::{closure#0}>
             at ./compiler/rustc_session/src/utils.rs:16:9
  38: rustc_interface::passes::start_codegen
             at ./compiler/rustc_interface/src/passes.rs:1092:19
  39: <rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}
             at ./compiler/rustc_interface/src/queries.rs:254:20
  40: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}
             at ./compiler/rustc_interface/src/passes.rs:821:42
  41: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}
             at ./compiler/rustc_middle/src/ty/context.rs:1771:50
  42: rustc_middle::ty::context::tls::set_tlv::<rustc_middle::ty::context::tls::enter_context<<rustc_interface::passes::QueryContext>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>
             at ./compiler/rustc_middle/src/ty/context.rs:1755:9
  43: rustc_middle::ty::context::tls::enter_context::<<rustc_interface::passes::QueryContext>::enter<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>
             at ./compiler/rustc_middle/src/ty/context.rs:1771:9
  44: <rustc_interface::passes::QueryContext>::enter::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core::result::Result<alloc::boxed::Box<dyn core::any::Any>, rustc_errors::ErrorReported>>
             at ./compiler/rustc_interface/src/passes.rs:821:9
  45: <rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}
             at ./compiler/rustc_interface/src/queries.rs:245:13
  46: <rustc_interface::queries::Query<alloc::boxed::Box<dyn core::any::Any>>>::compute::<<rustc_interface::queries::Queries>::ongoing_codegen::{closure#0}>
             at ./compiler/rustc_interface/src/queries.rs:38:28
  47: rustc_driver::run_compiler::{closure#1}::{closure#2}
             at ./compiler/rustc_driver/src/lib.rs:410:13
  48: <rustc_interface::interface::Compiler>::enter::<rustc_driver::run_compiler::{closure#1}::{closure#2}, core::result::Result<core::option::Option<rustc_interface::queries::Linker>, rustc_errors::ErrorReported>>
             at ./compiler/rustc_interface/src/queries.rs:393:19
  49: rustc_driver::run_compiler::{closure#1}
             at ./compiler/rustc_driver/src/lib.rs:315:22
  50: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}
             at ./compiler/rustc_interface/src/interface.rs:229:13
  51: rustc_span::with_source_map::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::interface::create_compiler_and_run<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#1}>
             at ./compiler/rustc_span/src/lib.rs:1011:5
  52: rustc_interface::interface::create_compiler_and_run::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>
             at ./compiler/rustc_interface/src/interface.rs:223:5
  53: rustc_interface::interface::run_compiler::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}
             at ./compiler/rustc_interface/src/interface.rs:245:12
  54: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}
             at ./compiler/rustc_interface/src/util.rs:148:13
  55: <scoped_tls::ScopedKey<rustc_span::SessionGlobals>>::set::<rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>
             at /home/woppopo/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-1.0.0/src/lib.rs:137:9
  56: rustc_span::create_session_globals_then::<core::result::Result<(), rustc_errors::ErrorReported>, rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}::{closure#0}>
             at ./compiler/rustc_span/src/lib.rs:109:5
  57: rustc_interface::util::setup_callbacks_and_run_in_thread_pool_with_globals::<rustc_interface::interface::run_compiler<core::result::Result<(), rustc_errors::ErrorReported>, rustc_driver::run_compiler::{closure#1}>::{closure#0}, core::result::Result<(), rustc_errors::ErrorReported>>::{closure#0}
             at ./compiler/rustc_interface/src/util.rs:146:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.60.0-dev running on x86_64-unknown-linux-gnu

note: compiler flags: -Z unstable-options -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=1 -C debug-assertions=on -C incremental -C symbol-mangling-version=legacy -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
error: could not compile `core`
warning: build failed, waiting for other jobs to finish...
error: build failed
Build completed unsuccessfully in 0:04:47
