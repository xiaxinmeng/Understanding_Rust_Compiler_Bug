plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.45
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: map[k].is_none()', compiler/rustc_middle/src/hir/map/collector.rs:50:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (fbf218a7f 2021-06-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
stack backtrace:
   0:     0x7f9f136f3060 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h5273f0fbd1c006fb
   1:     0x7f9f13761c0c - core::fmt::write::h9c2f5dc0511907c8
   2:     0x7f9f136e4235 - std::io::Write::write_fmt::h14e3582a4a15dba5
   3:     0x7f9f136f74f7 - std::panicking::default_hook::{{closure}}::h325bba56db83cd67
   4:     0x7f9f136f6ef1 - std::panicking::default_hook::h512e31ed380fe707
   5:     0x7f9f1420385d - rustc_driver::report_ice::h775c34be56e050cc
   6:     0x7f9f136f7d18 - std::panicking::rust_panic_with_hook::h8e27854352b22ef1
   7:     0x7f9f136f7827 - std::panicking::begin_panic_handler::{{closure}}::h7062a41d7e6c56f6
   8:     0x7f9f136f351c - std::sys_common::backtrace::__rust_end_short_backtrace::hf35148a98563df89
   9:     0x7f9f136f7789 - rust_begin_unwind
  10:     0x7f9f136b8be1 - core::panicking::panic_fmt::hb520c4f540682c9c
  11:     0x7f9f136b8dd3 - core::result::unwrap_failed::hc6bdcd9ade28635f
  12:     0x7f9f14cfe400 - rustc_query_system::query::plumbing::get_query_impl::h46e9c4ff8b45931a
  13:     0x7f9f14e67803 - rustc_query_system::query::plumbing::get_query::h8092eec6dcac42b2
  14:     0x7f9f164bc18e - rustc_middle::hir::map::Map::find::h0cbb0c468c6cc92e
  15:     0x7f9f164bf2e8 - rustc_middle::hir::map::Map::opt_span::h42158e9b3d231f7c
  16:     0x7f9f164adec5 - core::ops::function::FnOnce::call_once::h5c05252fbc4d0baa
  17:     0x7f9f14d06e9e - rustc_query_system::query::plumbing::get_query_impl::h59336026d9d55034
  18:     0x7f9f14e75fb1 - rustc_query_system::query::plumbing::get_query::hcc492a442637fc22
  19:     0x7f9f14f56c6b - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::ha6ffbcb0fa28e692
  20:     0x7f9f14f56b67 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::haaa5deacb7f5a586
  21:     0x7f9f14fdb94f - rustc_query_impl::make_query::hir_owner::h02217af551434ea7
  22:     0x7f9f14e293e0 - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::hcef6d5e18f8d325e
  23:     0x7f9f14f7aa4b - rustc_query_impl::Queries::try_collect_active_jobs::hf6e79bd2724db55a
  24:     0x7f9f14ecbcf1 - rustc_query_system::query::job::print_query_stack::h7a4278bf190dbc07
  25:     0x7f9f1431b234 - rustc_interface::interface::try_print_query_stack::hf6df9355d28ced88
  26:     0x7f9f142040b9 - rustc_driver::report_ice::h775c34be56e050cc
  27:     0x7f9f136f7d18 - std::panicking::rust_panic_with_hook::h8e27854352b22ef1
  28:     0x7f9f136f77f7 - std::panicking::begin_panic_handler::{{closure}}::h7062a41d7e6c56f6
  29:     0x7f9f136f351c - std::sys_common::backtrace::__rust_end_short_backtrace::hf35148a98563df89
  30:     0x7f9f136f7789 - rust_begin_unwind
  31:     0x7f9f136b8be1 - core::panicking::panic_fmt::hb520c4f540682c9c
  32:     0x7f9f136b8b2d - core::panicking::panic::hc07338839eea1cdf
  33:     0x7f9f16426017 - rustc_middle::hir::map::collector::NodeCollector::insert_entry::h5e8179a3b81bc750
  34:     0x7f9f164261a4 - rustc_middle::hir::map::collector::NodeCollector::insert_with_hash::he88b3b85d232ff78
  35:     0x7f9f164279e1 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_trait_ref::hab0433a1d2ef97f5
  36:     0x7f9f164b4d0f - rustc_hir::intravisit::walk_where_predicate::hd5761a06516e2086
  37:     0x7f9f164b35cc - rustc_hir::intravisit::walk_impl_item::hcd4d5e2d4af18c42
  38:     0x7f9f164276d4 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_impl_item::h480c2695ceff7233
  39:     0x7f9f164b7e14 - rustc_hir::intravisit::walk_item::h329d9b04f3ddac90
  40:     0x7f9f16426fd7 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::he0c58ed3451658d7
  41:     0x7f9f16426700 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h56d5c5181731a48a
  42:     0x7f9f164b7e9a - rustc_hir::intravisit::walk_item::h329d9b04f3ddac90
  43:     0x7f9f16426fd7 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::he0c58ed3451658d7
  44:     0x7f9f16426700 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h56d5c5181731a48a
  45:     0x7f9f164b7e9a - rustc_hir::intravisit::walk_item::h329d9b04f3ddac90
  46:     0x7f9f16426fd7 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::he0c58ed3451658d7
  47:     0x7f9f16426700 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h56d5c5181731a48a
  48:     0x7f9f164c068b - rustc_middle::hir::map::index_hir::h1d2d4fea197db3ca
  49:     0x7f9f14d22cb0 - rustc_query_system::query::plumbing::get_query_impl::h875dae9649ad9d8d
  50:     0x7f9f14e58235 - rustc_query_system::query::plumbing::get_query::h2e8f16d97951e3ae
  51:     0x7f9f164ad41a - core::ops::function::FnOnce::call_once::h0dbd9998b554bf45
  52:     0x7f9f14cfe0c7 - rustc_query_system::query::plumbing::get_query_impl::h46e9c4ff8b45931a
  53:     0x7f9f14e67803 - rustc_query_system::query::plumbing::get_query::h8092eec6dcac42b2
  54:     0x7f9f164bc18e - rustc_middle::hir::map::Map::find::h0cbb0c468c6cc92e
  55:     0x7f9f164bc65c - <rustc_middle::hir::map::Map as rustc_hir::intravisit::Map>::item::h3572b2e808d8e284
  56:     0x7f9f152517d8 - rustc_middle::hir::map::Map::visit_item_likes_in_module::h82b331764a65fea4
  57:     0x7f9f1528ee3b - rustc_passes::hir_id_validator::check_crate::h2a91ce24192248e8
  58:     0x7f9f1433080f - rustc_interface::passes::analysis::hb76d88494362c076
  59:     0x7f9f14cddb3a - rustc_query_system::query::plumbing::get_query_impl::h07878d2cbc2337ab
  60:     0x7f9f14e640c8 - rustc_query_system::query::plumbing::get_query::h72c0f73f7ff660e5
  61:     0x7f9f142707ce - rustc_interface::passes::QueryContext::enter::h46b03bff40819a7b
  62:     0x7f9f1424b828 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hb9ee6c100abc1750
  63:     0x7f9f1421a8aa - rustc_span::with_source_map::ha64dea0cc5926920
  64:     0x7f9f1424c8f8 - rustc_interface::interface::create_compiler_and_run::h12980bdad61b3f93
  65:     0x7f9f1421af22 - rustc_span::with_session_globals::h2694f79fd8cb6f1a
  66:     0x7f9f14270d9d - std::sys_common::backtrace::__rust_begin_short_backtrace::h9e5fc42faa5c26c7
  67:     0x7f9f14271276 - std::panicking::try::h2aeb1a16afbef4b8
  68:     0x7f9f14206293 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h3d5ed8e05a193447
  69:     0x7f9f137046d7 - std::sys::unix::thread::Thread::new::thread_start::h46202cfe4c4c2613
  70:     0x7f9f0e3b96db - start_thread
  71:     0x7f9f1338571f - __clone
  72:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (fbf218a7f 2021-06-10) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 4 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2018 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C metadata=eabf50a85df947a1 -C extra-filename=-eabf50a85df947a1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit status: 101
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
