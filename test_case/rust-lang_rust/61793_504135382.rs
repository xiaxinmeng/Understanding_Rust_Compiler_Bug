
thread 'rustc' panicked at 'assertion failed: target_offset >= offset', src\librustc_codegen_llvm\type_of.rs:130:9
stack backtrace:
   0: backtrace::backtrace::trace_unsynchronized<closure>
             at C:\Users\udoprog\.cargo\registry\src\github.com-1ecc6299db9ec823\backtrace-0.3.29\src\backtrace\mod.rs:66
   1: std::sys_common::backtrace::_print
             at D:\repo\rust\src\libstd\sys_common\backtrace.rs:47
   2: std::sys_common::backtrace::print
             at D:\repo\rust\src\libstd\sys_common\backtrace.rs:36
   3: std::panicking::default_hook::{{closure}}
             at D:\repo\rust\src\libstd\panicking.rs:198
   4: std::panicking::default_hook
             at D:\repo\rust\src\libstd\panicking.rs:212
   5: alloc::boxed::{{impl}}::call
             at D:\repo\rust\src\liballoc\boxed.rs:760
   6: rustc::util::common::panic_hook
             at D:\repo\rust\src\librustc\util\common.rs:40
   7: std::panicking::rust_panic_with_hook
             at D:\repo\rust\src\libstd\panicking.rs:479
   8: std::panicking::begin_panic<str*>
             at D:\repo\rust\src\libstd\panicking.rs:409
   9: rustc_codegen_llvm::type_of::struct_llfields
             at D:\repo\rust\src\librustc_codegen_llvm\type_of.rs:130
  10: rustc_codegen_llvm::type_of::{{impl}}::llvm_type
             at D:\repo\rust\src\librustc_codegen_llvm\type_of.rs:283
  11: rustc_codegen_llvm::type_of::struct_llfields
             at D:\repo\rust\src\librustc_codegen_llvm\type_of.rs:137
  12: rustc_codegen_llvm::type_of::{{impl}}::llvm_type
             at D:\repo\rust\src\librustc_codegen_llvm\type_of.rs:283
  13: rustc_codegen_llvm::type_::{{impl}}::backend_type
             at D:\repo\rust\src\librustc_codegen_llvm\type_.rs:316
  14: rustc_codegen_ssa::mir::place::PlaceRef<rustc_codegen_llvm::llvm_::ffi::::Value*>::alloca<rustc_codegen_llvm::llvm_::ffi::::Value*,rustc_codegen_llvm::builder::Builder>
             at D:\repo\rust\src\librustc_codegen_ssa\mir\place.rs:66
  15: core::iter::adapters::{{impl}}::fold::{{closure}}
             at D:\repo\rust\src\libcore\iter\adapters\mod.rs:589
  16: core::iter::adapters::{{impl}}::fold::{{closure}}
             at D:\repo\rust\src\libcore\iter\adapters\mod.rs:589
  17: core::iter::traits::iterator::Iterator::fold::{{closure}}
             at D:\repo\rust\src\libcore\iter\traits\iterator.rs:1685
  18: core::iter::traits::iterator::Iterator::try_fold
             at D:\repo\rust\src\libcore\iter\traits\iterator.rs:1573
  19: core::iter::traits::iterator::Iterator::fold
             at D:\repo\rust\src\libcore\iter\traits\iterator.rs:1685
  20: core::iter::adapters::{{impl}}::fold
             at D:\repo\rust\src\libcore\iter\adapters\mod.rs:589
  21: core::iter::adapters::{{impl}}::fold<rustc_codegen_ssa::mir::LocalRef<rustc_codegen_llvm::llvm_::ffi::::Value*>,core::iter::adapters::Map<core::ops::range::Range<usize>, fn(usize) -> rustc::mir::Local>,closure,(),mut closure*> 
             at D:\repo\rust\src\libcore\iter\adapters\mod.rs:589
  22: core::iter::adapters::chain::{{impl}}::fold<core::iter::adapters::chain::Chain<core::iter::sources::Once<rustc_codegen_ssa::mir::LocalRef<rustc_codegen_llvm::llvm_::ffi::::Value*>>, alloc::vec::IntoIter<rustc_codegen_ssa::mir::LocalRef<rustc_codegen_llvm:
             at D:\repo\rust\src\libcore\iter\adapters\chain.rs:113
  23: alloc::vec::{{impl}}::from_iter<rustc_codegen_ssa::mir::LocalRef<rustc_codegen_llvm::llvm_::ffi::::Value*>,core::iter::adapters::chain::Chain<core::iter::adapters::chain::Chain<core::iter::sources::Once<rustc_codegen_ssa::mir::LocalRef<rustc_codegen_llvm:
             at D:\repo\rust\src\liballoc\vec.rs:1909
  24: alloc::vec::{{impl}}::from_iter
             at D:\repo\rust\src\liballoc\vec.rs:1796
  25: rustc_data_structures::indexed_vec::{{impl}}::from_iter
             at D:\repo\rust\src\librustc_data_structures\indexed_vec.rs:764
  26: core::iter::traits::iterator::Iterator::collect
             at D:\repo\rust\src\libcore\iter\traits\iterator.rs:1466
  27: rustc_codegen_ssa::mir::codegen_mir<rustc_codegen_llvm::builder::Builder>
             at D:\repo\rust\src\librustc_codegen_ssa\mir\mod.rs:316
  28: rustc_codegen_ssa::base::codegen_instance<rustc_codegen_llvm::builder::Builder>
             at D:\repo\rust\src\librustc_codegen_ssa\base.rs:385
  29: rustc_codegen_ssa::mono_item::{{impl}}::define<rustc_codegen_llvm::builder::Builder>
             at D:\repo\rust\src\librustc_codegen_ssa\mono_item.rs:40
  30: rustc_codegen_llvm::base::compile_codegen_unit::module_codegen
             at D:\repo\rust\src\librustc_codegen_llvm\base.rs:143
  31: rustc::dep_graph::graph::DepGraph::with_task<rustc::ty::context::TyCtxt,syntax_pos::symbol::InternedString,rustc_codegen_ssa::ModuleCodegen<rustc_codegen_llvm::ModuleLlvm>,fn(mut rustc::ich::hcx::StableHashingContext*, rustc_codegen_ssa::ModuleCodegen<rus
             at D:\repo\rust\src\librustc\dep_graph\graph.rs:201
  32: rustc_codegen_llvm::base::compile_codegen_unit
             at D:\repo\rust\src\librustc_codegen_llvm\base.rs:110
  33: rustc_codegen_ssa::base::codegen_crate<rustc_codegen_llvm::LlvmCodegenBackend>
             at D:\repo\rust\src\librustc_codegen_ssa\base.rs:616
  34: rustc_codegen_llvm::{{impl}}::codegen_crate
             at D:\repo\rust\src\librustc_codegen_llvm\lib.rs:292
  35: rustc_interface::passes::start_codegen::{{closure}}
             at D:\repo\rust\src\librustc_interface\passes.rs:1081
  36: rustc::util::common::time_ext
             at D:\repo\rust\src\librustc\util\common.rs:151
  37: rustc::util::common::time<alloc::boxed::Box<Any>,closure>
             at D:\repo\rust\src\librustc\util\common.rs:145
  38: rustc_interface::passes::start_codegen
             at D:\repo\rust\src\librustc_interface\passes.rs:1080
  39: rustc::ty::context::tls::enter_global<closure,core::result::Result<alloc::boxed::Box<Any>, rustc::util::common::ErrorReported>>
             at D:\repo\rust\src\librustc\ty\context.rs:1962
  40: rustc_interface::passes::{{impl}}::access::{{closure}}<closure,core::result::Result<alloc::boxed::Box<Any>, rustc::util::common::ErrorReported>>
             at <::rustc_data_structures::box_region::declare_box_region_type macros>:17
  41: rustc_interface::passes::create_global_ctxt::{{closure}}
             at D:\repo\rust\src\librustc_interface\passes.rs:869
  42: rustc_interface::passes::BoxedGlobalCtxt::access
             at <::rustc_data_structures::box_region::declare_box_region_type macros>:19
  43: rustc_interface::passes::BoxedGlobalCtxt::enter<closure,core::result::Result<alloc::boxed::Box<Any>, rustc::util::common::ErrorReported>>
             at D:\repo\rust\src\librustc_interface\passes.rs:803
  44: rustc_interface::queries::{{impl}}::ongoing_codegen::{{closure}}
             at D:\repo\rust\src\librustc_interface\queries.rs:249
  45: rustc_interface::queries::Query<alloc::boxed::Box<Any>>::compute<alloc::boxed::Box<Any>,closure>
             at D:\repo\rust\src\librustc_interface\queries.rs:40
  46: rustc_interface::interface::Compiler::ongoing_codegen
             at D:\repo\rust\src\librustc_interface\queries.rs:246
  47: rustc_driver::run_compiler::{{closure}}
             at D:\repo\rust\src\librustc_driver\lib.rs:353
  48: rustc_interface::interface::run_compiler_in_existing_thread_pool<closure,core::result::Result<(), rustc::util::common::ErrorReported>>
             at D:\repo\rust\src\librustc_interface\interface.rs:123
  49: rustc_interface::interface::run_compiler::{{closure}}
             at D:\repo\rust\src\librustc_interface\interface.rs:142
  50: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}::{{closure}}
             at D:\repo\rust\src\librustc_interface\util.rs:192
  51: rustc::ty::context::tls::with_thread_locals::{{closure}}::{{closure}}
             at D:\repo\rust\src\librustc\ty\context.rs:1917
  52: std::thread::local::LocalKey<core::cell::Cell<fn(rustc_errors::diagnostic::Diagnostic*)>>::try_with
             at D:\repo\rust\src\libstd\thread\local.rs:299
  53: std::thread::local::LocalKey<core::cell::Cell<fn(rustc_errors::diagnostic::Diagnostic*)>>::with
             at D:\repo\rust\src\libstd\thread\local.rs:245
  54: rustc::ty::context::tls::with_thread_locals::{{closure}}
             at D:\repo\rust\src\librustc\ty\context.rs:1909
  55: std::thread::local::LocalKey<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> core::result::Result<(), core::fmt::Error>>>::try_with
             at D:\repo\rust\src\libstd\thread\local.rs:299
  56: std::thread::local::LocalKey<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> core::result::Result<(), core::fmt::Error>>>::with<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> core::res
             at D:\repo\rust\src\libstd\thread\local.rs:245
  57: rustc::ty::context::tls::with_thread_locals
             at D:\repo\rust\src\librustc\ty\context.rs:1901
  58: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}::{{closure}}
             at D:\repo\rust\src\librustc_interface\util.rs:192
  59: scoped_tls::ScopedKey<rustc_data_structures::sync::Lock<usize>>::set
             at C:\Users\udoprog\.cargo\registry\src\github.com-1ecc6299db9ec823\scoped-tls-1.0.0\src\lib.rs:137
  60: rustc_interface::util::spawn_thread_pool::{{closure}}::{{closure}}
             at D:\repo\rust\src\librustc_interface\util.rs:188
  61: scoped_tls::ScopedKey<syntax_pos::Globals>::set
             at C:\Users\udoprog\.cargo\registry\src\github.com-1ecc6299db9ec823\scoped-tls-1.0.0\src\lib.rs:137
  62: syntax::with_globals::{{closure}}
             at D:\repo\rust\src\libsyntax\lib.rs:106
  63: scoped_tls::ScopedKey<syntax::Globals>::set<syntax::Globals,closure,core::result::Result<(), rustc::util::common::ErrorReported>>
             at C:\Users\udoprog\.cargo\registry\src\github.com-1ecc6299db9ec823\scoped-tls-1.0.0\src\lib.rs:137
  64: rustc_interface::util::scoped_thread::{{closure}}
             at D:\repo\rust\src\librustc_interface\util.rs:164
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
query stack during panic:
end of query stack

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.37.0-dev running on x86_64-pc-windows-msvc

note: compiler flags: -C debuginfo=2 -C incremental --crate-type bin

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `test-error`.

To learn more, run the command again with --verbose.
