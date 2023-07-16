plain
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (f5d20e20b 2021-04-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:481:16
stack backtrace:
   0:     0x7ff5a7be1f20 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9d3c9e489717f97d
   1:     0x7ff5a7c5321d - core::fmt::write::hb8bdd6c6bccb681f
   2:     0x7ff5a7bd62f5 - std::io::Write::write_fmt::ha35a665cc1270480
   3:     0x7ff5a7be6117 - std::panicking::default_hook::{{closure}}::h7d7fb73fd85d7f76
   4:     0x7ff5a7be5b1b - std::panicking::default_hook::h740070dfec8c35e3
   5:     0x7ff5a86c300d - rustc_driver::report_ice::hdd3c11546b125974
   6:     0x7ff5a7be68ba - std::panicking::rust_panic_with_hook::hc4eb8825e604532b
   7:     0x7ff5a7be6447 - std::panicking::begin_panic_handler::{{closure}}::hf2766b2bb19805bc
   8:     0x7ff5a7be23bc - std::sys_common::backtrace::__rust_end_short_backtrace::h4b46cdd04ec9b444
   9:     0x7ff5a7be63a9 - rust_begin_unwind
  10:     0x7ff5a7baab41 - core::panicking::panic_fmt::h40fd73f0042911f3
  11:     0x7ff5a7baad33 - core::result::unwrap_failed::hfb6e2777deed73e2
  12:     0x7ff5a91d400b - rustc_query_system::query::plumbing::get_query_impl::h863a630692569c0e
  13:     0x7ff5a930a3f3 - rustc_query_system::query::plumbing::get_query::h76e1b748c6d6b441
  14:     0x7ff5aa78d27b - rustc_middle::hir::map::Map::find_entry::h78381824f9592892
  15:     0x7ff5aa790611 - rustc_middle::hir::map::Map::opt_span::hf8db6c2c8b4a36c1
  16:     0x7ff5aa781dc5 - core::ops::function::FnOnce::call_once::h067d3914ab282728
  17:     0x7ff5a95b3f25 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h1e5d2032f7c80ba2
  18:     0x7ff5a9399421 - rustc_data_structures::stack::ensure_sufficient_stack::hfcc15fbc697c3e9e
  19:     0x7ff5a926626b - rustc_query_system::query::plumbing::force_query_with_job::h943ded6cb36b86ec
  20:     0x7ff5a91a761b - rustc_query_system::query::plumbing::get_query_impl::h1ed9d4ea1f06bdba
  21:     0x7ff5a9310081 - rustc_query_system::query::plumbing::get_query::h8b74769279c2ab8d
  22:     0x7ff5a944c0bb - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::hfe097ddbe565d563
  23:     0x7ff5a944bfba - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::hff837fa95400e0fe
  24:     0x7ff5a9631f89 - rustc_query_impl::make_query::hir_owner::hf3c1c3e80448e339
  25:     0x7ff5a93e6a0b - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h9ce91e621f3dbad4
  26:     0x7ff5a96c62bd - <hashbrown::map::HashMap<K,V,S,A> as core::iter::traits::collect::Extend<(K,V)>>::extend::h6a6b0681ef71bbda
  27:     0x7ff5a92cbb2e - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h875d5431f3f825a2
  28:     0x7ff5a951f2a4 - rustc_query_impl::Queries::try_collect_active_jobs::hecb5056982eb9b11
  29:     0x7ff5a936d161 - rustc_query_system::query::job::print_query_stack::hb1cefba7294c620a
  30:     0x7ff5a87f7c54 - rustc_interface::interface::try_print_query_stack::hf35cdd2a64f936d0
  31:     0x7ff5a86c3869 - rustc_driver::report_ice::hdd3c11546b125974
  32:     0x7ff5a7be68ba - std::panicking::rust_panic_with_hook::hc4eb8825e604532b
  33:     0x7ff5a7be6447 - std::panicking::begin_panic_handler::{{closure}}::hf2766b2bb19805bc
  34:     0x7ff5a7be23bc - std::sys_common::backtrace::__rust_end_short_backtrace::h4b46cdd04ec9b444
  35:     0x7ff5a7be63a9 - rust_begin_unwind
  36:     0x7ff5a7baab41 - core::panicking::panic_fmt::h40fd73f0042911f3
  37:     0x7ff5a7baab02 - core::panicking::panic_bounds_check::hae9a824d0444470c
  38:     0x7ff5aa99142f - <rustc_span::def_id::LocalDefId as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable::h22ea3cc1bf9cf49d
  39:     0x7ff5aa990c05 - rustc_span::<impl rustc_data_structures::stable_hasher::HashStable<CTX> for rustc_span::span_encoding::Span>::hash_stable::hd66a815ca3d7b4c1
  40:     0x7ff5aa9477d9 - rustc_middle::ich::impls_hir::<impl rustc_hir::stable_hash_impls::HashStableContext for rustc_middle::ich::hcx::StableHashingContext>::hash_hir_mod::hc720b245fe852eb5
  41:     0x7ff5aa948fe9 - rustc_middle::ich::impls_hir::<impl rustc_hir::stable_hash_impls::HashStableContext for rustc_middle::ich::hcx::StableHashingContext>::hash_hir_item_like::hc1933412fd039df2
  42:     0x7ff5aa7897ab - rustc_hir::stable_hash_impls::<impl rustc_data_structures::stable_hasher::HashStable<HirCtx> for rustc_hir::hir::Item>::hash_stable::hfa37bbeeb075659d
  43:     0x7ff5aa870f5b - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::h26f38d5db6277f9b
  44:     0x7ff5aa870b14 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::hb27fdb0090d75848
  45:     0x7ff5aa973a09 - rustc_hir::intravisit::walk_crate::he7a8d1ed376c887e
  46:     0x7ff5aa791a29 - rustc_middle::hir::map::index_hir::h620d2801ad4a8b3b
  47:     0x7ff5a95e55ed - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hf81bcab7d27fb67a
  48:     0x7ff5a9374675 - rustc_data_structures::stack::ensure_sufficient_stack::h22cc713a67303625
  49:     0x7ff5a9275a52 - rustc_query_system::query::plumbing::force_query_with_job::hf8dd0d7c8ef091ef
  50:     0x7ff5a91c56da - rustc_query_system::query::plumbing::get_query_impl::h66a2237df153216f
  51:     0x7ff5a9313283 - rustc_query_system::query::plumbing::get_query::h9afcd88418d75810
  52:     0x7ff5aa782206 - core::ops::function::FnOnce::call_once::h9dffb10612d654e3
  53:     0x7ff5a95acfad - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h0c425e892b6b5540
  54:     0x7ff5a9399635 - rustc_data_structures::stack::ensure_sufficient_stack::hfe15fa759f588d53
  55:     0x7ff5a9272e42 - rustc_query_system::query::plumbing::force_query_with_job::he3ad5e07cd6c51e6
  56:     0x7ff5a91d3dba - rustc_query_system::query::plumbing::get_query_impl::h863a630692569c0e
  57:     0x7ff5a930a3f3 - rustc_query_system::query::plumbing::get_query::h76e1b748c6d6b441
  58:     0x7ff5aa78d27b - rustc_middle::hir::map::Map::find_entry::h78381824f9592892
  59:     0x7ff5aa78d586 - rustc_middle::hir::map::Map::item::h5b46e3c45431ee7b
  60:     0x7ff5a978d898 - rustc_middle::hir::map::Map::visit_item_likes_in_module::h7d3360f81c475d3d
  61:     0x7ff5a97a91cb - rustc_passes::hir_id_validator::check_crate::h1759f3a0f336b3f3
  62:     0x7ff5a881664f - rustc_interface::passes::analysis::hcdfb62e001d8560e
  63:     0x7ff5a95dab8d - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hc66d01b0819d4ba6
  64:     0x7ff5a95e8b69 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::h7e6a0c7bf702a9a3
  65:     0x7ff5a938b016 - rustc_data_structures::stack::ensure_sufficient_stack::ha98d34f32e80681c
  66:     0x7ff5a925c070 - rustc_query_system::query::plumbing::force_query_with_job::h5eb7eaede8fcf312
  67:     0x7ff5a91a6291 - rustc_query_system::query::plumbing::get_query_impl::h1835dc1b5a6913e1
  68:     0x7ff5a92f79f3 - rustc_query_system::query::plumbing::get_query::h0c4db8782c774045
  69:     0x7ff5a86fe573 - rustc_interface::passes::QueryContext::enter::h705a705792953cb7
  70:     0x7ff5a86d06eb - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h3f75259bf93b4c1b
  71:     0x7ff5a86c4996 - rustc_span::with_source_map::h5fe799628f0b4c2e
  72:     0x7ff5a86ce873 - scoped_tls::ScopedKey<T>::set::h23badfb5f7361ad9
  73:     0x7ff5a86c5343 - rustc_span::with_session_globals::h5f249d81e40dd19f
  74:     0x7ff5a86d1d5d - std::sys_common::backtrace::__rust_begin_short_backtrace::h290f7e269a142637
  75:     0x7ff5a86f3aa9 - std::panicking::try::h412af1618ffdefed
  76:     0x7ff5a8700ba3 - core::ops::function::FnOnce::call_once{{vtable.shim}}::ha5bde1c13ee98b96
  77:     0x7ff5a7bf5d08 - std::sys::unix::thread::Thread::new::thread_start::h24c663c639977be7
  78:     0x7ff5a28ac6db - start_thread
  79:     0x7ff5a787871f - __clone
  80:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (f5d20e20b 2021-04-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 4 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2018 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C metadata=eabf50a85df947a1 -C extra-filename=-eabf50a85df947a1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes -Z binary-dep-depinfo` (exit code: 254)
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
Build completed unsuccessfully in 0:04:40
