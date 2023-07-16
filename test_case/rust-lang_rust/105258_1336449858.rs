plain
---- [rustdoc] src/test/rustdoc/async-trait.rs stdout ----

error: rustdoc failed!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-trait/auxiliary" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc/async-trait" "--deny" "warnings" "/checkout/src/test/rustdoc/async-trait.rs" "--edition=2021"
stdout: none
--- stderr -------------------------------
thread 'rustc' panicked at 'DefId::expect_local: `DefId(20:5 ~ async_trait_dep[3bf3]::Meow::woof::{opaque#0})` isn't local', compiler/rustc_hir_analysis/src/check/compare_method.rs:514:49
   0:     0x7f93bc3eb0a5 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::hea411845e3e408da
   1:     0x7f93bc45aac8 - core::fmt::write::h63e8be5e0cfa70a9
   1:     0x7f93bc45aac8 - core::fmt::write::h63e8be5e0cfa70a9
   2:     0x7f93bc3dcdb1 - std::io::Write::write_fmt::h18c08f8feec5ec09
   3:     0x7f93bc3eaeb1 - std::sys_common::backtrace::print::h57adfb036b2c6321
   4:     0x7f93bc3ee214 - std::panicking::default_hook::{{closure}}::hc398313193f092a8
   5:     0x7f93bc3ededa - std::panicking::default_hook::hc512cf6fb1ae6652
   6:     0x7f93bce2eae4 - rustc_driver[a160918466817b6a]::DEFAULT_HOOK::{closure#0}::{closure#0}
   7:     0x7f93bc3ee984 - std::panicking::rust_panic_with_hook::h61b172dc12c873a6
   8:     0x7f93bc3ee6e9 - std::panicking::begin_panic_handler::{{closure}}::h459f6753eb797417
   9:     0x7f93bc3eb5c4 - std::sys_common::backtrace::__rust_end_short_backtrace::h9c7fa6c2c9e5262f
  10:     0x7f93bc3ee392 - rust_begin_unwind
  11:     0x7f93bc39eff3 - core::panicking::panic_fmt::h00cb1d3765b5f110
  12:     0x7f93bd7b679f - rustc_hir_analysis[54fb0a36f765165a]::check::compare_method::collect_trait_impl_trait_tys::{closure#0}
  13:     0x7f93bd7b1a89 - rustc_hir_analysis[54fb0a36f765165a]::check::compare_method::collect_trait_impl_trait_tys
  14:     0x7f93bebe1541 - rustc_query_system[a080239e8d755747]::query::plumbing::try_execute_query::<rustc_query_impl[9a77095f1790c749]::plumbing::QueryCtxt, rustc_query_system[a080239e8d755747]::query::caches::DefaultCache<rustc_span[909b40be0009fc44]::def_id::DefId, core[602b3aa9bcbb9b15]::result::Result<&std[fc26b5eef5206814]::collections::hash::map::HashMap<rustc_span[909b40be0009fc44]::def_id::DefId, rustc_middle[2c0a8b4e35451931]::ty::Ty, core[602b3aa9bcbb9b15]::hash::BuildHasherDefault<rustc_hash[10f1fe2f2195ba59]::FxHasher>>, rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>>>
  15:     0x7f93bed50b25 - rustc_query_system[a080239e8d755747]::query::plumbing::get_query::<rustc_query_impl[9a77095f1790c749]::queries::collect_trait_impl_trait_tys, rustc_query_impl[9a77095f1790c749]::plumbing::QueryCtxt>
  16:     0x7f93be8356b3 - <rustc_query_impl[9a77095f1790c749]::Queries as rustc_middle[2c0a8b4e35451931]::ty::query::QueryEngine>::collect_trait_impl_trait_tys
  17:     0x7f93bfc60c76 - <rustc_middle[2c0a8b4e35451931]::ty::context::TyCtxt>::bound_trait_impl_trait_tys
  18:     0x7f93bf8fdfcd - rustc_trait_selection[e68b1de0bada13bf]::traits::project::confirm_candidate
  19:     0x7f93bf90b2b6 - rustc_trait_selection[e68b1de0bada13bf]::traits::project::opt_normalize_projection_type
  20:     0x7f93bf8fb3ed - rustc_trait_selection[e68b1de0bada13bf]::traits::project::normalize_projection_type
  21:     0x7f93bf8f9787 - <rustc_trait_selection[e68b1de0bada13bf]::traits::project::AssocTypeNormalizer as rustc_middle[2c0a8b4e35451931]::ty::fold::TypeFolder>::fold_ty
  22:     0x7f93bd7989b4 - rustc_middle[2c0a8b4e35451931]::ty::util::fold_list::<rustc_trait_selection[e68b1de0bada13bf]::traits::project::AssocTypeNormalizer, rustc_middle[2c0a8b4e35451931]::ty::Ty, <&rustc_middle[2c0a8b4e35451931]::ty::list::List<rustc_middle[2c0a8b4e35451931]::ty::Ty> as rustc_middle[2c0a8b4e35451931]::ty::fold::TypeFoldable>::try_fold_with<rustc_trait_selection[e68b1de0bada13bf]::traits::project::AssocTypeNormalizer>::{closure#0}>
  23:     0x7f93bd5bf965 - <rustc_middle[2c0a8b4e35451931]::ty::sty::FnSig as rustc_middle[2c0a8b4e35451931]::ty::fold::TypeFoldable>::fold_with::<rustc_trait_selection[e68b1de0bada13bf]::traits::project::AssocTypeNormalizer>
  24:     0x7f93bd68dd5c - <rustc_trait_selection[e68b1de0bada13bf]::traits::project::AssocTypeNormalizer>::fold::<rustc_middle[2c0a8b4e35451931]::ty::sty::FnSig>
  25:     0x7f93bd695c73 - rustc_trait_selection[e68b1de0bada13bf]::traits::project::normalize_with_depth::<rustc_middle[2c0a8b4e35451931]::ty::sty::FnSig>
  26:     0x7f93bd63529a - <rustc_infer[a63aca6d5887c771]::infer::at::At as rustc_trait_selection[e68b1de0bada13bf]::traits::project::NormalizeExt>::normalize::<rustc_middle[2c0a8b4e35451931]::ty::sty::FnSig>
  27:     0x7f93bd5f9582 - <rustc_trait_selection[e68b1de0bada13bf]::traits::engine::ObligationCtxt>::normalize::<rustc_middle[2c0a8b4e35451931]::ty::sty::FnSig>
  28:     0x7f93bd7af7e7 - rustc_hir_analysis[54fb0a36f765165a]::check::compare_method::compare_predicate_entailment
  29:     0x7f93bd79eca7 - rustc_hir_analysis[54fb0a36f765165a]::check::compare_method::compare_impl_method
  30:     0x7f93bd613880 - rustc_hir_analysis[54fb0a36f765165a]::check::check::check_impl_items_against_trait
  31:     0x7f93bd60f539 - rustc_hir_analysis[54fb0a36f765165a]::check::check::check_item_type
  32:     0x7f93bd618caa - rustc_hir_analysis[54fb0a36f765165a]::check::check::check_mod_item_types
  33:     0x7f93bec4f213 - rustc_query_system[a080239e8d755747]::query::plumbing::try_execute_query::<rustc_query_impl[9a77095f1790c749]::plumbing::QueryCtxt, rustc_query_system[a080239e8d755747]::query::caches::VecCache<rustc_span[909b40be0009fc44]::def_id::LocalDefId, ()>>
  34:     0x7f93bed223f3 - rustc_query_system[a080239e8d755747]::query::plumbing::get_query::<rustc_query_impl[9a77095f1790c749]::queries::check_mod_item_types, rustc_query_impl[9a77095f1790c749]::plumbing::QueryCtxt>
  35:     0x7f93be85a510 - <rustc_query_impl[9a77095f1790c749]::Queries as rustc_middle[2c0a8b4e35451931]::ty::query::QueryEngine>::check_mod_item_types
  36:     0x5635038bc17a - <rustc_middle[2c0a8b4e35451931]::hir::map::Map>::for_each_module::<rustdoc[d77a20a53fbafea2]::core::run_global_ctxt::{closure#0}::{closure#0}>
  37:     0x5635037fc222 - <rustc_session[e238a9169147388c]::session::Session>::time::<(), rustdoc[d77a20a53fbafea2]::core::run_global_ctxt::{closure#0}>
  38:     0x5635037d5c41 - rustdoc[d77a20a53fbafea2]::core::run_global_ctxt
  39:     0x5635037fbcac - <rustc_session[e238a9169147388c]::session::Session>::time::<(rustdoc[d77a20a53fbafea2]::clean::types::Crate, rustdoc[d77a20a53fbafea2]::config::RenderOptions, rustdoc[d77a20a53fbafea2]::formats::cache::Cache), rustdoc[d77a20a53fbafea2]::main_args::{closure#1}::{closure#0}::{closure#1}::{closure#0}>
  40:     0x563503876095 - <rustc_interface[8d9c8267827c7019]::passes::QueryContext>::enter::<rustdoc[d77a20a53fbafea2]::main_args::{closure#1}::{closure#0}::{closure#1}, core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>>
  41:     0x5635039f9748 - <rustc_interface[8d9c8267827c7019]::interface::Compiler>::enter::<rustdoc[d77a20a53fbafea2]::main_args::{closure#1}::{closure#0}, core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>>
  42:     0x5635037a8f5f - rustc_span[909b40be0009fc44]::with_source_map::<core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>, rustc_interface[8d9c8267827c7019]::interface::run_compiler<core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>, rustdoc[d77a20a53fbafea2]::main_args::{closure#1}>::{closure#0}::{closure#0}>
  43:     0x56350373ddd4 - <scoped_tls[155b4886cdd22140]::ScopedKey<rustc_span[909b40be0009fc44]::SessionGlobals>>::set::<rustc_interface[8d9c8267827c7019]::interface::run_compiler<core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>, rustdoc[d77a20a53fbafea2]::main_args::{closure#1}>::{closure#0}, core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>>
  44:     0x5635037b60f9 - std[fc26b5eef5206814]::sys_common::backtrace::__rust_begin_short_backtrace::<rustc_interface[8d9c8267827c7019]::util::run_in_thread_pool_with_globals<rustc_interface[8d9c8267827c7019]::interface::run_compiler<core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>, rustdoc[d77a20a53fbafea2]::main_args::{closure#1}>::{closure#0}, core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>>
  45:     0x56350387fe16 - std[fc26b5eef5206814]::panicking::try::<core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>, core[602b3aa9bcbb9b15]::panic::unwind_safe::AssertUnwindSafe<<std[fc26b5eef5206814]::thread::Builder>::spawn_unchecked_<rustc_interface[8d9c8267827c7019]::util::run_in_thread_pool_with_globals<rustc_interface[8d9c8267827c7019]::interface::run_compiler<core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>, rustdoc[d77a20a53fbafea2]::main_args::{closure#1}>::{closure#0}, core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>>::{closure#1}::{closure#0}>>
  46:     0x563503903bd5 - <<std[fc26b5eef5206814]::thread::Builder>::spawn_unchecked_<rustc_interface[8d9c8267827c7019]::util::run_in_thread_pool_with_globals<rustc_interface[8d9c8267827c7019]::interface::run_compiler<core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>, rustdoc[d77a20a53fbafea2]::main_args::{closure#1}>::{closure#0}, core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>>::{closure#0}::{closure#0}, core[602b3aa9bcbb9b15]::result::Result<(), rustc_errors[3bfff2e1ae44d057]::ErrorGuaranteed>>::{closure#1} as core[602b3aa9bcbb9b15]::ops::function::FnOnce<()>>::call_once::{shim:vtable#0}
  47:     0x7f93bc3faf9e - std::sys::unix::thread::Thread::new::thread_start::h471c613b26fa8067
  48:     0x7f93bc087b43 - <unknown>
  49:     0x7f93bc119a00 - <unknown>
  50:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.67.0-nightly (4674f6afc 2022-12-04) running on x86_64-unknown-linux-gnu

query stack during panic:
#0 [collect_trait_impl_trait_tys] comparing an impl and trait method signature, inferring any hidden `impl Trait` types in the process
#1 [check_mod_item_types] checking item types in top-level module
------------------------------------------



