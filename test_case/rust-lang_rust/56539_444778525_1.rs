
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: src/librustc_metadata/encoder.rs:1158: cannot encode info for item Item { name: FooAlias, id: NodeId(13), hir_id: HirId { owner: DefIndex(0:4), local_id: 0 }, attrs: [], node: TraitAlias(Generics { params: [], where_clause: WhereClause { id: NodeId(14), predicates: [] }, span: src/lib.rs:1:1: 1:1 }, [Trait(PolyTraitRef { bound_generic_params: [], trait_ref: TraitRef { path: path(Foo), ref_id: NodeId(15), hir_ref_id: HirId { owner: DefIndex(0:4), local_id: 3 } }, span: src/lib.rs:4:22: 4:25 }, None)]), vis: Spanned { node: Public, span: src/lib.rs:4:1: 4:4 }, span: src/lib.rs:4:1: 4:26 }

thread 'main' panicked at 'Box<Any>', src/librustc_errors/lib.rs:600:9
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:71
   2: std::panicking::default_hook::{{closure}}
             at src/libstd/sys_common/backtrace.rs:59
             at src/libstd/panicking.rs:211
   3: std::panicking::default_hook
             at src/libstd/panicking.rs:227
   4: rustc::util::common::panic_hook
   5: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:495
   6: std::panicking::begin_panic
   7: rustc_errors::Handler::bug
   8: rustc::util::bug::opt_span_bug_fmt::{{closure}}
   9: rustc::ty::context::tls::with_opt::{{closure}}
  10: rustc::ty::context::tls::with_context_opt
  11: rustc::ty::context::tls::with_opt
  12: rustc::util::bug::opt_span_bug_fmt
  13: rustc::util::bug::bug_fmt
  14: rustc_metadata::encoder::<impl rustc_metadata::isolated_encoder::IsolatedEncoder<'a, 'b, 'tcx>>::encode_info_for_item
  15: rustc::ty::context::tls::with_context
  16: <rustc_metadata::encoder::EncodeVisitor<'a, 'b, 'tcx> as rustc::hir::intravisit::Visitor<'tcx>>::visit_item
  17: rustc::hir::Crate::visit_all_item_likes
  18: rustc_metadata::encoder::encode_metadata
  19: rustc_metadata::cstore_impl::<impl rustc::middle::cstore::CrateStore for rustc_metadata::cstore::CStore>::encode_metadata
  20: rustc::ty::context::TyCtxt::encode_metadata
  21: rustc_codegen_llvm::base::write_metadata
  22: rustc::util::common::time
  23: rustc_codegen_ssa::base::codegen_crate
  24: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_utils::codegen_backend::CodegenBackend>::codegen_crate
  25: rustc::util::common::time
  26: rustc_driver::driver::phase_4_codegen
  27: rustc_driver::driver::compile_input::{{closure}}
  28: rustc::ty::context::tls::enter_context
  29: <std::thread::local::LocalKey<T>>::with
  30: rustc::ty::context::TyCtxt::create_and_enter
  31: rustc_driver::driver::compile_input
  32: rustc_driver::run_compiler_with_pool
  33: <scoped_tls::ScopedKey<T>>::set
  34: rustc_driver::run_compiler
  35: rustc_driver::monitor::{{closure}}
  36: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:102
  37: rustc_driver::run
  38: rustc_driver::main
  39: std::rt::lang_start::{{closure}}
  40: std::panicking::try::do_call
             at src/libstd/rt.rs:59
             at src/libstd/panicking.rs:310
  41: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:102
  42: std::rt::lang_start_internal
             at src/libstd/panicking.rs:289
             at src/libstd/panic.rs:398
             at src/libstd/rt.rs:58
  43: main
  44: __libc_start_main
  45: <unknown>
query stack during panic:
end of query stack
error: aborting due to previous error


note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-nightly (14997d56a 2018-12-05) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `playground`.

To learn more, run the command again with --verbose.

