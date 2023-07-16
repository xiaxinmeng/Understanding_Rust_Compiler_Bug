plain
   Compiling rustc_codegen_ssa v0.0.0 (/checkout/compiler/rustc_codegen_ssa)
   Compiling rustc_resolve v0.0.0 (/checkout/compiler/rustc_resolve)
   Compiling rustc_trait_selection v0.0.0 (/checkout/compiler/rustc_trait_selection)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
error: internal compiler error: compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs:346:17: cannot relate region: LUB(ReErased, ReEmpty(U0))
thread 'rustc' panicked at 'Box<dyn Any>', /checkout/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
stack backtrace:
   0:     0x7f4e9f4ece12 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h08649ce12940e8c1
   1:     0x7f4e9f554a78 - core::fmt::write::ha01458c252ca8e28
   2:     0x7f4e9f4dd0e1 - std::io::Write::write_fmt::h4fb7f0f47561e7a9
   3:     0x7f4e9f4f0129 - std::panicking::default_hook::{{closure}}::h61addd9ad436ef38
   4:     0x7f4e9f4efdca - std::panicking::default_hook::h46350f1a9fa39981
   5:     0x7f4e9ffef034 - rustc_driver[bb0ac4008ad3e9fc]::DEFAULT_HOOK::{closure#0}::{closure#0}
   6:     0x7f4e9f4f098f - std::panicking::rust_panic_with_hook::h70294a24cb020d21
   7:     0x7f4ea29a7e13 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[81b66f48ab2827ec]::ExplicitBug>::{closure#0}
   8:     0x7f4ea29a66e6 - std[836a811975e52724]::sys_common::backtrace::__rust_end_short_backtrace::<std[836a811975e52724]::panicking::begin_panic<rustc_errors[81b66f48ab2827ec]::ExplicitBug>::{closure#0}, !>
   9:     0x7f4e9ff72b76 - std[836a811975e52724]::panicking::begin_panic::<rustc_errors[81b66f48ab2827ec]::ExplicitBug>
  10:     0x7f4ea27b0476 - std[836a811975e52724]::panic::panic_any::<rustc_errors[81b66f48ab2827ec]::ExplicitBug>
  11:     0x7f4ea27ac5d6 - <rustc_errors[81b66f48ab2827ec]::HandlerInner>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  12:     0x7f4ea27ac2a0 - <rustc_errors[81b66f48ab2827ec]::Handler>::bug::<&alloc[f55ce12b9f25f528]::string::String>
  13:     0x7f4ea299e92e - rustc_middle[7c2d6da264b3b0e3]::ty::context::tls::with_opt::<rustc_middle[7c2d6da264b3b0e3]::util::bug::opt_span_bug_fmt<rustc_span[ec683a5befddaf22]::span_encoding::Span>::{closure#0}, ()>
  14:     0x7f4ea299f129 - rustc_middle[7c2d6da264b3b0e3]::util::bug::opt_span_bug_fmt::<rustc_span[ec683a5befddaf22]::span_encoding::Span>
  15:     0x7f4e9ff7a0b5 - rustc_middle[7c2d6da264b3b0e3]::util::bug::bug_fmt
  16:     0x7f4ea274000a - <rustc_infer[882077c6ac31a764]::infer::lexical_region_resolve::LexicalResolver>::lub_concrete_regions
  17:     0x7f4ea273be4b - <rustc_infer[882077c6ac31a764]::infer::lexical_region_resolve::LexicalResolver>::expand_node
  18:     0x7f4ea27386ae - <rustc_infer[882077c6ac31a764]::infer::lexical_region_resolve::LexicalResolver>::infer_variable_values
  19:     0x7f4ea273eca1 - rustc_infer[882077c6ac31a764]::infer::lexical_region_resolve::resolve
  20:     0x7f4ea267888c - <rustc_infer[882077c6ac31a764]::infer::InferCtxt>::resolve_regions
  21:     0x7f4ea2678b1d - <rustc_infer[882077c6ac31a764]::infer::InferCtxt>::resolve_regions_and_report_errors
  22:     0x7f4ea095e0e2 - <rustc_typeck[a339ae6f6ee1e29a]::check::fn_ctxt::FnCtxt>::regionck_item
  23:     0x7f4ea0b774ce - <rustc_infer[882077c6ac31a764]::infer::InferCtxtBuilder>::enter::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_typeck[a339ae6f6ee1e29a]::check::compare_method::raw_compare_const_impl::{closure#0}>
  24:     0x7f4ea0c84e7f - rustc_typeck[a339ae6f6ee1e29a]::check::compare_method::raw_compare_const_impl
  25:     0x7f4ea1b6aa32 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::compare_assoc_const_impl_item_with_trait_item, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  26:     0x7f4ea1710325 - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::compare_assoc_const_impl_item_with_trait_item
  27:     0x7f4ea0c9f408 - rustc_ty_utils[5d973650caea066e]::instance::inner_resolve_instance
  28:     0x7f4ea0c9bb33 - rustc_ty_utils[5d973650caea066e]::instance::resolve_instance
  29:     0x7f4ea1b2db8c - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::resolve_instance, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  30:     0x7f4ea170d6ed - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::resolve_instance
  31:     0x7f4ea2920798 - <rustc_middle[7c2d6da264b3b0e3]::ty::instance::Instance>::resolve_opt_const_arg
  32:     0x7f4ea06fd18b - <rustc_const_eval[af402a68111e18b7]::interpret::eval_context::InterpCx<rustc_mir_transform[58eb2bf1acf89467]::const_prop_lint::ConstPropMachine>>::mir_const_to_op
  33:     0x7f4ea06fbca7 - <rustc_const_eval[af402a68111e18b7]::interpret::eval_context::InterpCx<rustc_mir_transform[58eb2bf1acf89467]::const_prop_lint::ConstPropMachine>>::eval_operand
  34:     0x7f4ea06d3f48 - <rustc_const_eval[af402a68111e18b7]::interpret::eval_context::InterpCx<rustc_mir_transform[58eb2bf1acf89467]::const_prop_lint::ConstPropMachine>>::eval_rvalue_into_place
  35:     0x7f4ea0602fb1 - <rustc_mir_transform[58eb2bf1acf89467]::const_prop_lint::ConstPropagator as rustc_middle[7c2d6da264b3b0e3]::mir::visit::Visitor>::visit_statement
  36:     0x7f4ea0602a4f - <rustc_mir_transform[58eb2bf1acf89467]::const_prop_lint::ConstPropagator as rustc_middle[7c2d6da264b3b0e3]::mir::visit::Visitor>::visit_body
  37:     0x7f4ea05ff0ec - <rustc_mir_transform[58eb2bf1acf89467]::const_prop_lint::ConstProp as rustc_mir_transform[58eb2bf1acf89467]::pass_manager::MirLint>::run_lint
  38:     0x7f4ea053518f - rustc_mir_transform[58eb2bf1acf89467]::pass_manager::run_passes
  39:     0x7f4ea05d3572 - rustc_mir_transform[58eb2bf1acf89467]::run_post_borrowck_cleanup_passes
  40:     0x7f4ea05d3136 - rustc_mir_transform[58eb2bf1acf89467]::mir_drops_elaborated_and_const_checked
  41:     0x7f4ea1a4f17c - rustc_query_system[e920471a40e4ffdd]::query::plumbing::try_execute_query::<rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt, rustc_query_system[e920471a40e4ffdd]::query::caches::DefaultCache<rustc_middle[7c2d6da264b3b0e3]::ty::WithOptConstParam<rustc_span[ec683a5befddaf22]::def_id::LocalDefId>, &rustc_data_structures[445150d0d950bdef]::steal::Steal<rustc_middle[7c2d6da264b3b0e3]::mir::Body>>>
  42:     0x7f4ea1b690f3 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  43:     0x7f4ea16bec37 - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  44:     0x7f4ea05d3d63 - rustc_mir_transform[58eb2bf1acf89467]::optimized_mir
  45:     0x7f4ea1a8203d - rustc_query_system[e920471a40e4ffdd]::query::plumbing::try_execute_query::<rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt, rustc_query_system[e920471a40e4ffdd]::query::caches::DefaultCache<rustc_span[ec683a5befddaf22]::def_id::DefId, &rustc_middle[7c2d6da264b3b0e3]::mir::Body>>
  46:     0x7f4ea1b21b72 - rustc_query_system[e920471a40e4ffdd]::query::plumbing::get_query::<rustc_query_impl[f9863c4b755a1443]::queries::optimized_mir, rustc_query_impl[f9863c4b755a1443]::plumbing::QueryCtxt>
  47:     0x7f4ea16c06e9 - <rustc_query_impl[f9863c4b755a1443]::Queries as rustc_middle[7c2d6da264b3b0e3]::ty::query::QueryEngine>::optimized_mir
  48:     0x7f4ea1f69286 - <rustc_metadata[52c24d36431af71a]::rmeta::encoder::EncodeContext>::encode_crate_root
  49:     0x7f4ea1f80334 - rustc_metadata[52c24d36431af71a]::rmeta::encoder::encode_metadata_impl
  50:     0x7f4ea20429e1 - rustc_data_structures[445150d0d950bdef]::sync::join::<rustc_metadata[52c24d36431af71a]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[52c24d36431af71a]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[52c24d36431af71a]::rmeta::encoder::EncodedMetadata, ()>
  51:     0x7f4ea1f7f96e - rustc_metadata[52c24d36431af71a]::rmeta::encoder::encode_metadata
  52:     0x7f4ea011eb5d - <rustc_interface[d3b72adc6f23e1ad]::passes::QueryContext>::enter::<<rustc_interface[d3b72adc6f23e1ad]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[6d9550a4e960c99f]::result::Result<alloc[f55ce12b9f25f528]::boxed::Box<dyn core[6d9550a4e960c99f]::any::Any>, rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  53:     0x7f4ea0109f3e - <rustc_interface[d3b72adc6f23e1ad]::queries::Queries>::ongoing_codegen
  54:     0x7f4ea000da9f - <rustc_interface[d3b72adc6f23e1ad]::interface::Compiler>::enter::<rustc_driver[bb0ac4008ad3e9fc]::run_compiler::{closure#1}::{closure#2}, core[6d9550a4e960c99f]::result::Result<core[6d9550a4e960c99f]::option::Option<rustc_interface[d3b72adc6f23e1ad]::queries::Linker>, rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  55:     0x7f4e9fff0666 - rustc_span[ec683a5befddaf22]::with_source_map::<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_interface[d3b72adc6f23e1ad]::interface::create_compiler_and_run<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[bb0ac4008ad3e9fc]::run_compiler::{closure#1}>::{closure#1}>
  56:     0x7f4ea000ecbe - <scoped_tls[112f8d9a5d871235]::ScopedKey<rustc_span[ec683a5befddaf22]::SessionGlobals>>::set::<rustc_interface[d3b72adc6f23e1ad]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[bb0ac4008ad3e9fc]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  57:     0x7f4ea005dad9 - std[836a811975e52724]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[d3b72adc6f23e1ad]::util::run_in_thread_pool_with_globals<rustc_interface[d3b72adc6f23e1ad]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[bb0ac4008ad3e9fc]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>
  58:     0x7f4ea005e739 - <<std[836a811975e52724]::thread::Builder>::spawn_unchecked_<rustc_interface[d3b72adc6f23e1ad]::util::run_in_thread_pool_with_globals<rustc_interface[d3b72adc6f23e1ad]::interface::run_compiler<core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>, rustc_driver[bb0ac4008ad3e9fc]::run_compiler::{closure#1}>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>::{closure#0}, core[6d9550a4e960c99f]::result::Result<(), rustc_errors[81b66f48ab2827ec]::ErrorGuaranteed>>::{closure#1} as core[6d9550a4e960c99f]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  59:     0x7f4e9f4fd353 - std::sys::unix::thread::Thread::new::thread_start::h09105972e562a0e6
  60:     0x7f4e99a4d609 - start_thread
  61:     0x7f4e9f360133 - clone
  62:                0x0 - <unknown>
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (bac0328f2 2022-06-25) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [compare_assoc_const_impl_item_with_trait_item] checking assoc const `<impl at compiler/rustc_query_impl/src/plumbing.rs:323:9: 359:10>::TRY_LOAD_FROM_DISK` has the same type as trait item
#1 [resolve_instance] resolving instance `<queries::trigger_delay_span_bug as rustc_query_system::query::config::QueryDescription<plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK`
#2 [mir_drops_elaborated_and_const_checked] elaborating drops for `<impl at compiler/rustc_query_impl/src/plumbing.rs:323:9: 359:10>::make_vtable`
#3 [optimized_mir] optimizing MIR for `<impl at compiler/rustc_query_impl/src/plumbing.rs:323:9: 359:10>::make_vtable`
error: could not compile `rustc_query_impl`
warning: build failed, waiting for other jobs to finish...
Build completed unsuccessfully in 0:08:53
