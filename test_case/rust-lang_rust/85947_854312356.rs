plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.43
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: map[k].is_none()', compiler/rustc_middle/src/hir/map/collector.rs:50:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (731cce589 2021-06-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:481:16
stack backtrace:
   0:     0x7fdc954eb070 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h70326a0434640970
   1:     0x7fdc95559dec - core::fmt::write::h9c2f5dc0511907c8
   2:     0x7fdc954dc3b5 - std::io::Write::write_fmt::hca3a5e7182fb92e0
   3:     0x7fdc954ef497 - std::panicking::default_hook::{{closure}}::hde5192802311335c
   4:     0x7fdc954eee91 - std::panicking::default_hook::hd3aaff52500fccbf
   5:     0x7fdc95ff4bed - rustc_driver::report_ice::h507f8f2e00908254
   6:     0x7fdc954efd7b - std::panicking::rust_panic_with_hook::hd6c0a49e8c2b85f9
   7:     0x7fdc954ef7c7 - std::panicking::begin_panic_handler::{{closure}}::hd4500351440fa19b
   8:     0x7fdc954eb50c - std::sys_common::backtrace::__rust_end_short_backtrace::h53c64d13a53c5062
   9:     0x7fdc954ef729 - rust_begin_unwind
  10:     0x7fdc954b0c21 - core::panicking::panic_fmt::hb520c4f540682c9c
  11:     0x7fdc954b0e13 - core::result::unwrap_failed::hc6bdcd9ade28635f
  12:     0x7fdc96ae9629 - rustc_query_system::query::plumbing::get_query_impl::h46e9c4ff8b45931a
  13:     0x7fdc96c4ccf3 - rustc_query_system::query::plumbing::get_query::h8092eec6dcac42b2
  14:     0x7fdc982775fe - rustc_middle::hir::map::Map::find::h0cbb0c468c6cc92e
  15:     0x7fdc9827a758 - rustc_middle::hir::map::Map::opt_span::h42158e9b3d231f7c
  16:     0x7fdc98269905 - core::ops::function::FnOnce::call_once::h5c05252fbc4d0baa
  17:     0x7fdc96af1f9f - rustc_query_system::query::plumbing::get_query_impl::h59336026d9d55034
  18:     0x7fdc96c5b171 - rustc_query_system::query::plumbing::get_query::hcc492a442637fc22
  19:     0x7fdc96d4307b - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::ha6ffbcb0fa28e692
  20:     0x7fdc96d42f77 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::haaa5deacb7f5a586
  21:     0x7fdc96dcbe6f - rustc_query_impl::make_query::hir_owner::h02217af551434ea7
  22:     0x7fdc96c092da - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h2c70712ab648184a
  23:     0x7fdc96d6bd3b - rustc_query_impl::Queries::try_collect_active_jobs::h062226da7246dfdd
  24:     0x7fdc96cb765c - rustc_query_system::query::job::print_query_stack::h7a4278bf190dbc07
  25:     0x7fdc9610f624 - rustc_interface::interface::try_print_query_stack::h63b291a44bee08b0
  26:     0x7fdc95ff5449 - rustc_driver::report_ice::h507f8f2e00908254
  27:     0x7fdc954efd7b - std::panicking::rust_panic_with_hook::hd6c0a49e8c2b85f9
  28:     0x7fdc954ef797 - std::panicking::begin_panic_handler::{{closure}}::hd4500351440fa19b
  29:     0x7fdc954eb50c - std::sys_common::backtrace::__rust_end_short_backtrace::h53c64d13a53c5062
  30:     0x7fdc954ef729 - rust_begin_unwind
  31:     0x7fdc954b0c21 - core::panicking::panic_fmt::hb520c4f540682c9c
  32:     0x7fdc954b0b6d - core::panicking::panic::hc07338839eea1cdf
  33:     0x7fdc981e1b71 - rustc_middle::hir::map::collector::NodeCollector::insert_entry::h5e8179a3b81bc750
  34:     0x7fdc981e1cf4 - rustc_middle::hir::map::collector::NodeCollector::insert_with_hash::he88b3b85d232ff78
  35:     0x7fdc981e3531 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_trait_ref::hab0433a1d2ef97f5
  36:     0x7fdc982702bf - rustc_hir::intravisit::walk_where_predicate::hd5761a06516e2086
  37:     0x7fdc9826eb7c - rustc_hir::intravisit::walk_impl_item::hcd4d5e2d4af18c42
  38:     0x7fdc981e3224 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_impl_item::h480c2695ceff7233
  39:     0x7fdc982733c4 - rustc_hir::intravisit::walk_item::h329d9b04f3ddac90
  40:     0x7fdc981e2b27 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::he0c58ed3451658d7
  41:     0x7fdc981e2250 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h56d5c5181731a48a
  42:     0x7fdc9827344a - rustc_hir::intravisit::walk_item::h329d9b04f3ddac90
  43:     0x7fdc981e2b27 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::he0c58ed3451658d7
  44:     0x7fdc981e2250 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h56d5c5181731a48a
  45:     0x7fdc9827344a - rustc_hir::intravisit::walk_item::h329d9b04f3ddac90
  46:     0x7fdc981e2b27 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::he0c58ed3451658d7
  47:     0x7fdc981e2250 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h56d5c5181731a48a
  48:     0x7fdc9827bafb - rustc_middle::hir::map::index_hir::h1d2d4fea197db3ca
  49:     0x7fdc96b0ed51 - rustc_query_system::query::plumbing::get_query_impl::h875dae9649ad9d8d
  50:     0x7fdc96c3d725 - rustc_query_system::query::plumbing::get_query::h2e8f16d97951e3ae
  51:     0x7fdc982690fa - core::ops::function::FnOnce::call_once::h0dbd9998b554bf45
  52:     0x7fdc96ae92f1 - rustc_query_system::query::plumbing::get_query_impl::h46e9c4ff8b45931a
  53:     0x7fdc96c4ccf3 - rustc_query_system::query::plumbing::get_query::h8092eec6dcac42b2
  54:     0x7fdc982775fe - rustc_middle::hir::map::Map::find::h0cbb0c468c6cc92e
  55:     0x7fdc98277acc - <rustc_middle::hir::map::Map as rustc_hir::intravisit::Map>::item::h3572b2e808d8e284
  56:     0x7fdc97033b98 - rustc_middle::hir::map::Map::visit_item_likes_in_module::h82b331764a65fea4
  57:     0x7fdc970712bb - rustc_passes::hir_id_validator::check_crate::h2a91ce24192248e8
  58:     0x7fdc96124df3 - rustc_interface::passes::analysis::h65636812d2f16d61
  59:     0x7fdc96ac660b - rustc_query_system::query::plumbing::get_query_impl::h07878d2cbc2337ab
  60:     0x7fdc96c495b8 - rustc_query_system::query::plumbing::get_query::h72c0f73f7ff660e5
  61:     0x7fdc96043335 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hc6874c2f4c3d8ea9
  62:     0x7fdc9600acba - rustc_span::with_source_map::h5d4dd1995537e0bf
  63:     0x7fdc96044788 - rustc_interface::interface::create_compiler_and_run::had4124cebd81e1a1
  64:     0x7fdc9600b32f - rustc_span::with_session_globals::hcdaac4904f18e596
  65:     0x7fdc9604511d - std::sys_common::backtrace::__rust_begin_short_backtrace::h1d12817e221dcaca
  66:     0x7fdc96045276 - std::panicking::try::h44a1cead8e0129dd
  67:     0x7fdc95ff7c93 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h43c9215331efad6e
  68:     0x7fdc954fc907 - std::sys::unix::thread::Thread::new::thread_start::ha065aefdd908c136
  69:     0x7fdc901b16db - start_thread
  70:     0x7fdc9517d71f - __clone
  71:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.54.0-nightly (731cce589 2021-06-04) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 4 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2018 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C metadata=eabf50a85df947a1 -C extra-filename=-eabf50a85df947a1 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes -Z binary-dep-depinfo` (exit status: 254)
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit status: 101
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
