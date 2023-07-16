plain
   Compiling memchr v2.4.1
   Compiling std v0.0.0 (/checkout/library/std)
   Compiling compiler_builtins v0.1.49
   Compiling unwind v0.0.0 (/checkout/library/unwind)
thread 'rustc' panicked at 'No HirId for DefId(0:53 ~ core[54ad]::assert_matches::{misc#2})', compiler/rustc_middle/src/hir/map/mod.rs:217:32

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (9272f9f2e 2021-09-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'No HirId for DefId(0:53 ~ core[54ad]::assert_matches::{misc#2})', compiler/rustc_middle/src/hir/map/mod.rs:217:32
stack backtrace:
   0:     0x7f4308194d0c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha79b5e4378aa5cd1
   1:     0x7f43081f70ac - core::fmt::write::ha1a4a55045432609
   2:     0x7f4308185815 - std::io::Write::write_fmt::ha77b5b5476173c3d
   3:     0x7f430819874c - std::panicking::default_hook::{{closure}}::h237ac32d61b17a78
   4:     0x7f430819822e - std::panicking::default_hook::hc15ca670fb008621
   5:     0x7f4308cb5071 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h03cb86a62fa6d09a
   6:     0x7f4308198f58 - std::panicking::rust_panic_with_hook::hb2f60e2e73732381
   7:     0x7f4308198a60 - std::panicking::begin_panic_handler::{{closure}}::h4208554e88c83c09
   8:     0x7f43081951b4 - std::sys_common::backtrace::__rust_end_short_backtrace::hb521b50c6eaccfc2
   9:     0x7f43081989c9 - rust_begin_unwind
  10:     0x7f43081587cb - std::panicking::begin_panic_fmt::h4b117265f2d2045e
  11:     0x7f430b151868 - rustc_middle::hir::map::Map::local_def_id_to_hir_id::{{closure}}::hc67f2303173abfca
  12:     0x7f430b15179f - rustc_middle::hir::map::Map::local_def_id_to_hir_id::h8e02827cdc4812d6
  13:     0x7f430b156b1e - rustc_middle::hir::map::Map::span_if_local::h88c8413111868f2e
  14:     0x7f430b09caee - core::ops::function::FnOnce::call_once::h593ba59c3ee098de
  15:     0x7f430a02f4fc - rustc_query_system::query::plumbing::try_execute_query::h58b18ebad8efe164
  16:     0x7f430a145b78 - rustc_query_system::query::plumbing::get_query::h3f0e40948b430c8a
  17:     0x7f430a2de726 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h2a9cc6ea50896d74
  18:     0x7f430a28a54d - rustc_query_impl::make_query::codegen_fn_attrs::h0c1e54c5cadca441
  19:     0x7f430a0fe87e - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::hd874dc0df1914953
  20:     0x7f430a464bc4 - rustc_query_impl::Queries::try_collect_active_jobs::hd73d546b73461331
  21:     0x7f430a26f5b1 - rustc_query_system::query::job::print_query_stack::h5af151e616d68b96
  22:     0x7f4308dcb8af - rustc_interface::interface::try_print_query_stack::h87cb17eb2f30cb91
  23:     0x7f4308cb5909 - rustc_driver::report_ice::h0a9710fbb163d866
  24:     0x7f4308198f58 - std::panicking::rust_panic_with_hook::hb2f60e2e73732381
  25:     0x7f4308198a60 - std::panicking::begin_panic_handler::{{closure}}::h4208554e88c83c09
  26:     0x7f43081951b4 - std::sys_common::backtrace::__rust_end_short_backtrace::hb521b50c6eaccfc2
  27:     0x7f43081989c9 - rust_begin_unwind
  28:     0x7f43081587cb - std::panicking::begin_panic_fmt::h4b117265f2d2045e
  29:     0x7f430b027e38 - rustc_middle::hir::map::Map::local_def_id_to_hir_id::{{closure}}::hc67f2303173abfca
  30:     0x7f430b027d6f - rustc_middle::hir::map::Map::local_def_id_to_hir_id::h8e02827cdc4812d6
  31:     0x7f430b0600bc - rustc_middle::ty::<impl rustc_middle::ty::context::TyCtxt>::get_attrs::h26a3cebb2e5457d5
  32:     0x7f43094c886d - rustc_typeck::collect::codegen_fn_attrs::h5a991ac0e6c4486b
  33:     0x7f430a43aeb2 - rustc_data_structures::stack::ensure_sufficient_stack::hfe9c5ab26bf130dd
  34:     0x7f430a05f5d9 - rustc_query_system::query::plumbing::try_execute_query::hda95606c2a134404
  35:     0x7f430a151096 - rustc_query_system::query::plumbing::get_query::h4ff5433229d199b0
  36:     0x7f430989c1bb - rustc_passes::dead::has_allow_dead_code_or_lang_attr::hc08226ae15cc90eb
  37:     0x7f430989c2c7 - <rustc_passes::dead::LifeSeeder as rustc_hir::itemlikevisit::ItemLikeVisitor>::visit_item::hef68091795b313bc
  38:     0x7f43098d9923 - rustc_middle::hir::map::Map::visit_all_item_likes::h12eada62ef5e8adf
  39:     0x7f430989da9b - rustc_passes::dead::check_crate::h4985160017a4b0d8
  40:     0x7f4308e72bb0 - rustc_session::utils::<impl rustc_session::session::Session>::time::h31448ef460a8afc8
  41:     0x7f4308e32848 - std::panicking::try::h95a353436b5ccd4e
  42:     0x7f4308e47f4d - <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h6b6d2c481f006c52
  43:     0x7f4308e32a66 - std::panicking::try::hdd874dbd5613f81e
  44:     0x7f4308e72c6e - rustc_session::utils::<impl rustc_session::session::Session>::time::h36e8cafce3056e0f
  45:     0x7f4308df6600 - rustc_interface::passes::analysis::h53c5a62013e5372f
  46:     0x7f430a04857a - rustc_query_system::query::plumbing::try_execute_query::ha156c10c8e67d4a7
  47:     0x7f430a197e74 - rustc_query_system::query::plumbing::get_query::hfbf7beaed4d7da97
  48:     0x7f4308d23bb7 - rustc_interface::passes::QueryContext::enter::ha7aebef821df5ef2
  49:     0x7f4308cf86d8 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h642f8bde6186d1b9
  50:     0x7f4308cbc1a0 - rustc_span::with_source_map::h2683753e2ec53529
  51:     0x7f4308cfa30d - std::sys_common::backtrace::__rust_begin_short_backtrace::h3690c34e789b02bc
  52:     0x7f4308d24c86 - std::panicking::try::h72ebd651474b41b6
  53:     0x7f4308ca7843 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hdd82a498b77c0179
  54:     0x7f43081a6423 - std::sys::unix::thread::Thread::new::thread_start::h3b142aabf8e36292
  55:     0x7f4302e5c6db - start_thread
  56:     0x7f4307e2871f - __clone
  57:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (9272f9f2e 2021-09-18) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z crate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/") -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 -C embed-bitcode=yes --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread panicked while processing panic. aborting.
rustc exited with signal: 4 (core dumped)

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name core --edition=2018 library/core/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C codegen-units=1 -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C metadata=0286f51ede2a0cbe -C extra-filename=-0286f51ede2a0cbe --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-std/release/deps -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Cembed-bitcode=yes '-Zcrate-attr=doc(html_root_url="https://doc.rust-lang.org/nightly/")' -Z binary-dep-depinfo` (exit status: 254)
