
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.25.0-dev running on x86_64-pc-windows-msvc

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'missing specialization: `<encoder::EncodeContext as SpecializedEncoder<rustc::hir::def_id::LocalDefId>>::specialized_encode` not overridden', libseriali
ze\serialize.rs:788:9
stack backtrace:
   0:     0x7ffedc5eb4a7 - std::sys_common::backtrace::print
                               at D:\rust-lang\rust\src\libstd\sys_common\backtrace.rs:61
   1:     0x7ffedc5e059f - std::panicking::default_hook::{{closure}}
                               at D:\rust-lang\rust\src\libstd\panicking.rs:380
   2:     0x7ffedc5e01e4 - std::panicking::default_hook
                               at D:\rust-lang\rust\src\libstd\panicking.rs:390
   3:     0x7ffedc5e0b65 - std::panicking::rust_panic_with_hook
                               at D:\rust-lang\rust\src\libstd\panicking.rs:576
   4:     0x7ffedc5e0970 - std::panicking::begin_panic<alloc::string::String>
                               at D:\rust-lang\rust\src\libstd\panicking.rs:537
   5:     0x7ffedc5e08c1 - std::panicking::begin_panic_fmt
                               at D:\rust-lang\rust\src\libstd\panicking.rs:521
   6:     0x7ffee23845e9 - serialize::serialize::{{impl}}::not_found<std::io::error::Error,rustc_metadata::encoder::EncodeContext,rustc::hir::def_id::LocalDefId>
                               at \<panic macros>:7
   7:     0x7ffee222ff84 - serialize::serialize::Encoder::emit_map<rustc_metadata::encoder::EncodeContext,closure>
                               at D:\rust-lang\rust\src\libserialize\serialize.rs:131
   8:     0x7ffee2226d19 - serialize::serialize::Encoder::emit_struct<rustc_metadata::encoder::EncodeContext,closure>
                               at D:\rust-lang\rust\src\libserialize\serialize.rs:83
   9:     0x7ffee22f80f5 - rustc::ty::context::{{impl}}::encode<rustc_metadata::encoder::EncodeContext>
                               at D:\rust-lang\rust\src\librustc\ty\context.rs:334
  10:     0x7ffee224f4ae - rustc_metadata::encoder::EncodeContext::lazy<rustc::ty::context::TypeckTables>
                               at D:\rust-lang\rust\src\librustc_metadata\encoder.rs:248
  11:     0x7ffee233291a - rustc_metadata::isolated_encoder::IsolatedEncoder::encode_body
                               at D:\rust-lang\rust\src\librustc_metadata\astencode.rs:58
  12:     0x7ffee2335ec9 - rustc_metadata::isolated_encoder::IsolatedEncoder::encode_info_for_item
                               at D:\rust-lang\rust\src\librustc_metadata\encoder.rs:1113
  13:     0x7ffee225358f - rustc_metadata::encoder::{{impl}}::visit_item
                               at D:\rust-lang\rust\src\librustc_metadata\encoder.rs:1474
  14:     0x7ffee22de93b - rustc::hir::Crate::visit_all_item_likes<rustc::hir::itemlikevisit::DeepVisitor<rustc_metadata::encoder::EncodeVisitor>>
                               at D:\rust-lang\rust\src\librustc\hir\mod.rs:647
  15:     0x7ffee22549ac - rustc_metadata::encoder::encode_metadata
                               at D:\rust-lang\rust\src\librustc_metadata\encoder.rs:1677
  16:     0x7ffee23575ab - rustc_metadata::cstore_impl::{{impl}}::encode_metadata
                               at D:\rust-lang\rust\src\librustc_metadata\cstore_impl.rs:526
  17:     0x7ffece47d34f - rustc::ty::context::TyCtxt::encode_metadata
                               at D:\rust-lang\rust\src\librustc\ty\context.rs:1371
  18:     0x7ffee17bd642 - rustc_trans::base::write_metadata
                               at D:\rust-lang\rust\src\librustc_trans\base.rs:646
  19:     0x7ffee1824d89 - rustc::util::common::time<(mut rustc_llvm::ffi::Context_opaque*, mut rustc_llvm::ffi::Module_opaque*, rustc::middle::cstore::EncodedMetadata),closure>
                               at D:\rust-lang\rust\src\librustc\util\common.rs:120
  20:     0x7ffee17bdf70 - rustc_trans::base::trans_crate
                               at D:\rust-lang\rust\src\librustc_trans\base.rs:726
  21:     0x7ffee17fb112 - rustc_trans::{{impl}}::trans_crate
                               at D:\rust-lang\rust\src\librustc_trans\lib.rs:210
  22:     0x7ffee2a8f02c - rustc::util::common::time<alloc::boxed::Box<Any>,closure>
                               at D:\rust-lang\rust\src\librustc\util\common.rs:120
  23:     0x7ffee2b3c5f6 - rustc_driver::driver::phase_4_translate_to_llvm
                               at D:\rust-lang\rust\src\librustc_driver\driver.rs:1088
  24:     0x7ffee2afbc66 - rustc_driver::driver::compile_input::{{closure}}
                               at D:\rust-lang\rust\src\librustc_driver\driver.rs:238
  25:     0x7ffee2af4fb9 - std::thread::local::LocalKey<core::cell::Cell<core::option::Option<(const rustc::ty::context::tls::ThreadLocalGlobalCtxt*, const rustc::ty::context::tls:
:ThreadLocalInterners*)>>>::with<core::cell::Cell<core::option::Option<(const rustc::ty::context::tls::ThreadLocalGlobalCtxt*, const rustc::ty::context::tls::ThreadLocalInterners*)
>>,closure,core::result::Result<core::result::Result<(rustc::session::config::OutputFilenames, alloc::boxed::Box<Any>, rustc::dep_graph::graph::DepGraph), rustc::session::CompileIn
complete>, rustc::session::CompileIncomplete>>
                               at D:\rust-lang\rust\src\libstd\thread\local.rs:288
  26:     0x7ffee2af8f70 - std::thread::local::LocalKey<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> core::result::Result<(), core::fmt::Error
>>>::with<core::cell::Cell<fn(syntax_pos::span_encoding::Span, mut core::fmt::Formatter*) -> core::result::Result<(), core::fmt::Error>>,closure,core::result::Result<core::result::
Result<(rustc::session::config::OutputFilenames, alloc::boxed::Box<Any>, rustc::dep_graph::graph::DepGraph), rustc::session::CompileIncomplete>, rustc::session::CompileIncomplete>>

                               at D:\rust-lang\rust\src\libstd\thread\local.rs:288
  27:     0x7ffee2b24173 - rustc_driver::driver::compile_input
                               at D:\rust-lang\rust\src\librustc_driver\driver.rs:274
  28:     0x7ffee2b3ebaf - rustc_driver::run_compiler
                               at D:\rust-lang\rust\src\librustc_driver\lib.rs:302
  29:     0x7ffee2a5792d - std::sys_common::backtrace::__rust_begin_short_backtrace<closure,()>
                               at D:\rust-lang\rust\src\libstd\sys_common\backtrace.rs:130
  30:     0x7ffedc60cbc1 - panic_unwind::__rust_maybe_catch_panic
                               at D:\rust-lang\rust\src\libpanic_unwind\lib.rs:102
  31:     0x7ffee2a63811 - alloc::boxed::{{impl}}::call_box<(),closure>
                               at D:\rust-lang\rust\src\liballoc\boxed.rs:788
  32:     0x7ffedc5ee08b - std::sys_common::thread::start_thread
                               at D:\rust-lang\rust\src\libstd\sys_common\thread.rs:24
  33:     0x7ffedc5ddfe6 - std::sys::windows::thread::{{impl}}::new::thread_start
                               at D:\rust-lang\rust\src\libstd\sys\windows\thread.rs:55
  34:     0x7fff1b1a1fe3 - BaseThreadInitThunk
