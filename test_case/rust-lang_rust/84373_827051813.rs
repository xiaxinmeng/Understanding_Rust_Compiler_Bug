plain
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (41b960e80 2021-04-26) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:481:16
stack backtrace:
   0:     0x7fc71cb4af20 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9d3c9e489717f97d
   1:     0x7fc71cbbc21d - core::fmt::write::hb8bdd6c6bccb681f
   2:     0x7fc71cb3f2f5 - std::io::Write::write_fmt::ha35a665cc1270480
   3:     0x7fc71cb4f117 - std::panicking::default_hook::{{closure}}::h7d7fb73fd85d7f76
   4:     0x7fc71cb4eb1b - std::panicking::default_hook::h740070dfec8c35e3
   5:     0x7fc71d62c00d - rustc_driver::report_ice::hdd3c11546b125974
   6:     0x7fc71cb4f8ba - std::panicking::rust_panic_with_hook::hc4eb8825e604532b
   7:     0x7fc71cb4f447 - std::panicking::begin_panic_handler::{{closure}}::hf2766b2bb19805bc
   8:     0x7fc71cb4b3bc - std::sys_common::backtrace::__rust_end_short_backtrace::h4b46cdd04ec9b444
   9:     0x7fc71cb4f3a9 - rust_begin_unwind
  10:     0x7fc71cb13b41 - core::panicking::panic_fmt::h40fd73f0042911f3
  11:     0x7fc71cb13d33 - core::result::unwrap_failed::hfb6e2777deed73e2
  12:     0x7fc71e13ccdb - rustc_query_system::query::plumbing::get_query_impl::h863a630692569c0e
  13:     0x7fc71e2730c3 - rustc_query_system::query::plumbing::get_query::h76e1b748c6d6b441
  14:     0x7fc71f6f5bdb - rustc_middle::hir::map::Map::find_entry::h78381824f9592892
  15:     0x7fc71f6f8f71 - rustc_middle::hir::map::Map::opt_span::hf8db6c2c8b4a36c1
  16:     0x7fc71f6ea725 - core::ops::function::FnOnce::call_once::h067d3914ab282728
  17:     0x7fc71e51cbf5 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h1e5d2032f7c80ba2
  18:     0x7fc71e3020f1 - rustc_data_structures::stack::ensure_sufficient_stack::hfcc15fbc697c3e9e
  19:     0x7fc71e1cef3b - rustc_query_system::query::plumbing::force_query_with_job::h943ded6cb36b86ec
  20:     0x7fc71e1102eb - rustc_query_system::query::plumbing::get_query_impl::h1ed9d4ea1f06bdba
  21:     0x7fc71e278d51 - rustc_query_system::query::plumbing::get_query::h8b74769279c2ab8d
  22:     0x7fc71e3b4d8b - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::hfe097ddbe565d563
  23:     0x7fc71e3b4c8a - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::hff837fa95400e0fe
  24:     0x7fc71e59ac59 - rustc_query_impl::make_query::hir_owner::hf3c1c3e80448e339
  25:     0x7fc71e34f6db - <core::iter::adapters::map::Map<I,F> as core::iter::traits::iterator::Iterator>::fold::h9ce91e621f3dbad4
  26:     0x7fc71e62ef8d - <hashbrown::map::HashMap<K,V,S,A> as core::iter::traits::collect::Extend<(K,V)>>::extend::h6a6b0681ef71bbda
  27:     0x7fc71e2347fe - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h875d5431f3f825a2
  28:     0x7fc71e487f74 - rustc_query_impl::Queries::try_collect_active_jobs::hecb5056982eb9b11
  29:     0x7fc71e2d5e31 - rustc_query_system::query::job::print_query_stack::hb1cefba7294c620a
  30:     0x7fc71d760c84 - rustc_interface::interface::try_print_query_stack::hf35cdd2a64f936d0
  31:     0x7fc71d62c869 - rustc_driver::report_ice::hdd3c11546b125974
  32:     0x7fc71cb4f8ba - std::panicking::rust_panic_with_hook::hc4eb8825e604532b
  33:     0x7fc71cb4f447 - std::panicking::begin_panic_handler::{{closure}}::hf2766b2bb19805bc
  34:     0x7fc71cb4b3bc - std::sys_common::backtrace::__rust_end_short_backtrace::h4b46cdd04ec9b444
  35:     0x7fc71cb4f3a9 - rust_begin_unwind
  36:     0x7fc71cb13b41 - core::panicking::panic_fmt::h40fd73f0042911f3
  37:     0x7fc71cb13b02 - core::panicking::panic_bounds_check::hae9a824d0444470c
  38:     0x7fc71f8f9d5f - <rustc_span::def_id::LocalDefId as rustc_data_structures::stable_hasher::HashStable<CTX>>::hash_stable::h22ea3cc1bf9cf49d
  39:     0x7fc71f8f9535 - rustc_span::<impl rustc_data_structures::stable_hasher::HashStable<CTX> for rustc_span::span_encoding::Span>::hash_stable::hd66a815ca3d7b4c1
  40:     0x7fc71f8b0109 - rustc_middle::ich::impls_hir::<impl rustc_hir::stable_hash_impls::HashStableContext for rustc_middle::ich::hcx::StableHashingContext>::hash_hir_mod::hc720b245fe852eb5
  41:     0x7fc71f8b1919 - rustc_middle::ich::impls_hir::<impl rustc_hir::stable_hash_impls::HashStableContext for rustc_middle::ich::hcx::StableHashingContext>::hash_hir_item_like::hc1933412fd039df2
  42:     0x7fc71f6f210b - rustc_hir::stable_hash_impls::<impl rustc_data_structures::stable_hasher::HashStable<HirCtx> for rustc_hir::hir::Item>::hash_stable::hfa37bbeeb075659d
  43:     0x7fc71f7d98ab - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::h26f38d5db6277f9b
  44:     0x7fc71f7d9464 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::hb27fdb0090d75848
  45:     0x7fc71f8dc339 - rustc_hir::intravisit::walk_crate::he7a8d1ed376c887e
  46:     0x7fc71f6fa389 - rustc_middle::hir::map::index_hir::h620d2801ad4a8b3b
  47:     0x7fc71e54e2bd - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hf81bcab7d27fb67a
  48:     0x7fc71e2dd345 - rustc_data_structures::stack::ensure_sufficient_stack::h22cc713a67303625
  49:     0x7fc71e1de722 - rustc_query_system::query::plumbing::force_query_with_job::hf8dd0d7c8ef091ef
  50:     0x7fc71e12e3aa - rustc_query_system::query::plumbing::get_query_impl::h66a2237df153216f
  51:     0x7fc71e27bf53 - rustc_query_system::query::plumbing::get_query::h9afcd88418d75810
  52:     0x7fc71f6eab66 - core::ops::function::FnOnce::call_once::h9dffb10612d654e3
  53:     0x7fc71e515c7d - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::h0c425e892b6b5540
  54:     0x7fc71e302305 - rustc_data_structures::stack::ensure_sufficient_stack::hfe15fa759f588d53
  55:     0x7fc71e1dbb12 - rustc_query_system::query::plumbing::force_query_with_job::he3ad5e07cd6c51e6
  56:     0x7fc71e13ca8a - rustc_query_system::query::plumbing::get_query_impl::h863a630692569c0e
  57:     0x7fc71e2730c3 - rustc_query_system::query::plumbing::get_query::h76e1b748c6d6b441
  58:     0x7fc71f6f5bdb - rustc_middle::hir::map::Map::find_entry::h78381824f9592892
  59:     0x7fc71f6f5ee6 - rustc_middle::hir::map::Map::item::h5b46e3c45431ee7b
  60:     0x7fc71e6f6568 - rustc_middle::hir::map::Map::visit_item_likes_in_module::h7d3360f81c475d3d
  61:     0x7fc71e711e9b - rustc_passes::hir_id_validator::check_crate::h1759f3a0f336b3f3
  62:     0x7fc71d77f67f - rustc_interface::passes::analysis::hcdfb62e001d8560e
  63:     0x7fc71e54385d - rustc_query_system::dep_graph::graph::DepGraph<K>::with_task_impl::hc66d01b0819d4ba6
  64:     0x7fc71e551839 - rustc_query_system::dep_graph::graph::DepGraph<K>::with_eval_always_task::h7e6a0c7bf702a9a3
  65:     0x7fc71e2f3ce6 - rustc_data_structures::stack::ensure_sufficient_stack::ha98d34f32e80681c
  66:     0x7fc71e1c4d40 - rustc_query_system::query::plumbing::force_query_with_job::h5eb7eaede8fcf312
  67:     0x7fc71e10ef61 - rustc_query_system::query::plumbing::get_query_impl::h1835dc1b5a6913e1
  68:     0x7fc71e2606c3 - rustc_query_system::query::plumbing::get_query::h0c4db8782c774045
  69:     0x7fc71d667573 - rustc_interface::passes::QueryContext::enter::h705a705792953cb7
  70:     0x7fc71d6396eb - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h3f75259bf93b4c1b
  71:     0x7fc71d62d996 - rustc_span::with_source_map::h5fe799628f0b4c2e
  72:     0x7fc71d637873 - scoped_tls::ScopedKey<T>::set::h23badfb5f7361ad9
  73:     0x7fc71d62e343 - rustc_span::with_session_globals::h5f249d81e40dd19f
  74:     0x7fc71d63ad5d - std::sys_common::backtrace::__rust_begin_short_backtrace::h290f7e269a142637
  75:     0x7fc71d65caa9 - std::panicking::try::h412af1618ffdefed
  76:     0x7fc71d669ba3 - core::ops::function::FnOnce::call_once{{vtable.shim}}::ha5bde1c13ee98b96
  77:     0x7fc71cb5ed08 - std::sys::unix::thread::Thread::new::thread_start::h24c663c639977be7
  78:     0x7fc7178156db - start_thread
  79:     0x7fc71c7e171f - __clone
  80:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.53.0-nightly (41b960e80 2021-04-26) running on x86_64-unknown-linux-gnu

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
Build completed unsuccessfully in 0:04:43
