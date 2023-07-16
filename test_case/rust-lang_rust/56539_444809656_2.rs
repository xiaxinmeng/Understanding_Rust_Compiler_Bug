rust
$ RUST_BACKTRACE=1 cargo build
   Compiling offst-common v0.1.0 (/home/real/temp/trait_alias_crash/common)
warning: unused import: `future`
 --> src/lib.rs:6:15
  |
6 | use futures::{future, Future};
  |               ^^^^^^
  |
  = note: #[warn(unused_imports)] on by default

error: internal compiler error: src/librustc_metadata/encoder.rs:1158: cannot encode info for item Item { name: BytesConnTransform, id: NodeId(113), hir_id: HirId { owner: DefIndex(0:17), local_id: 0 }, attrs: [], node: TraitAlias(Generics { params: [GenericParam { id: NodeId(114), name: Plain(Arg#0), attrs: [], bounds: [], span: src/lib.rs:24:30: 24:33, pure_wrt_drop: false, kind: Type { default: None, synthetic: None } }], where_clause: WhereClause { id: NodeId(115), predicates: [] }, span: src/lib.rs:24:29: 24:34 }, [Trait(PolyTraitRef { bound_generic_params: [], trait_ref: TraitRef { path: path(ConnTransform<OldSendItem = Vec<u8>, OldRecvItem = Vec<u8>, NewSendItem =
Vec<u8>, NewRecvItem = Vec<u8>, Arg = Arg>), ref_id: NodeId(116), hir_ref_id: HirId { owner: DefIndex(0:17), local_id: 27 } }, span: src/lib.rs:24:37: 26:58 }, None)]), vis: Spanned { node: Public, span: src/lib.rs:24:1: 24:4 }, span: src/lib.rs:24:1: 26:59 }

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
             at src/libstd/panicking.rs:480
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
  21: <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::ExtraBackendMethods>::write_metadata
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
  35: syntax::with_globals
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

note: rustc 1.32.0-nightly (6bfb46e4a 2018-11-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 -C incremental --crate-type lib

note: some of the compiler flags provided by cargo are hidden

error: Could not compile `offst-common`.

