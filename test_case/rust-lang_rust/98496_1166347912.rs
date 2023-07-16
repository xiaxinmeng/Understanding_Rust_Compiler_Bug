plain
   Compiling rustc_monomorphize v0.0.0 (/checkout/compiler/rustc_monomorphize)
   Compiling rustc_query_impl v0.0.0 (/checkout/compiler/rustc_query_impl)
[RUSTC-TIMING] rustc_symbol_mangling test:false 13.966
   Compiling rustc_ast_lowering v0.0.0 (/checkout/compiler/rustc_ast_lowering)
error: internal compiler error: compiler/rustc_infer/src/infer/lexical_region_resolve/mod.rs:346:17: cannot relate region: LUB(ReErased, ReEmpty(U0))

thread 'rustc' panicked at 'Box<dyn Any>', /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/compiler/rustc_errors/src/lib.rs:1391:9
stack backtrace:
   0:     0x7f57cd1791bd - std::backtrace_rs::backtrace::libunwind::trace::hcb7e19a47ced099a
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/std/src/../../backtrace/src/backtrace/libunwind.rs:93:5
   1:     0x7f57cd1791bd - std::backtrace_rs::backtrace::trace_unsynchronized::hceb194f364d5d30b
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f57cd1791bd - std::sys_common::backtrace::_print_fmt::h05e589b1b17e92aa
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/std/src/sys_common/backtrace.rs:66:5
   3:     0x7f57cd1791bd - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha8303e86b7d477d5
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/std/src/sys_common/backtrace.rs:45:22
   4:     0x7f57cd1d4fcc - core::fmt::write::h800feb67a1d0ee26
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/core/src/fmt/mod.rs:1197:17
   5:     0x7f57cd16a831 - std::io::Write::write_fmt::h737805e6646fc929
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/std/src/io/mod.rs:1672:15
   6:     0x7f57cd17be85 - std::sys_common::backtrace::_print::hdc47058c8f3b037b
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/std/src/sys_common/backtrace.rs:48:5
   7:     0x7f57cd17be85 - std::sys_common::backtrace::print::h95af586996dc3475
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/std/src/sys_common/backtrace.rs:35:9
   8:     0x7f57cd17be85 - std::panicking::default_hook::{{closure}}::h8c723a5091df9e3e
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/std/src/panicking.rs:295:22
   9:     0x7f57cd17bba6 - std::panicking::default_hook::h7f7bb039449f2a3f
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/std/src/panicking.rs:314:9
  10:     0x7f57cdb310f6 - rustc_driver[b5e73b2c6a1a184f]::DEFAULT_HOOK::{closure#0}::{closure#0}
  11:     0x7f57cd17c55a - std::panicking::rust_panic_with_hook::h3492834d94b144b9
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/std/src/panicking.rs:702:17
  12:     0x7f57d015b0a3 - std[436c541dc92e00ba]::panicking::begin_panic::<rustc_errors[d33f2cd9b88c7e0f]::ExplicitBug>::{closure#0}
  13:     0x7f57d0158ee6 - std[436c541dc92e00ba]::sys_common::backtrace::__rust_end_short_backtrace::<std[436c541dc92e00ba]::panicking::begin_panic<rustc_errors[d33f2cd9b88c7e0f]::ExplicitBug>::{closure#0}, !>
  14:     0x7f57cda730b6 - std[436c541dc92e00ba]::panicking::begin_panic::<rustc_errors[d33f2cd9b88c7e0f]::ExplicitBug>
  15:     0x7f57d00654a6 - std[436c541dc92e00ba]::panic::panic_any::<rustc_errors[d33f2cd9b88c7e0f]::ExplicitBug>
  16:     0x7f57d00615b0 - <rustc_errors[d33f2cd9b88c7e0f]::HandlerInner>::bug::<&alloc[f3638e56fdfa7604]::string::String>
  17:     0x7f57d0061200 - <rustc_errors[d33f2cd9b88c7e0f]::Handler>::bug::<&alloc[f3638e56fdfa7604]::string::String>
  18:     0x7f57d017072a - rustc_middle[973ba3d1ad21327e]::ty::context::tls::with_opt::<rustc_middle[973ba3d1ad21327e]::util::bug::opt_span_bug_fmt<rustc_span[500af9fdb9a1a167]::span_encoding::Span>::{closure#0}, ()>
  19:     0x7f57d0170799 - rustc_middle[973ba3d1ad21327e]::util::bug::opt_span_bug_fmt::<rustc_span[500af9fdb9a1a167]::span_encoding::Span>
  20:     0x7f57cda74235 - rustc_middle[973ba3d1ad21327e]::util::bug::bug_fmt
  21:     0x7f57cff3f4aa - <rustc_infer[b7ab57084ea3e1da]::infer::lexical_region_resolve::LexicalResolver>::lub_concrete_regions
  22:     0x7f57cff3a556 - <rustc_infer[b7ab57084ea3e1da]::infer::lexical_region_resolve::LexicalResolver>::expand_node
  23:     0x7f57cff3cd7d - rustc_infer[b7ab57084ea3e1da]::infer::lexical_region_resolve::resolve
  24:     0x7f57cfeb086d - <rustc_infer[b7ab57084ea3e1da]::infer::InferCtxt>::resolve_regions
  25:     0x7f57cfeb0b03 - <rustc_infer[b7ab57084ea3e1da]::infer::InferCtxt>::resolve_regions_and_report_errors
  26:     0x7f57ce3a364f - <rustc_typeck[20696322f9349c00]::check::fn_ctxt::FnCtxt>::regionck_item
  27:     0x7f57ce52baea - <rustc_infer[b7ab57084ea3e1da]::infer::InferCtxtBuilder>::enter::<core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>, rustc_typeck[20696322f9349c00]::check::compare_method::raw_compare_const_impl::{closure#0}>
  28:     0x7f57ce627123 - rustc_typeck[20696322f9349c00]::check::compare_method::raw_compare_const_impl
  29:     0x7f57cf2ce62d - rustc_query_system[35694d78d79aca90]::query::plumbing::try_execute_query::<rustc_query_impl[e022af8221098411]::plumbing::QueryCtxt, rustc_query_system[35694d78d79aca90]::query::caches::DefaultCache<(&rustc_middle[973ba3d1ad21327e]::ty::assoc::AssocItem, rustc_span[500af9fdb9a1a167]::span_encoding::Span, &rustc_middle[973ba3d1ad21327e]::ty::assoc::AssocItem, rustc_middle[973ba3d1ad21327e]::ty::sty::TraitRef), core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>>>
  30:     0x7f57cf36c93e - rustc_query_system[35694d78d79aca90]::query::plumbing::get_query::<rustc_query_impl[e022af8221098411]::queries::compare_assoc_const_impl_item_with_trait_item, rustc_query_impl[e022af8221098411]::plumbing::QueryCtxt>
  31:     0x7f57cf1d0d5e - <rustc_query_impl[e022af8221098411]::Queries as rustc_middle[973ba3d1ad21327e]::ty::query::QueryEngine>::compare_assoc_const_impl_item_with_trait_item
  32:     0x7f57ce671d76 - rustc_ty_utils[8af79899bd37be16]::instance::inner_resolve_instance
  33:     0x7f57ce66ee19 - rustc_ty_utils[8af79899bd37be16]::instance::resolve_instance
  34:     0x7f57cf345372 - rustc_query_system[35694d78d79aca90]::query::plumbing::get_query::<rustc_query_impl[e022af8221098411]::queries::resolve_instance, rustc_query_impl[e022af8221098411]::plumbing::QueryCtxt>
  35:     0x7f57cf1d0c08 - <rustc_query_impl[e022af8221098411]::Queries as rustc_middle[973ba3d1ad21327e]::ty::query::QueryEngine>::resolve_instance
  36:     0x7f57cffe13af - <rustc_middle[973ba3d1ad21327e]::ty::instance::Instance>::resolve_opt_const_arg
  37:     0x7f57ce135d1e - <rustc_const_eval[69844a21f882465f]::interpret::eval_context::InterpCx<rustc_mir_transform[a3349d3defae93ef]::const_prop_lint::ConstPropMachine>>::mir_const_to_op
  38:     0x7f57ce11d9ac - <rustc_const_eval[69844a21f882465f]::interpret::eval_context::InterpCx<rustc_mir_transform[a3349d3defae93ef]::const_prop_lint::ConstPropMachine>>::eval_rvalue_into_place
  39:     0x7f57ce04b6ae - <rustc_mir_transform[a3349d3defae93ef]::const_prop_lint::ConstPropagator as rustc_middle[973ba3d1ad21327e]::mir::visit::Visitor>::visit_statement
  40:     0x7f57ce048f9d - <rustc_mir_transform[a3349d3defae93ef]::const_prop_lint::ConstProp as rustc_mir_transform[a3349d3defae93ef]::pass_manager::MirLint>::run_lint
  41:     0x7f57cdfa0091 - rustc_mir_transform[a3349d3defae93ef]::pass_manager::run_passes
  42:     0x7f57cdf6c3b7 - rustc_mir_transform[a3349d3defae93ef]::run_post_borrowck_cleanup_passes
  43:     0x7f57cdf6be22 - rustc_mir_transform[a3349d3defae93ef]::mir_drops_elaborated_and_const_checked
  44:     0x7f57cf25b895 - rustc_query_system[35694d78d79aca90]::query::plumbing::try_execute_query::<rustc_query_impl[e022af8221098411]::plumbing::QueryCtxt, rustc_query_system[35694d78d79aca90]::query::caches::DefaultCache<rustc_middle[973ba3d1ad21327e]::ty::WithOptConstParam<rustc_span[500af9fdb9a1a167]::def_id::LocalDefId>, &rustc_data_structures[4d29c0893d508c1]::steal::Steal<rustc_middle[973ba3d1ad21327e]::mir::Body>>>
  45:     0x7f57cf36b305 - rustc_query_system[35694d78d79aca90]::query::plumbing::get_query::<rustc_query_impl[e022af8221098411]::queries::mir_drops_elaborated_and_const_checked, rustc_query_impl[e022af8221098411]::plumbing::QueryCtxt>
  46:     0x7f57cf1cefb6 - <rustc_query_impl[e022af8221098411]::Queries as rustc_middle[973ba3d1ad21327e]::ty::query::QueryEngine>::mir_drops_elaborated_and_const_checked
  47:     0x7f57cdf6ca12 - rustc_mir_transform[a3349d3defae93ef]::optimized_mir
  48:     0x7f57cf2a51e0 - rustc_query_system[35694d78d79aca90]::query::plumbing::try_execute_query::<rustc_query_impl[e022af8221098411]::plumbing::QueryCtxt, rustc_query_system[35694d78d79aca90]::query::caches::DefaultCache<rustc_span[500af9fdb9a1a167]::def_id::DefId, &rustc_middle[973ba3d1ad21327e]::mir::Body>>
  49:     0x7f57cf33b953 - rustc_query_system[35694d78d79aca90]::query::plumbing::get_query::<rustc_query_impl[e022af8221098411]::queries::optimized_mir, rustc_query_impl[e022af8221098411]::plumbing::QueryCtxt>
  50:     0x7f57cf750540 - <rustc_metadata[9ae727fa2acea5c1]::rmeta::encoder::EncodeContext>::encode_crate_root
  51:     0x7f57cf76be52 - rustc_metadata[9ae727fa2acea5c1]::rmeta::encoder::encode_metadata_impl
  52:     0x7f57cf847191 - rustc_data_structures[4d29c0893d508c1]::sync::join::<rustc_metadata[9ae727fa2acea5c1]::rmeta::encoder::encode_metadata::{closure#0}, rustc_metadata[9ae727fa2acea5c1]::rmeta::encoder::encode_metadata::{closure#1}, rustc_metadata[9ae727fa2acea5c1]::rmeta::encoder::EncodedMetadata, ()>
  53:     0x7f57cf76b491 - rustc_metadata[9ae727fa2acea5c1]::rmeta::encoder::encode_metadata
  54:     0x7f57cdbed771 - <rustc_interface[5f796f16de8402be]::passes::QueryContext>::enter::<<rustc_interface[5f796f16de8402be]::queries::Queries>::ongoing_codegen::{closure#0}::{closure#0}, core[331a752f2dceee8e]::result::Result<alloc[f3638e56fdfa7604]::boxed::Box<dyn core[331a752f2dceee8e]::any::Any>, rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>>
  55:     0x7f57cdc86108 - <rustc_interface[5f796f16de8402be]::queries::Queries>::ongoing_codegen
  56:     0x7f57cdac07a5 - <rustc_interface[5f796f16de8402be]::interface::Compiler>::enter::<rustc_driver[b5e73b2c6a1a184f]::run_compiler::{closure#1}::{closure#2}, core[331a752f2dceee8e]::result::Result<core[331a752f2dceee8e]::option::Option<rustc_interface[5f796f16de8402be]::queries::Linker>, rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>>
  57:     0x7f57cdb33ce3 - rustc_span[500af9fdb9a1a167]::with_source_map::<core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>, rustc_interface[5f796f16de8402be]::interface::create_compiler_and_run<core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>, rustc_driver[b5e73b2c6a1a184f]::run_compiler::{closure#1}>::{closure#1}>
  58:     0x7f57cdac151a - <scoped_tls[b383e8bf08376368]::ScopedKey<rustc_span[500af9fdb9a1a167]::SessionGlobals>>::set::<rustc_interface[5f796f16de8402be]::interface::run_compiler<core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>, rustc_driver[b5e73b2c6a1a184f]::run_compiler::{closure#1}>::{closure#0}, core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>>
  59:     0x7f57cdb2085f - std[436c541dc92e00ba]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5f796f16de8402be]::util::run_in_thread_pool_with_globals<rustc_interface[5f796f16de8402be]::interface::run_compiler<core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>, rustc_driver[b5e73b2c6a1a184f]::run_compiler::{closure#1}>::{closure#0}, core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>>::{closure#0}, core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>>
  60:     0x7f57cdae57a9 - <<std[436c541dc92e00ba]::thread::Builder>::spawn_unchecked_<rustc_interface[5f796f16de8402be]::util::run_in_thread_pool_with_globals<rustc_interface[5f796f16de8402be]::interface::run_compiler<core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>, rustc_driver[b5e73b2c6a1a184f]::run_compiler::{closure#1}>::{closure#0}, core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>>::{closure#0}, core[331a752f2dceee8e]::result::Result<(), rustc_errors[d33f2cd9b88c7e0f]::ErrorGuaranteed>>::{closure#1} as core[331a752f2dceee8e]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  61:     0x7f57cd186493 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h79fd45ca6c7bea42
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/alloc/src/boxed.rs:1951:9
  62:     0x7f57cd186493 - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hbf6f4caed7de8063
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/alloc/src/boxed.rs:1951:9
  63:     0x7f57cd186493 - std::sys::unix::thread::Thread::new::thread_start::h9e22626341d0d487
                               at /rustc/f99589b0312bfdb02699eab496c80e3446034f1a/library/std/src/sys/unix/thread.rs:108:17
  64:     0x7f57ccab98ca - start_thread
  65:     0x7f57cc81fabd - clone
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.64.0-nightly (f99589b03 2022-06-25) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -Z unstable-options -C linker=clang -C symbol-mangling-version=v0 -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -Z unstable-options -C split-debuginfo=off -Z tls-model=initial-exec -Z unstable-options -C prefer-dynamic -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
#0 [compare_assoc_const_impl_item_with_trait_item] checking assoc const `<impl at compiler/rustc_query_impl/src/plumbing.rs:323:9: 359:10>::TRY_LOAD_FROM_DISK` has the same type as trait item
#1 [resolve_instance] resolving instance `<queries::trigger_delay_span_bug as rustc_query_system::query::config::QueryDescription<plumbing::QueryCtxt>>::TRY_LOAD_FROM_DISK`
#2 [mir_drops_elaborated_and_const_checked] elaborating drops for `<impl at compiler/rustc_query_impl/src/plumbing.rs:323:9: 359:10>::make_vtable`
#3 [optimized_mir] optimizing MIR for `<impl at compiler/rustc_query_impl/src/plumbing.rs:323:9: 359:10>::make_vtable`
[RUSTC-TIMING] rustc_query_impl test:false 7.352
error: could not compile `rustc_query_impl`
warning: build failed, waiting for other jobs to finish...
[RUSTC-TIMING] rustc_incremental test:false 24.007
