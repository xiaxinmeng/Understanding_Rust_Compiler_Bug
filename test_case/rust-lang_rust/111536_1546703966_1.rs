
thread 'rustc' panicked at 'DefId(94:79 ~ factor_service[3174]::byte_storage::Foo::foo::{opaque#1}) does not have a "def_span"', compiler/rustc_metadata/src/rmeta/decoder/cstore_impl.rs:205:1
stack backtrace:
   0:        0x100c50738 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha9e6c436959f1b8e
   1:        0x100ca075c - core::fmt::write::h03620c265cda50c9
   2:        0x100c46658 - std::io::Write::write_fmt::h4582733544e760df
   3:        0x100c50598 - std::sys_common::backtrace::print::h52cbdd72190599ef
   4:        0x100c52f38 - std::panicking::default_hook::{{closure}}::hbd7990be14834316
   5:        0x100c52d3c - std::panicking::default_hook::hefa394ac64f6527f
   6:        0x109175b58 - rustc_driver_impl[9431cfa2328e8da]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:        0x100c5352c - std::panicking::rust_panic_with_hook::h7504e9066f7b4f07
   8:        0x100c53340 - std::panicking::begin_panic_handler::{{closure}}::h8b23331f56ad415f
   9:        0x100c50b34 - std::sys_common::backtrace::__rust_end_short_backtrace::h2cbfeb47d7910a46
  10:        0x100c530d4 - _rust_begin_unwind
  11:        0x100ccc054 - core::panicking::panic_fmt::h12faa9f03ec5a41d
  12:        0x10c804290 - rustc_metadata[f2a84a8141f6c436]::rmeta::decoder::cstore_impl::provide_extern::def_span::{closure#2}
  13:        0x10c804174 - rustc_metadata[f2a84a8141f6c436]::rmeta::decoder::cstore_impl::provide_extern::def_span
  14:        0x10c428d98 - <std[614eb408c5fa570d]::thread::local::LocalKey<core[dc40836e4f3e0da7]::cell::Cell<*const ()>>>::with::<rustc_middle[662f54c0a25f7c56]::ty::context::tls::enter_context<rustc_query_system[2ed44d2bc999b7a]::query::plumbing::execute_job_incr<rustc_query_impl[eba986b151bf483e]::queries::def_span, rustc_query_impl[eba986b151bf483e]::plumbing::QueryCtxt>::{closure#2}, (rustc_middle[662f54c0a25f7c56]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[2ed44d2bc999b7a]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle[662f54c0a25f7c56]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[2ed44d2bc999b7a]::dep_graph::graph::DepNodeIndex)>
  15:        0x10c3070e0 - rustc_query_system[2ed44d2bc999b7a]::query::plumbing::try_execute_query::<rustc_query_impl[eba986b151bf483e]::queries::def_span, rustc_query_impl[eba986b151bf483e]::plumbing::QueryCtxt>
  16:        0x10c507b34 - <rustc_query_impl[eba986b151bf483e]::Queries as rustc_middle[662f54c0a25f7c56]::ty::query::QueryEngine>::def_span
  17:        0x10cee00c4 - <rustc_infer[7cfcef9d7360a476]::infer::InferCtxt>::infer_projection
  18:        0x10ccde9e0 - rustc_trait_selection[29c2db0f17aee347]::traits::project::normalize_projection_type
  19:        0x10ccdd6c4 - <rustc_trait_selection[29c2db0f17aee347]::traits::project::AssocTypeNormalizer as rustc_type_ir[50bb8a969b2c53c]::fold::TypeFolder<rustc_middle[662f54c0a25f7c56]::ty::context::TyCtxt>>::fold_ty
  20:        0x10b6362fc - <&rustc_middle[662f54c0a25f7c56]::ty::list::List<rustc_middle[662f54c0a25f7c56]::ty::Ty> as rustc_type_ir[50bb8a969b2c53c]::fold::TypeFoldable<rustc_middle[662f54c0a25f7c56]::ty::context::TyCtxt>>::try_fold_with::<rustc_trait_selection[29c2db0f17aee347]::traits::project::AssocTypeNormalizer>
  21:        0x10b6bb5c4 - <rustc_trait_selection[29c2db0f17aee347]::traits::project::AssocTypeNormalizer>::fold::<(rustc_middle[662f54c0a25f7c56]::ty::sty::FnSig, rustc_middle[662f54c0a25f7c56]::ty::InstantiatedPredicates)>
  22:        0x10b6bfcfc - rustc_trait_selection[29c2db0f17aee347]::traits::project::normalize_with_depth::<(rustc_middle[662f54c0a25f7c56]::ty::sty::FnSig, rustc_middle[662f54c0a25f7c56]::ty::InstantiatedPredicates)>
  23:        0x10b632918 - <rustc_infer[7cfcef9d7360a476]::infer::at::At as rustc_trait_selection[29c2db0f17aee347]::traits::project::NormalizeExt>::normalize::<(rustc_middle[662f54c0a25f7c56]::ty::sty::FnSig, rustc_middle[662f54c0a25f7c56]::ty::InstantiatedPredicates)>
  24:        0x10b56a1cc - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::normalize::<(rustc_middle[662f54c0a25f7c56]::ty::sty::FnSig, rustc_middle[662f54c0a25f7c56]::ty::InstantiatedPredicates)>
  25:        0x10b6a1b40 - <rustc_hir_typeck[c13bf6c1f93f170b]::method::confirm::ConfirmContext>::confirm
  26:        0x10b5b63ec - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::lookup_method
  27:        0x10b5a72a0 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_kind
  28:        0x10b561bc8 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  29:        0x10b570f84 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_argument_types
  30:        0x10b554668 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::confirm_builtin_call
  31:        0x10b552110 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_call
  32:        0x10b5a7208 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_kind
  33:        0x10b561bc8 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  34:        0x10b5a2aa4 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_match::{closure#0}
  35:        0x10b5a6e58 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_kind
  36:        0x10b561bc8 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  37:        0x10b57711c - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_stmt
  38:        0x10b5776b0 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_block_with_expected
  39:        0x10b5a6e28 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_kind
  40:        0x10b561bc8 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  41:        0x10b562dac - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_return_expr
  42:        0x10b63f974 - rustc_hir_typeck[c13bf6c1f93f170b]::check::check_fn
  43:        0x10b5a4ac4 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_closure
  44:        0x10b5a7228 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_kind
  45:        0x10b561bc8 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  46:        0x10b576b40 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_decl
  47:        0x10b576f60 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_stmt
  48:        0x10b5776b0 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_block_with_expected
  49:        0x10b5a6e28 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_kind
  50:        0x10b561bc8 - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_expr_with_expectation_and_args
  51:        0x10b562dac - <rustc_hir_typeck[c13bf6c1f93f170b]::fn_ctxt::FnCtxt>::check_return_expr
  52:        0x10b63f974 - rustc_hir_typeck[c13bf6c1f93f170b]::check::check_fn
  53:        0x10b5e9610 - rustc_hir_typeck[c13bf6c1f93f170b]::typeck
  54:        0x10c426644 - <std[614eb408c5fa570d]::thread::local::LocalKey<core[dc40836e4f3e0da7]::cell::Cell<*const ()>>>::with::<rustc_middle[662f54c0a25f7c56]::ty::context::tls::enter_context<rustc_query_system[2ed44d2bc999b7a]::query::plumbing::execute_job_incr<rustc_query_impl[eba986b151bf483e]::queries::typeck, rustc_query_impl[eba986b151bf483e]::plumbing::QueryCtxt>::{closure#2}, (rustc_middle[662f54c0a25f7c56]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[2ed44d2bc999b7a]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle[662f54c0a25f7c56]::query::erase::Erased<[u8; 8usize]>, rustc_query_system[2ed44d2bc999b7a]::dep_graph::graph::DepNodeIndex)>
  55:        0x10c30558c - rustc_query_system[2ed44d2bc999b7a]::query::plumbing::try_execute_query::<rustc_query_impl[eba986b151bf483e]::queries::typeck, rustc_query_impl[eba986b151bf483e]::plumbing::QueryCtxt>
  56:        0x10c27cfc4 - rustc_query_system[2ed44d2bc999b7a]::query::plumbing::force_query::<rustc_query_impl[eba986b151bf483e]::queries::typeck, rustc_query_impl[eba986b151bf483e]::plumbing::QueryCtxt>
  57:        0x10c4b6460 - <rustc_query_impl[eba986b151bf483e]::plumbing::query_callback<rustc_query_impl[eba986b151bf483e]::queries::typeck>::{closure#0} as core[dc40836e4f3e0da7]::ops::function::FnOnce<(rustc_middle[662f54c0a25f7c56]::ty::context::TyCtxt, rustc_query_system[2ed44d2bc999b7a]::dep_graph::dep_node::DepNode<rustc_middle[662f54c0a25f7c56]::dep_graph::dep_node::DepKind>)>>::call_once
  58:        0x10c493e58 - <rustc_query_system[2ed44d2bc999b7a]::dep_graph::graph::DepGraphData<rustc_middle[662f54c0a25f7c56]::dep_graph::dep_node::DepKind>>::try_mark_previous_green::<rustc_query_impl[eba986b151bf483e]::plumbing::QueryCtxt>
  59:        0x10c493b10 - <rustc_query_system[2ed44d2bc999b7a]::dep_graph::graph::DepGraphData<rustc_middle[662f54c0a25f7c56]::dep_graph::dep_node::DepKind>>::try_mark_green::<rustc_query_impl[eba986b151bf483e]::plumbing::QueryCtxt>
  60:        0x10c3dd130 - <std[614eb408c5fa570d]::thread::local::LocalKey<core[dc40836e4f3e0da7]::cell::Cell<*const ()>>>::with::<rustc_middle[662f54c0a25f7c56]::ty::context::tls::enter_context<rustc_query_system[2ed44d2bc999b7a]::query::plumbing::execute_job_incr<rustc_query_impl[eba986b151bf483e]::queries::typeck_item_bodies, rustc_query_impl[eba986b151bf483e]::plumbing::QueryCtxt>::{closure#1}, core[dc40836e4f3e0da7]::option::Option<(rustc_middle[662f54c0a25f7c56]::query::erase::Erased<[u8; 0usize]>, rustc_query_system[2ed44d2bc999b7a]::dep_graph::graph::DepNodeIndex)>>::{closure#0}, core[dc40836e4f3e0da7]::option::Option<(rustc_middle[662f54c0a25f7c56]::query::erase::Erased<[u8; 0usize]>, rustc_query_system[2ed44d2bc999b7a]::dep_graph::graph::DepNodeIndex)>>
  61:        0x10c2c55dc - rustc_query_system[2ed44d2bc999b7a]::query::plumbing::try_execute_query::<rustc_query_impl[eba986b151bf483e]::queries::typeck_item_bodies, rustc_query_impl[eba986b151bf483e]::plumbing::QueryCtxt>
  62:        0x10c504738 - <rustc_query_impl[eba986b151bf483e]::Queries as rustc_middle[662f54c0a25f7c56]::ty::query::QueryEngine>::typeck_item_bodies
  63:        0x10b728860 - rustc_hir_analysis[a2dc3de1ec131d7c]::check_crate
  64:        0x109226a34 - rustc_interface[5a3a908aa1aef180]::passes::analysis
  65:        0x10c428974 - <std[614eb408c5fa570d]::thread::local::LocalKey<core[dc40836e4f3e0da7]::cell::Cell<*const ()>>>::with::<rustc_middle[662f54c0a25f7c56]::ty::context::tls::enter_context<rustc_query_system[2ed44d2bc999b7a]::query::plumbing::execute_job_incr<rustc_query_impl[eba986b151bf483e]::queries::analysis, rustc_query_impl[eba986b151bf483e]::plumbing::QueryCtxt>::{closure#2}, (rustc_middle[662f54c0a25f7c56]::query::erase::Erased<[u8; 1usize]>, rustc_query_system[2ed44d2bc999b7a]::dep_graph::graph::DepNodeIndex)>::{closure#0}, (rustc_middle[662f54c0a25f7c56]::query::erase::Erased<[u8; 1usize]>, rustc_query_system[2ed44d2bc999b7a]::dep_graph::graph::DepNodeIndex)>
  66:        0x10c30699c - rustc_query_system[2ed44d2bc999b7a]::query::plumbing::try_execute_query::<rustc_query_impl[eba986b151bf483e]::queries::analysis, rustc_query_impl[eba986b151bf483e]::plumbing::QueryCtxt>
  67:        0x10c4fb748 - <rustc_query_impl[eba986b151bf483e]::Queries as rustc_middle[662f54c0a25f7c56]::ty::query::QueryEngine>::analysis
  68:        0x10914f2d0 - <rustc_middle[662f54c0a25f7c56]::ty::context::GlobalCtxt>::enter::<rustc_driver_impl[9431cfa2328e8da]::run_compiler::{closure#1}::{closure#2}::{closure#4}, core[dc40836e4f3e0da7]::result::Result<(), rustc_span[7733720cb78d73dd]::ErrorGuaranteed>>
  69:        0x10914ec38 - <rustc_interface[5a3a908aa1aef180]::interface::Compiler>::enter::<rustc_driver_impl[9431cfa2328e8da]::run_compiler::{closure#1}::{closure#2}, core[dc40836e4f3e0da7]::result::Result<core[dc40836e4f3e0da7]::option::Option<rustc_interface[5a3a908aa1aef180]::queries::Linker>, rustc_span[7733720cb78d73dd]::ErrorGuaranteed>>
  70:        0x109112ca4 - rustc_span[7733720cb78d73dd]::set_source_map::<core[dc40836e4f3e0da7]::result::Result<(), rustc_span[7733720cb78d73dd]::ErrorGuaranteed>, rustc_interface[5a3a908aa1aef180]::interface::run_compiler<core[dc40836e4f3e0da7]::result::Result<(), rustc_span[7733720cb78d73dd]::ErrorGuaranteed>, rustc_driver_impl[9431cfa2328e8da]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  71:        0x109118250 - <scoped_tls[4e67b4d1df739e32]::ScopedKey<rustc_span[7733720cb78d73dd]::SessionGlobals>>::set::<rustc_interface[5a3a908aa1aef180]::interface::run_compiler<core[dc40836e4f3e0da7]::result::Result<(), rustc_span[7733720cb78d73dd]::ErrorGuaranteed>, rustc_driver_impl[9431cfa2328e8da]::run_compiler::{closure#1}>::{closure#0}, core[dc40836e4f3e0da7]::result::Result<(), rustc_span[7733720cb78d73dd]::ErrorGuaranteed>>
  72:        0x109109a1c - std[614eb408c5fa570d]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[5a3a908aa1aef180]::util::run_in_thread_pool_with_globals<rustc_interface[5a3a908aa1aef180]::interface::run_compiler<core[dc40836e4f3e0da7]::result::Result<(), rustc_span[7733720cb78d73dd]::ErrorGuaranteed>, rustc_driver_impl[9431cfa2328e8da]::run_compiler::{closure#1}>::{closure#0}, core[dc40836e4f3e0da7]::result::Result<(), rustc_span[7733720cb78d73dd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[dc40836e4f3e0da7]::result::Result<(), rustc_span[7733720cb78d73dd]::ErrorGuaranteed>>
  73:        0x10911d42c - <<std[614eb408c5fa570d]::thread::Builder>::spawn_unchecked_<rustc_interface[5a3a908aa1aef180]::util::run_in_thread_pool_with_globals<rustc_interface[5a3a908aa1aef180]::interface::run_compiler<core[dc40836e4f3e0da7]::result::Result<(), rustc_span[7733720cb78d73dd]::ErrorGuaranteed>, rustc_driver_impl[9431cfa2328e8da]::run_compiler::{closure#1}>::{closure#0}, core[dc40836e4f3e0da7]::result::Result<(), rustc_span[7733720cb78d73dd]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[dc40836e4f3e0da7]::result::Result<(), rustc_span[7733720cb78d73dd]::ErrorGuaranteed>>::{closure#1} as core[dc40836e4f3e0da7]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  74:        0x100c5bcd0 - std::sys::unix::thread::Thread::new::thread_start::h02c2263b1a3b4fc6
  75:        0x18ec7bfa8 - __pthread_joiner_wake

error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.71.0-nightly (fec9adcdb 2023-04-21) running on aarch64-apple-darwin

note: compiler flags: -C embed-bitcode=no -C split-debuginfo=unpacked -C debuginfo=2 -C incremental=[REDACTED]

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [def_span] looking up span for `factor_service::byte_storage::Foo::foo::{opaque#1}`
#1 [typeck] type-checking `cassandra`
#2 [typeck_item_bodies] type-checking all item bodies
#3 [analysis] running analysis passes on this crate
end of query stack
there was a panic while trying to force a dep node
try_mark_green dep node stack:
#0 typeck_item_bodies(0-0)
end of try_mark_green dep node stack
error: could not compile `factor-service` (test "cassandra_byte_storage_tests")
