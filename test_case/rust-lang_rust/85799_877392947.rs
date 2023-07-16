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

note: rustc 1.55.0-nightly (58dd5e8c5 2021-07-09) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
stack backtrace:
   0:     0x7f5a90fba4e0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h7a6a93b131f2f4a9
   1:     0x7f5a9102d2fc - core::fmt::write::h554baf3029e46fd4
   2:     0x7f5a90fab725 - std::io::Write::write_fmt::h6174ac9fdbb0def8
   3:     0x7f5a90fbe8f7 - std::panicking::default_hook::{{closure}}::h66774db567c52595
   4:     0x7f5a90fbe2ff - std::panicking::default_hook::h9818569ee0b6b08d
   5:     0x7f5a91ac23b1 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::hdcec51b99f0e3431
   6:     0x7f5a90fbf118 - std::panicking::rust_panic_with_hook::hf465f5595671703e
   7:     0x7f5a90fbec27 - std::panicking::begin_panic_handler::{{closure}}::h680bdb0473565baa
   8:     0x7f5a90fba99c - std::sys_common::backtrace::__rust_end_short_backtrace::h7b8b3b56e09356cf
   9:     0x7f5a90fbeb89 - rust_begin_unwind
  10:     0x7f5a90f84a21 - core::panicking::panic_fmt::h137d40a743197ca3
  11:     0x7f5a90f84c13 - core::result::unwrap_failed::h211c608cd1950a84
  12:     0x7f5a925f8178 - rustc_query_system::query::plumbing::get_query_impl::hbb51039add9caca5
  13:     0x7f5a92700ddd - rustc_query_system::query::plumbing::get_query::h03a5f2481f3cb4c1
  14:     0x7f5a93d3f2fe - rustc_middle::hir::map::Map::find::h67f327cad512a5a5
  15:     0x7f5a93d42428 - rustc_middle::hir::map::Map::opt_span::h89781a143268facb
  16:     0x7f5a93d32cb5 - core::ops::function::FnOnce::call_once::h87ca621b9a794c88
  17:     0x7f5a925b67fd - rustc_query_system::query::plumbing::get_query_impl::h421d05be5482445a
  18:     0x7f5a9270d022 - rustc_query_system::query::plumbing::get_query::h41f2adb40f023188
  19:     0x7f5a927eedab - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::hd9be40711eef7b7d
  20:     0x7f5a927eecb7 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::h0a867475d355c1f8
  21:     0x7f5a927f067f - rustc_query_impl::make_query::hir_owner::h5300dac4721f593c
  22:     0x7f5a926d765a - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h8ebcad1698ffab7b
  23:     0x7f5a9288616b - rustc_query_impl::Queries::try_collect_active_jobs::hf07593f2f8c6e7e5
  24:     0x7f5a92a4082c - rustc_query_system::query::job::print_query_stack::h954545ed20ae7cd1
  25:     0x7f5a91c85774 - rustc_interface::interface::try_print_query_stack::h7a6dc429c05046d6
  26:     0x7f5a91ac2c49 - rustc_driver::report_ice::h480d29c9ccbaa079
  27:     0x7f5a90fbf118 - std::panicking::rust_panic_with_hook::hf465f5595671703e
  28:     0x7f5a90fbebf7 - std::panicking::begin_panic_handler::{{closure}}::h680bdb0473565baa
  29:     0x7f5a90fba99c - std::sys_common::backtrace::__rust_end_short_backtrace::h7b8b3b56e09356cf
  30:     0x7f5a90fbeb89 - rust_begin_unwind
  31:     0x7f5a90f84a21 - core::panicking::panic_fmt::h137d40a743197ca3
  32:     0x7f5a90f8496d - core::panicking::panic::hbccfdb56d7fae999
  33:     0x7f5a93c8c0d2 - rustc_middle::hir::map::collector::NodeCollector::insert_entry::ha4495ea58e201163
  34:     0x7f5a93c8c254 - rustc_middle::hir::map::collector::NodeCollector::insert_with_hash::h3665666183c00715
  35:     0x7f5a93c8da91 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_trait_ref::h155c1db2a4114102
  36:     0x7f5a93d380af - rustc_hir::intravisit::walk_where_predicate::h1599d86eafb531e8
  37:     0x7f5a93d368ac - rustc_hir::intravisit::walk_impl_item::h4407cb7ca9639464
  38:     0x7f5a93c8d784 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_impl_item::h45c5caee0947791c
  39:     0x7f5a93d3b2c4 - rustc_hir::intravisit::walk_item::h96835ed551620a92
  40:     0x7f5a93c8d087 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::hb0e2c89bb1c29ec5
  41:     0x7f5a93c8c7b0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h2812c3c9d4d20d1f
  42:     0x7f5a93d3b34a - rustc_hir::intravisit::walk_item::h96835ed551620a92
  43:     0x7f5a93c8d087 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::hb0e2c89bb1c29ec5
  44:     0x7f5a93c8c7b0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h2812c3c9d4d20d1f
  45:     0x7f5a93d3b34a - rustc_hir::intravisit::walk_item::h96835ed551620a92
  46:     0x7f5a93c8d087 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_item::hb0e2c89bb1c29ec5
  47:     0x7f5a93c8c7b0 - <rustc_middle::hir::map::collector::NodeCollector as rustc_hir::intravisit::Visitor>::visit_nested_item::h2812c3c9d4d20d1f
  48:     0x7f5a93d4378b - rustc_middle::hir::map::index_hir::h0936bd13ff9328f5
  49:     0x7f5a9260daa2 - rustc_query_system::query::plumbing::get_query_impl::hef39a5b44c795787
  50:     0x7f5a9270ba26 - rustc_query_system::query::plumbing::get_query::h3cae5810226e9452
  51:     0x7f5a93d3272a - core::ops::function::FnOnce::call_once::h1d40a87bda2fd73b
  52:     0x7f5a925f7e22 - rustc_query_system::query::plumbing::get_query_impl::hbb51039add9caca5
  53:     0x7f5a92700ddd - rustc_query_system::query::plumbing::get_query::h03a5f2481f3cb4c1
  54:     0x7f5a93d3f2fe - rustc_middle::hir::map::Map::find::h67f327cad512a5a5
  55:     0x7f5a93d3f7cc - <rustc_middle::hir::map::Map as rustc_hir::intravisit::Map>::item::h5afed420d6d2ea86
  56:     0x7f5a92af1b68 - rustc_middle::hir::map::Map::visit_item_likes_in_module::hd4a5760ef1bfc267
  57:     0x7f5a92b386cb - rustc_passes::hir_id_validator::check_crate::h6734f36909519697
  58:     0x7f5a91bfaa43 - rustc_interface::passes::analysis::h119a2cd252fcc489
  59:     0x7f5a9259543c - rustc_query_system::query::plumbing::get_query_impl::h069e86361b42446b
  60:     0x7f5a92700ac9 - rustc_query_system::query::plumbing::get_query::h03254aff20c21ba9
  61:     0x7f5a91b3547e - rustc_interface::passes::QueryContext::enter::hb96070ffa7a04cbe
  62:     0x7f5a91b120d2 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h08e866a4863789fd
  63:     0x7f5a91ad54ba - rustc_span::with_source_map::ha1ffa9a3c29d82f2
  64:     0x7f5a91b135cd - rustc_interface::interface::create_compiler_and_run::hde67ddc8839dd45e
  65:     0x7f5a91adc1f3 - scoped_tls::ScopedKey<T>::set::h08b38bf239952620
  66:     0x7f5a91b35f8b - std::sys_common::backtrace::__rust_begin_short_backtrace::h2a5a60634c9974d4
  67:     0x7f5a91b138b6 - std::panicking::try::hf2a717cf757cf8b1
  68:     0x7f5a91ac4ae3 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hb0298b4ff434849a
  69:     0x7f5a90fcbf07 - std::sys::unix::thread::Thread::new::thread_start::hcdb3b57eba1017f5
  70:     0x7f5a8bc846db - start_thread
  71:     0x7f5a90c5071f - __clone
  72:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.55.0-nightly (58dd5e8c5 2021-07-09) running on x86_64-unknown-linux-gnu

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
Build completed unsuccessfully in 0:04:47
