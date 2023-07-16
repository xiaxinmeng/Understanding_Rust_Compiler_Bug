plain
   Compiling libc v0.2.93
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.46
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'assertion failed: map[k].is_none()', compiler/rustc_middle/src/hir/map/collector.rs:50:5

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (f7b6ba9fc 2021-07-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
stack backtrace:
   0:     0x7f5579e843b0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a6a93b131f2f4a9
   1:     0x7f5579ef6a2c - core::fmt::write::h554baf3029e46fd4
   2:     0x7f5579e75635 - std::io::Write::write_fmt::h6174ac9fdbb0def8
   3:     0x7f5579e887c7 - std::panicking::default_hook::{{closure}}::h66774db567c52595
   4:     0x7f5579e881cf - std::panicking::default_hook::h9818569ee0b6b08d
   5:     0x7f557a98e711 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::hdcec51b99f0e3431
   6:     0x7f5579e88fe8 - std::panicking::rust_panic_with_hook::hf465f5595671703e
   7:     0x7f5579e88af7 - std::panicking::begin_panic_handler::{{closure}}::h680bdb0473565baa
   8:     0x7f5579e8486c - std::sys_common::backtrace::__rust_end_short_backtrace::h7b8b3b56e09356cf
   9:     0x7f5579e88a59 - rust_begin_unwind
  10:     0x7f5579e4ef51 - core::panicking::panic_fmt::h137d40a743197ca3
  11:     0x7f5579e4f143 - core::result::unwrap_failed::h211c608cd1950a84
  12:     0x7f557b4cfe6f - rustc_query_system::query::plumbing::get_query_impl::hbb51039add9caca5
  13:     0x7f557b5db86d - rustc_query_system::query::plumbing::get_query::h03a5f2481f3cb4c1
  14:     0x7f557cc3621e - rustc_middle::hir::map::Map::find::h67f327cad512a5a5
  15:     0x7f557cc393b8 - rustc_middle::hir::map::Map::opt_span::h89781a143268facb
  16:     0x7f557cc396a3 - rustc_middle::hir::map::Map::span_if_local::h2bef0a4f2f5c2242
  17:     0x7f557ca2602e - core::ops::function::FnOnce::call_once::h87ca621b9a794c88
  18:     0x7f557b48e68c - rustc_query_system::query::plumbing::get_query_impl::h421d05be5482445a
  19:     0x7f557b5e7df2 - rustc_query_system::query::plumbing::get_query::h41f2adb40f023188
  20:     0x7f557b6c39eb - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::hd9be40711eef7b7d
  21:     0x7f557b6c38f7 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::h0a867475d355c1f8
  22:     0x7f557b74c7ef - rustc_query_impl::make_query::hir_owner::h5300dac4721f593c
  23:     0x7f557b5adae0 - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h0b6139b43215bc80
  24:     0x7f557b6e927b - rustc_query_impl::Queries::try_collect_active_jobs::hac47c7995f46a256
  25:     0x7f557b91e141 - rustc_query_system::query::job::print_query_stack::h954545ed20ae7cd1
  26:     0x7f557aaaeba4 - rustc_interface::interface::try_print_query_stack::h7a6dc429c05046d6
  27:     0x7f557a98efa9 - rustc_driver::report_ice::h480d29c9ccbaa079
  28:     0x7f5579e88fe8 - std::panicking::rust_panic_with_hook::hf465f5595671703e
  29:     0x7f5579e88ac7 - std::panicking::begin_panic_handler::{{closure}}::h680bdb0473565baa
  30:     0x7f5579e8486c - std::sys_common::backtrace::__rust_end_short_backtrace::h7b8b3b56e09356cf
  31:     0x7f5579e88a59 - rust_begin_unwind
  32:     0x7f5579e4ef51 - core::panicking::panic_fmt::h137d40a743197ca3
  33:     0x7f5579e4ee9d - core::panicking::panic::hbccfdb56d7fae999
  34:     0x7f557cb7e5f2 - rustc_middle::hir::map::collector::NodeCollector::insert_entry::ha4495ea58e201163
  35:     0x7f557cb7e774 - rustc_middle::hir::map::collector::NodeCollector::insert_with_hash::h3665666183c00715
  36:     0x7f557cb7ffb1 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_trait_ref::h155c1db2a4114102
  37:     0x7f557cc2fccf - rustc_hir::intravisit::walk_where_predicate::h1599d86eafb531e8
  38:     0x7f557cc2e4cc - rustc_hir::intravisit::walk_impl_item::h4407cb7ca9639464
  39:     0x7f557cb7fca4 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_impl_item::h45c5caee0947791c
  40:     0x7f557cc32ee4 - rustc_hir::intravisit::walk_item::h96835ed551620a92
  41:     0x7f557cb7f5a7 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::hb0e2c89bb1c29ec5
  42:     0x7f557cb7ecd0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h2812c3c9d4d20d1f
  43:     0x7f557cc32f6a - rustc_hir::intravisit::walk_item::h96835ed551620a92
  44:     0x7f557cb7f5a7 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::hb0e2c89bb1c29ec5
  45:     0x7f557cb7ecd0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h2812c3c9d4d20d1f
  46:     0x7f557cc32f6a - rustc_hir::intravisit::walk_item::h96835ed551620a92
  47:     0x7f557cb7f5a7 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::hb0e2c89bb1c29ec5
  48:     0x7f557cb7ecd0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h2812c3c9d4d20d1f
  49:     0x7f557cc3a72b - rustc_middle::hir::map::index_hir::h0936bd13ff9328f5
  50:     0x7f557b4e6c81 - rustc_query_system::query::plumbing::get_query_impl::hef39a5b44c795787
  51:     0x7f557b5e67f6 - rustc_query_system::query::plumbing::get_query::h3cae5810226e9452
  52:     0x7f557ca25aa7 - core::ops::function::FnOnce::call_once::h1d40a87bda2fd73b
  53:     0x7f557b4cfb18 - rustc_query_system::query::plumbing::get_query_impl::hbb51039add9caca5
  54:     0x7f557b5db86d - rustc_query_system::query::plumbing::get_query::h03a5f2481f3cb4c1
  55:     0x7f557cc3621e - rustc_middle::hir::map::Map::find::h67f327cad512a5a5
  56:     0x7f557cc366ec - <rustc_middle::hir::map::Map as rustc_hir::intravisit::Map>::item::h5afed420d6d2ea86
  57:     0x7f557b9d82e8 - rustc_middle::hir::map::Map::visit_item_likes_in_module::hd4a5760ef1bfc267
  58:     0x7f557ba17b8b - rustc_passes::hir_id_validator::check_crate::h6734f36909519697
  59:     0x7f557aac453f - rustc_interface::passes::analysis::h119a2cd252fcc489
  60:     0x7f557b46d10b - rustc_query_system::query::plumbing::get_query_impl::h069e86361b42446b
  61:     0x7f557b5db559 - rustc_query_system::query::plumbing::get_query::h03254aff20c21ba9
  62:     0x7f557a97e78a - rustc_interface::passes::QueryContext::enter::hf509752f0a5c421e
  63:     0x7f557a9df221 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h2e9fd115c32f3128
  64:     0x7f557a9a7f86 - rustc_span::with_source_map::ha1ffa9a3c29d82f2
  65:     0x7f557a9e079d - rustc_interface::interface::create_compiler_and_run::hde67ddc8839dd45e
  66:     0x7f557a9a8c52 - rustc_span::with_session_globals::hb301b60f6de6a3aa
  67:     0x7f557aa026fd - std::sys_common::backtrace::__rust_begin_short_backtrace::h2a5a60634c9974d4
  68:     0x7f557aa02c56 - std::panicking::try::hf2a717cf757cf8b1
  69:     0x7f557a991523 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hb0298b4ff434849a
  70:     0x7f5579e95cb7 - std::sys::unix::thread::Thread::new::thread_start::hcdb3b57eba1017f5
  71:     0x7f5574b4f6db - start_thread
  72:     0x7f5579b1b71f - __clone
  73:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (f7b6ba9fc 2021-07-02) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 4 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2018 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C metadata=ec9eb50245b31834 -C extra-filename=-ec9eb50245b31834 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
error: build failed
command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace compiler-builtins-c" "--manifest-path" "/checkout/library/test/Cargo.toml" "--message-format" "json-render-diagnostics"
expected success, got: exit status: 101
failed to run: /checkout/obj/build/bootstrap/debug/bootstrap --stage 2 test --exclude src/tools/tidy
