
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-dev running on x86_64-pc-windows-msvc

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'missing specialization: `<encoder::EncodeContext as SpecializedEncoder<rustc::hir::def_id::LocalDefId>>::specialized_encode` not overridden', libseriali
ze\serialize.rs:788:9
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
stack backtrace:
   0: std::sys_common::backtrace::print
             at D:\rust-lang\rust\src\libstd\sys_common\backtrace.rs:61
   1: std::panicking::default_hook::{{closure}}
             at D:\rust-lang\rust\src\libstd\panicking.rs:380
   2: std::panicking::default_hook
             at D:\rust-lang\rust\src\libstd\panicking.rs:390
   3: std::panicking::rust_panic_with_hook
             at D:\rust-lang\rust\src\libstd\panicking.rs:576
   4: std::panicking::begin_panic<alloc::string::String>
             at D:\rust-lang\rust\src\libstd\panicking.rs:537
   5: std::panicking::begin_panic_fmt
             at D:\rust-lang\rust\src\libstd\panicking.rs:521
   6: serialize::serialize::{{impl}}::not_found<std::io::error::Error,rustc_metadata::encoder::EncodeContext,rustc::hir::def_id::LocalDefId>
             at \<panic macros>:7
   7: serialize::serialize::Encoder::emit_map<rustc_metadata::encoder::EncodeContext,closure>
             at D:\rust-lang\rust\src\libserialize\serialize.rs:131
   8: serialize::serialize::Encoder::emit_struct<rustc_metadata::encoder::EncodeContext,closure>
             at D:\rust-lang\rust\src\libserialize\serialize.rs:83
   9: rustc::ty::context::{{impl}}::encode<rustc_metadata::encoder::EncodeContext>
             at D:\rust-lang\rust\src\librustc\ty\context.rs:334
  10: rustc_metadata::encoder::EncodeContext::lazy<rustc::ty::context::TypeckTables>
             at D:\rust-lang\rust\src\librustc_metadata\encoder.rs:248
  11: rustc_metadata::isolated_encoder::IsolatedEncoder::encode_body
             at D:\rust-lang\rust\src\librustc_metadata\astencode.rs:58
  12: rustc_metadata::isolated_encoder::IsolatedEncoder::encode_info_for_item
             at D:\rust-lang\rust\src\librustc_metadata\encoder.rs:1113
  13: rustc_metadata::encoder::{{impl}}::visit_item
             at D:\rust-lang\rust\src\librustc_metadata\encoder.rs:1474
  14: rustc::hir::Crate::visit_all_item_likes<rustc::hir::itemlikevisit::DeepVisitor<rustc_metadata::encoder::EncodeVisitor>>
             at D:\rust-lang\rust\src\librustc\hir\mod.rs:647
  15: rustc_metadata::encoder::encode_metadata
             at D:\rust-lang\rust\src\librustc_metadata\encoder.rs:1677
  16: rustc_metadata::cstore_impl::{{impl}}::encode_metadata
             at D:\rust-lang\rust\src\librustc_metadata\cstore_impl.rs:526
  17: rustc::ty::context::TyCtxt::encode_metadata
             at D:\rust-lang\rust\src\librustc\ty\context.rs:1371
  18: rustc_trans::base::write_metadata
             at D:\rust-lang\rust\src\librustc_trans\base.rs:646
  19: rustc::util::common::time<(mut rustc_llvm::ffi::Context_opaque*, mut rustc_llvm::ffi::Module_opaque*, rustc::middle::cstore::EncodedMetadata),closure>
             at D:\rust-lang\rust\src\librustc\util\common.rs:120
  20: rustc_trans::base::trans_crate
             at D:\rust-lang\rust\src\librustc_trans\base.rs:726
  21: rustc_trans::{{impl}}::trans_crate
             at D:\rust-lang\rust\src\librustc_trans\lib.rs:210
  22: rustc::util::common::time<alloc::boxed::Box<Any>,closure>
             at D:\rust-lang\rust\src\librustc\util\common.rs:120
  23: rustc_driver::driver::phase_4_translate_to_llvm
             at D:\rust-lang\rust\src\librustc_driver\driver.rs:1088
  24: rustc_driver::driver::compile_input::{{closure}}
             at D:\rust-lang\rust\src\librustc_driver\driver.rs:238
  25: std::thread::local::LocalKey<core::cell::Cell<core::option::Option<(const rustc::ty::context::tls::ThreadLocalGlobalCtxt*, const rustc::ty::context::tls::ThreadLocalInterners
*)>>>::with<core::cell::Cell<core::option::Option<(const rustc::ty::context::tls::ThreadLocalGlobalCtxt*, const rustc::ty::context::tls::ThreadLocalInterners*)>>,closure,core::resu
lt::Result<core::result::Result<(rustc::session::config::OutputFilenames, alloc::boxed::Box<Any>, rustc::dep_graph::graph::DepGraph), rustc::session::CompileIncomplete>, rustc::ses
sion::CompileIncomplete>>
             at D:\rust-lang\rust\src\libstd\thread\local.rs:288
  26: std::thread::local::LocalKey<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> core::result::Result<(), core::fmt::Error>>>::with<core::cell:
:Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> core::result::Result<(), core::fmt::Error>>,closure,core::result::Result<core::result::Result<(rustc::sessio
n::config::OutputFilenames, alloc::boxed::Box<Any>, rustc::dep_graph::graph::DepGraph), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>
             at D:\rust-lang\rust\src\libstd\thread\local.rs:288
  27: rustc_driver::driver::compile_input
             at D:\rust-lang\rust\src\librustc_driver\driver.rs:274
  28: rustc_driver::run_compiler
             at D:\rust-lang\rust\src\librustc_driver\lib.rs:302
