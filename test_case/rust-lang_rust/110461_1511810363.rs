plain
   Compiling memchr v2.5.0
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.91
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'expected an impl item, found Item { ident: FullOps#0, owner_id: DefId(0:301 ~ core[ba0d]::num::bignum::FullOps), kind: Trait(No, Normal, Generics { params: [], predicates: [], has_where_clause_predicates: false, where_clause_span: library/core/src/num/bignum.rs:23:25: 23:25 (#0), span: library/core/src/num/bignum.rs:23:18: 23:18 (#0) }, [Trait(PolyTraitRef { bound_generic_params: [], trait_ref: TraitRef { path: Path { span: library/core/src/num/bignum.rs:23:20: 23:25 (#0), res: Def(Trait, DefId(0:2675 ~ core[ba0d]::marker::Sized)), segments: [PathSegment { ident: Sized#0, hir_id: HirId(DefId(0:301 ~ core[ba0d]::num::bignum::FullOps).1), res: Def(Trait, DefId(0:2675 ~ core[ba0d]::marker::Sized)), args: None, infer_args: false }] }, hir_ref_id: HirId(DefId(0:301 ~ core[ba0d]::num::bignum::FullOps).2) }, span: library/core/src/num/bignum.rs:23:20: 23:25 (#0) }, None)], [TraitItemRef { id: TraitItemId { owner_id: DefId(0:302 ~ core[ba0d]::num::bignum::FullOps::full_mul_add) }, ident: full_mul_add#0, kind: Fn { has_self: true }, span: library/core/src/num/bignum.rs:26:5: 26:95 (#0) }, TraitItemRef { id: TraitItemId { owner_id: DefId(0:303 ~ core[ba0d]::num::bignum::FullOps::full_div_rem) }, ident: full_div_rem#0, kind: Fn { has_self: true }, span: library/core/src/num/bignum.rs:30:5: 31:52 (#0) }]), span: library/core/src/num/bignum.rs:23:1: 32:2 (#0), vis_span: library/core/src/num/bignum.rs:23:1: 23:4 (#0) }', compiler/rustc_ast_lowering/src/item.rs:142:49
   0:     0x7f31ffefac45 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h328e207c0fd2205c
   1:     0x7f31fff671d8 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f31ffeee8e1 - std::io::Write::write_fmt::hf1d0b0080eeddb6d
   3:     0x7f31ffefaa55 - std::sys_common::backtrace::print::h031dcf236c316efb
   3:     0x7f31ffefaa55 - std::sys_common::backtrace::print::h031dcf236c316efb
   4:     0x7f31ffefdc14 - std::panicking::default_hook::{{closure}}::h9f518d39880907e1
   5:     0x7f31ffefd902 - std::panicking::default_hook::h689a8190a575275d
   6:     0x7f32009655b5 - rustc_driver_impl[9a3c472079608d22]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f31ffefe349 - std::panicking::rust_panic_with_hook::h527f6e576e1297bc
   8:     0x7f31ffefe0c9 - std::panicking::begin_panic_handler::{{closure}}::hbd4fa4e569c60071
   9:     0x7f31ffefb0f6 - std::sys_common::backtrace::__rust_end_short_backtrace::hb8d4f72b868865f5
  10:     0x7f31ffefdd92 - rust_begin_unwind
  11:     0x7f31ffeb70b3 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7f320383207b - <rustc_hir[eeef8644f984b7e7]::hir::Item>::expect_failed
  13:     0x7f3203831ff0 - <rustc_hir[eeef8644f984b7e7]::hir::Item>::expect_impl
  14:     0x7f320173892e - <rustc_ast_lowering[710de97a5e33d12f]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[710de97a5e33d12f]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[710de97a5e33d12f]::item::ItemLowerer>::lower_assoc_item::{closure#0}>::{closure#0}>
  15:     0x7f32017dcb3a - <rustc_ast_lowering[710de97a5e33d12f]::item::ItemLowerer>::lower_node
  16:     0x7f320172cdc0 - rustc_ast_lowering[710de97a5e33d12f]::lower_to_hir
  17:     0x7f320243d4f9 - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::execute_job_non_incr<rustc_query_impl[e14e60d8f45db920]::queries::hir_crate, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>
  18:     0x7f32027006bd - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::hir_crate, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  19:     0x7f3202586906 - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::hir_crate
  20:     0x7f3203613fdd - <rustc_middle[b8dd7aca70be3dc8]::hir::provide::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<(rustc_middle[b8dd7aca70be3dc8]::ty::context::TyCtxt, rustc_hir[eeef8644f984b7e7]::hir_id::OwnerId)>>::call_once
  21:     0x7f320243d610 - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::execute_job_non_incr<rustc_query_impl[e14e60d8f45db920]::queries::hir_owner, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 16usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 16usize]>>
  22:     0x7f3202700dd6 - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::hir_owner, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  23:     0x7f3202587b51 - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::hir_owner
  24:     0x7f32035608da - <rustc_middle[b8dd7aca70be3dc8]::hir::map::Map>::get_module
  25:     0x7f320356589f - rustc_middle[b8dd7aca70be3dc8]::hir::map::hir_crate_items
  26:     0x7f3202432846 - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::execute_job_non_incr<rustc_query_impl[e14e60d8f45db920]::queries::hir_crate_items, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>
  27:     0x7f320267230d - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::hir_crate_items, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  28:     0x7f3202586f06 - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::hir_crate_items
  29:     0x7f3201a8365b - rustc_passes[f2874c0291f044e1]::hir_id_validator::check_crate
  30:     0x7f3200a1f6b9 - rustc_interface[458b19741ed3cc5]::passes::analysis
  31:     0x7f320243ce7c - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::execute_job_non_incr<rustc_query_impl[e14e60d8f45db920]::queries::analysis, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 1usize]>>
  32:     0x7f32026fb776 - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::analysis, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  33:     0x7f320258bfca - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::analysis
  34:     0x7f32009a809d - <rustc_interface[458b19741ed3cc5]::queries::QueryResult<&rustc_middle[b8dd7aca70be3dc8]::ty::context::GlobalCtxt>>::enter::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  35:     0x7f32009baa16 - <rustc_interface[458b19741ed3cc5]::interface::Compiler>::enter::<rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}::{closure#2}, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::option::Option<rustc_interface[458b19741ed3cc5]::queries::Linker>, rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>
  36:     0x7f32009d7be0 - rustc_span[f084618c3ebd7322]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_interface[458b19741ed3cc5]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  37:     0x7f320096fd82 - <scoped_tls[b5fd76ccba344725]::ScopedKey<rustc_span[f084618c3ebd7322]::SessionGlobals>>::set::<rustc_interface[458b19741ed3cc5]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>
  38:     0x7f32009a034a - std[ea8c57938de3e3e8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[458b19741ed3cc5]::util::run_in_thread_pool_with_globals<rustc_interface[458b19741ed3cc5]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>
  39:     0x7f3200969328 - std[ea8c57938de3e3e8]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[ea8c57938de3e3e8]::thread::Builder>::spawn_unchecked_<rustc_interface[458b19741ed3cc5]::util::run_in_thread_pool_with_globals<rustc_interface[458b19741ed3cc5]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  40:     0x7f3200979376 - <<std[ea8c57938de3e3e8]::thread::Builder>::spawn_unchecked_<rustc_interface[458b19741ed3cc5]::util::run_in_thread_pool_with_globals<rustc_interface[458b19741ed3cc5]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  41:     0x7f31fff0a58e - std::sys::unix::thread::Thread::new::thread_start::h29b0cb2325153421
  42:     0x7f31ffca7b43 - <unknown>
  43:     0x7f31ffd39a00 - <unknown>
  44:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (f9ae67674 2023-04-17) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'called `Option::unwrap()` on a `None` value', /checkout/compiler/rustc_query_system/src/query/job.rs:139:44
   0:     0x7f31ffefac45 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h328e207c0fd2205c
   1:     0x7f31fff671d8 - core::fmt::write::hf3b1e4fb936f95f6
   2:     0x7f31ffeee8e1 - std::io::Write::write_fmt::hf1d0b0080eeddb6d
   3:     0x7f31ffefaa55 - std::sys_common::backtrace::print::h031dcf236c316efb
   3:     0x7f31ffefaa55 - std::sys_common::backtrace::print::h031dcf236c316efb
   4:     0x7f31ffefdc14 - std::panicking::default_hook::{{closure}}::h9f518d39880907e1
   5:     0x7f31ffefd902 - std::panicking::default_hook::h689a8190a575275d
   6:     0x7f32009655b5 - rustc_driver_impl[9a3c472079608d22]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f31ffefe349 - std::panicking::rust_panic_with_hook::h527f6e576e1297bc
   8:     0x7f31ffefe082 - std::panicking::begin_panic_handler::{{closure}}::hbd4fa4e569c60071
   9:     0x7f31ffefb0f6 - std::sys_common::backtrace::__rust_end_short_backtrace::hb8d4f72b868865f5
  10:     0x7f31ffefdd92 - rust_begin_unwind
  11:     0x7f31ffeb70b3 - core::panicking::panic_fmt::hb12531409cdb4ed6
  12:     0x7f31ffeb714d - core::panicking::panic::hc61b82565d1e8446
  13:     0x7f3202363b7d - <rustc_query_system[47afec1e8a2d425a]::query::job::QueryJobId>::find_cycle_in_stack::<rustc_middle[b8dd7aca70be3dc8]::dep_graph::dep_node::DepKind>
  14:     0x7f3200853aff - rustc_query_system[47afec1e8a2d425a]::query::plumbing::cycle_error::<rustc_query_impl[e14e60d8f45db920]::queries::limits, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  15:     0x7f32026f950b - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::limits, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  16:     0x7f32025f40a7 - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::limits
  17:     0x7f32036477f9 - <rustc_middle[b8dd7aca70be3dc8]::ty::context::TyCtxt>::type_length_limit
  18:     0x7f3203635876 - <rustc_middle[b8dd7aca70be3dc8]::ty::context::TyCtxt>::def_path_str_with_substs
  19:     0x7f320359d761 - rustc_middle[b8dd7aca70be3dc8]::query::descs::hir_attrs
  20:     0x7f320227a4a8 - rustc_query_impl[e14e60d8f45db920]::plumbing::create_query_frame::<rustc_hir[eeef8644f984b7e7]::hir_id::OwnerId>
  21:     0x7f3202612fe3 - <rustc_query_impl[e14e60d8f45db920]::query_structs::hir_attrs::{closure#0}::{closure#0} as core[54ab13d2a06817e1]::ops::function::FnOnce<(rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt, rustc_hir[eeef8644f984b7e7]::hir_id::OwnerId)>>::call_once
  22:     0x7f3202621102 - <rustc_query_system[47afec1e8a2d425a]::query::plumbing::QueryState<rustc_hir[eeef8644f984b7e7]::hir_id::OwnerId, rustc_middle[b8dd7aca70be3dc8]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  23:     0x7f3202584365 - <rustc_query_impl[e14e60d8f45db920]::Queries>::try_collect_active_jobs
  24:     0x7f320085449f - rustc_query_system[47afec1e8a2d425a]::query::plumbing::cycle_error::<rustc_query_impl[e14e60d8f45db920]::queries::hir_crate, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  25:     0x7f3202700858 - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::hir_crate, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  26:     0x7f3202586906 - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::hir_crate
  27:     0x7f32036137ad - <rustc_middle[b8dd7aca70be3dc8]::hir::provide::{closure#5} as core[54ab13d2a06817e1]::ops::function::FnOnce<(rustc_middle[b8dd7aca70be3dc8]::ty::context::TyCtxt, rustc_hir[eeef8644f984b7e7]::hir_id::OwnerId)>>::call_once
  28:     0x7f320243d45f - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e14e60d8f45db920]::queries::hir_attrs, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>
  29:     0x7f32026ffd54 - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::hir_attrs, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  30:     0x7f3202589431 - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::hir_attrs
  31:     0x7f32035625ff - <rustc_middle[b8dd7aca70be3dc8]::hir::map::Map>::attrs
  32:     0x7f320350b944 - <rustc_middle[b8dd7aca70be3dc8]::middle::limits::provide::{closure#0} as core[54ab13d2a06817e1]::ops::function::FnOnce<(rustc_middle[b8dd7aca70be3dc8]::ty::context::TyCtxt, ())>>::call_once
  33:     0x7f320243cbd1 - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_load_from_disk_and_cache_in_memory<rustc_query_impl[e14e60d8f45db920]::queries::limits, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#1}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 32usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 32usize]>>
  34:     0x7f32026f9081 - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::limits, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  35:     0x7f32025f40a7 - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::limits
  36:     0x7f32036477f9 - <rustc_middle[b8dd7aca70be3dc8]::ty::context::TyCtxt>::type_length_limit
  37:     0x7f3203635876 - <rustc_middle[b8dd7aca70be3dc8]::ty::context::TyCtxt>::def_path_str_with_substs
  38:     0x7f320359d361 - rustc_middle[b8dd7aca70be3dc8]::query::descs::hir_owner
  39:     0x7f320227a4a8 - rustc_query_impl[e14e60d8f45db920]::plumbing::create_query_frame::<rustc_hir[eeef8644f984b7e7]::hir_id::OwnerId>
  40:     0x7f3202613053 - <rustc_query_impl[e14e60d8f45db920]::query_structs::hir_owner::{closure#0}::{closure#0} as core[54ab13d2a06817e1]::ops::function::FnOnce<(rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt, rustc_hir[eeef8644f984b7e7]::hir_id::OwnerId)>>::call_once
  41:     0x7f3202621102 - <rustc_query_system[47afec1e8a2d425a]::query::plumbing::QueryState<rustc_hir[eeef8644f984b7e7]::hir_id::OwnerId, rustc_middle[b8dd7aca70be3dc8]::dep_graph::dep_node::DepKind>>::try_collect_active_jobs::<rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  42:     0x7f3202584365 - <rustc_query_impl[e14e60d8f45db920]::Queries>::try_collect_active_jobs
  43:     0x7f3202444326 - rustc_query_system[47afec1e8a2d425a]::query::job::print_query_stack::<rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  44:     0x7f3200ae224a - rustc_interface[458b19741ed3cc5]::interface::try_print_query_stack
  45:     0x7f32009660fa - rustc_driver_impl[9a3c472079608d22]::report_ice
  46:     0x7f3200965608 - rustc_driver_impl[9a3c472079608d22]::DEFAULT_HOOK::{closure#0}::{closure#0}
  47:     0x7f31ffefe349 - std::panicking::rust_panic_with_hook::h527f6e576e1297bc
  48:     0x7f31ffefe0c9 - std::panicking::begin_panic_handler::{{closure}}::hbd4fa4e569c60071
  49:     0x7f31ffefb0f6 - std::sys_common::backtrace::__rust_end_short_backtrace::hb8d4f72b868865f5
  50:     0x7f31ffefdd92 - rust_begin_unwind
  51:     0x7f31ffeb70b3 - core::panicking::panic_fmt::hb12531409cdb4ed6
  52:     0x7f320383207b - <rustc_hir[eeef8644f984b7e7]::hir::Item>::expect_failed
  53:     0x7f3203831ff0 - <rustc_hir[eeef8644f984b7e7]::hir::Item>::expect_impl
  54:     0x7f320173892e - <rustc_ast_lowering[710de97a5e33d12f]::LoweringContext>::with_hir_id_owner::<<rustc_ast_lowering[710de97a5e33d12f]::item::ItemLowerer>::with_lctx<<rustc_ast_lowering[710de97a5e33d12f]::item::ItemLowerer>::lower_assoc_item::{closure#0}>::{closure#0}>
  55:     0x7f32017dcb3a - <rustc_ast_lowering[710de97a5e33d12f]::item::ItemLowerer>::lower_node
  56:     0x7f320172cdc0 - rustc_ast_lowering[710de97a5e33d12f]::lower_to_hir
  57:     0x7f320243d4f9 - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::execute_job_non_incr<rustc_query_impl[e14e60d8f45db920]::queries::hir_crate, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>
  58:     0x7f32027006bd - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::hir_crate, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  59:     0x7f3202586906 - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::hir_crate
  60:     0x7f3203613fdd - <rustc_middle[b8dd7aca70be3dc8]::hir::provide::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<(rustc_middle[b8dd7aca70be3dc8]::ty::context::TyCtxt, rustc_hir[eeef8644f984b7e7]::hir_id::OwnerId)>>::call_once
  61:     0x7f320243d610 - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::execute_job_non_incr<rustc_query_impl[e14e60d8f45db920]::queries::hir_owner, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 16usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 16usize]>>
  62:     0x7f3202700dd6 - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::hir_owner, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  63:     0x7f3202587b51 - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::hir_owner
  64:     0x7f32035608da - <rustc_middle[b8dd7aca70be3dc8]::hir::map::Map>::get_module
  65:     0x7f320356589f - rustc_middle[b8dd7aca70be3dc8]::hir::map::hir_crate_items
  66:     0x7f3202432846 - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::execute_job_non_incr<rustc_query_impl[e14e60d8f45db920]::queries::hir_crate_items, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 8usize]>>
  67:     0x7f320267230d - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::hir_crate_items, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  68:     0x7f3202586f06 - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::hir_crate_items
  69:     0x7f3201a8365b - rustc_passes[f2874c0291f044e1]::hir_id_validator::check_crate
  70:     0x7f3200a1f6b9 - rustc_interface[458b19741ed3cc5]::passes::analysis
  71:     0x7f320243ce7c - <std[ea8c57938de3e3e8]::thread::local::LocalKey<core[54ab13d2a06817e1]::cell::Cell<*const ()>>>::with::<rustc_middle[b8dd7aca70be3dc8]::ty::context::tls::enter_context<rustc_query_system[47afec1e8a2d425a]::query::plumbing::execute_job_non_incr<rustc_query_impl[e14e60d8f45db920]::queries::analysis, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 1usize]>>::{closure#0}, rustc_middle[b8dd7aca70be3dc8]::query::erase::Erased<[u8; 1usize]>>
  72:     0x7f32026fb776 - rustc_query_system[47afec1e8a2d425a]::query::plumbing::try_execute_query::<rustc_query_impl[e14e60d8f45db920]::queries::analysis, rustc_query_impl[e14e60d8f45db920]::plumbing::QueryCtxt>
  73:     0x7f320258bfca - <rustc_query_impl[e14e60d8f45db920]::Queries as rustc_middle[b8dd7aca70be3dc8]::ty::query::QueryEngine>::analysis
  74:     0x7f32009a809d - <rustc_interface[458b19741ed3cc5]::queries::QueryResult<&rustc_middle[b8dd7aca70be3dc8]::ty::context::GlobalCtxt>>::enter::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}::{closure#2}::{closure#4}>
  75:     0x7f32009baa16 - <rustc_interface[458b19741ed3cc5]::interface::Compiler>::enter::<rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}::{closure#2}, core[54ab13d2a06817e1]::result::Result<core[54ab13d2a06817e1]::option::Option<rustc_interface[458b19741ed3cc5]::queries::Linker>, rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>
  76:     0x7f32009d7be0 - rustc_span[f084618c3ebd7322]::set_source_map::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_interface[458b19741ed3cc5]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}>::{closure#0}::{closure#0}>
  77:     0x7f320096fd82 - <scoped_tls[b5fd76ccba344725]::ScopedKey<rustc_span[f084618c3ebd7322]::SessionGlobals>>::set::<rustc_interface[458b19741ed3cc5]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>
  78:     0x7f32009a034a - std[ea8c57938de3e3e8]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[458b19741ed3cc5]::util::run_in_thread_pool_with_globals<rustc_interface[458b19741ed3cc5]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>
  79:     0x7f3200969328 - std[ea8c57938de3e3e8]::panicking::try::<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, core[54ab13d2a06817e1]::panic::unwind_safe::AssertUnwindSafe<<std[ea8c57938de3e3e8]::thread::Builder>::spawn_unchecked_<rustc_interface[458b19741ed3cc5]::util::run_in_thread_pool_with_globals<rustc_interface[458b19741ed3cc5]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  80:     0x7f3200979376 - <<std[ea8c57938de3e3e8]::thread::Builder>::spawn_unchecked_<rustc_interface[458b19741ed3cc5]::util::run_in_thread_pool_with_globals<rustc_interface[458b19741ed3cc5]::interface::run_compiler<core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>, rustc_driver_impl[9a3c472079608d22]::run_compiler::{closure#1}>::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[54ab13d2a06817e1]::result::Result<(), rustc_span[f084618c3ebd7322]::ErrorGuaranteed>>::{closure#1} as core[54ab13d2a06817e1]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  81:     0x7f31fff0a58e - std::sys::unix::thread::Thread::new::thread_start::h29b0cb2325153421
  82:     0x7f31ffca7b43 - <unknown>
  83:     0x7f31ffd39a00 - <unknown>
  84:                0x0 - <unknown>
error: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md


note: rustc 1.71.0-nightly (f9ae67674 2023-04-17) running on x86_64-unknown-linux-gnu

note: compiler flags: --crate-type lib -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Z unstable-options -C symbol-mangling-version=legacy -Z unstable-options -Z unstable-options -Z macro-backtrace -C link-args=-Wl,-z,origin -C link-args=-Wl,-rpath,$ORIGIN/../lib -C split-debuginfo=off -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -Z inline-mir -C embed-bitcode=yes -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 6 (SIGABRT) (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2021 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts,future-incompat --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -Zunstable-options --check-cfg 'names()' --check-cfg 'values()' -C metadata=fa992565d2130c71 -C extra-filename=-fa992565d2130c71 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Csymbol-mangling-version=legacy -Zunstable-options -Zunstable-options '--check-cfg=values(bootstrap)' '--check-cfg=values(stdarch_intel_sde)' '--check-cfg=values(no_fp_fmt_parse)' '--check-cfg=values(no_global_oom_handling)' '--check-cfg=values(no_rc)' '--check-cfg=values(no_sync)' '--check-cfg=values(freebsd12)' '--check-cfg=values(freebsd13)' '--check-cfg=values(backtrace_in_libstd)' '--check-cfg=values(target_env,"libnx")' '--check-cfg=values(target_arch,"asmjs","spirv","nvptx","xtensa","loongarch64")' '--check-cfg=values(target_env,"ohos")' -Zmacro-backtrace -Clink-args=-Wl,-z,origin '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Csplit-debuginfo=off -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Zinline-mir -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
Build completed unsuccessfully in 0:03:04
