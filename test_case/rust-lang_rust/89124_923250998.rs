plain
note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (adab8e63c 2021-09-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
stack backtrace:
   0:     0x7f6d022f3d0c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha79b5e4378aa5cd1
   1:     0x7f6d023560ac - core::fmt::write::ha1a4a55045432609
   2:     0x7f6d022e4815 - std::io::Write::write_fmt::ha77b5b5476173c3d
   3:     0x7f6d022f774c - std::panicking::default_hook::{{closure}}::h237ac32d61b17a78
   4:     0x7f6d022f722e - std::panicking::default_hook::hc15ca670fb008621
   Compiling opaque-debug v0.3.0
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
   Compiling rustc_graphviz v0.0.0 (/checkout/compiler/rustc_graphviz)
   5:     0x7f6d02e0f5b1 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h75fb797b45db1776
   6:     0x7f6d022f7f58 - std::panicking::rust_panic_with_hook::hb2f60e2e73732381
   7:     0x7f6d022f7a60 - std::panicking::begin_panic_handler::{{closure}}::h4208554e88c83c09
   8:     0x7f6d022f41b4 - std::sys_common::backtrace::__rust_end_short_backtrace::hb521b50c6eaccfc2
   9:     0x7f6d022f79c9 - rust_begin_unwind
  10:     0x7f6d022b8a81 - core::panicking::panic_fmt::hfbe17c5f03585ac9
  11:     0x7f6d022b8c33 - core::result::unwrap_failed::h752962d7f626a669
  12:     0x7f6d0418ebd1 - rustc_query_system::query::plumbing::try_execute_query::hc1a3e74b02ac7c3c
  13:     0x7f6d042685f4 - rustc_query_system::query::plumbing::get_query::h169eed2183cfd7f9
  14:     0x7f6d05234f8e - rustc_middle::hir::map::Map::find::hae8a3decc89ed46f
  15:     0x7f6d052395c5 - rustc_middle::hir::map::Map::opt_span::h7644d3b95c6c2898
  16:     0x7f6d0522c7e5 - core::ops::function::FnOnce::call_once::h0ebc3ef60304533c
  17:     0x7f6d041585fc - rustc_query_system::query::plumbing::try_execute_query::h2b2c51d7c9d33180
  18:     0x7f6d042bfe98 - rustc_query_system::query::plumbing::get_query::hdca99159f5eaf717
  19:     0x7f6d044432f6 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h16de62d2c2630ceb
  20:     0x7f6d04443117 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::hd24eaaf3619e37fe
  21:     0x7f6d04368a30 - rustc_query_impl::make_query::hir_owner::h0ef52c2ddc17fbaf
  22:     0x7f6d04232fca - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h35758ab4fb53d24c
  23:     0x7f6d0444ad1d - rustc_query_impl::Queries::try_collect_active_jobs::hcad663d2880ea3e9
  24:     0x7f6d043246d1 - rustc_query_system::query::job::print_query_stack::h3758d174de5ceac6
  25:     0x7f6d02fdf832 - rustc_interface::interface::try_print_query_stack::hb3277d3ec655aecb
  26:     0x7f6d02e0fe49 - rustc_driver::report_ice::hcd4fc540c8f17698
  27:     0x7f6d022f7f58 - std::panicking::rust_panic_with_hook::hb2f60e2e73732381
  28:     0x7f6d022f7a60 - std::panicking::begin_panic_handler::{{closure}}::h4208554e88c83c09
  29:     0x7f6d022f41b4 - std::sys_common::backtrace::__rust_end_short_backtrace::hb521b50c6eaccfc2
  30:     0x7f6d022f79c9 - rust_begin_unwind
  31:     0x7f6d022b8a81 - core::panicking::panic_fmt::hfbe17c5f03585ac9
  32:     0x7f6d022b8a42 - core::panicking::panic_bounds_check::h8a5383bf4f7d511b
  33:     0x7f6d0522ddea - core::ops::function::FnOnce::call_once::hfe34fa842dc5246d
  34:     0x7f6d0418e563 - rustc_query_system::query::plumbing::try_execute_query::hc1a3e74b02ac7c3c
  35:     0x7f6d042685f4 - rustc_query_system::query::plumbing::get_query::h169eed2183cfd7f9
  36:     0x7f6d05234f8e - rustc_middle::hir::map::Map::find::hae8a3decc89ed46f
  37:     0x7f6d0523436a - rustc_middle::hir::map::Map::opt_def_kind::h4289cdbdeb6bb909
  38:     0x7f6d0522d2fa - core::ops::function::FnOnce::call_once::h58eee2a5ac74208b
  39:     0x7f6d04183f4a - rustc_query_system::query::plumbing::try_execute_query::ha5c547753a037e3a
  40:     0x7f6d042a0266 - rustc_query_system::query::plumbing::get_query::ha046709acccc9f49
  41:     0x7f6d048d6e02 - rustc_metadata::rmeta::encoder::EncodeContext::encode_def_ids::h802795e6c1aff5f2
  42:     0x7f6d048d02b6 - rustc_metadata::rmeta::encoder::EncodeContext::encode_crate_root::h8a4884cf1c15b3f3
  43:     0x7f6d048e655d - rustc_metadata::rmeta::encoder::encode_metadata_impl::h36ee9259f3c51122
  44:     0x7f6d04943541 - rustc_data_structures::sync::join::h5776fb7fa6fd9dc1
  45:     0x7f6d048e5dfb - rustc_metadata::rmeta::encoder::encode_metadata::h6e11fcbf726743cf
  46:     0x7f6d0491243c - rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata::he993bb6adaa63ab7
  47:     0x7f6d051400f9 - rustc_middle::ty::context::TyCtxt::encode_metadata::hd43b1d377204080e
  48:     0x7f6d02f39a37 - rustc_interface::queries::Queries::ongoing_codegen::ha4e02a81b8f89348
  49:     0x7f6d02e52419 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hbeb50213beb70b36
  50:     0x7f6d02e166c0 - rustc_span::with_source_map::hd8dd5f54a8af4509
  51:     0x7f6d02e545ed - std::sys_common::backtrace::__rust_begin_short_backtrace::h7725144654d299c0
  52:     0x7f6d02e7e5a6 - std::panicking::try::h8f4a65864319a59a
  53:     0x7f6d02e01e43 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h0ecb08b50610b0a6
  54:     0x7f6d02305423 - std::sys::unix::thread::Thread::new::thread_start::h3b142aabf8e36292
  55:     0x7f6cfcfbb6db - start_thread
  56:     0x7f6d01f8771f - __clone
  57:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (adab8e63c 2021-09-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
---
   Compiling termcolor v1.1.2
   Compiling serde v1.0.125
   Compiling annotate-snippets v0.8.0
   Compiling rustc_fs_util v0.0.0 (/checkout/compiler/rustc_fs_util)
rustc exited with signal: 4 (core dumped)
error: could not compile `remove_dir_all`
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/bootstrap/debug/rustc --crate-name remove_dir_all /cargo/registry/src/github.com-1ecc6299db9ec823/remove_dir_all-0.5.3/src/lib.rs --error-format=json --json=diagnostic-rendered-ansi,artifacts --crate-type lib --emit=dep-info,metadata,link -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C metadata=8a88b87ad7d28ba7 -C extra-filename=-8a88b87ad7d28ba7 --out-dir /checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps --target x86_64-unknown-linux-gnu -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/x86_64-unknown-linux-gnu/release/deps -L dependency=/checkout/obj/build/x86_64-unknown-linux-gnu/stage1-rustc/release/deps --cap-lints allow -Zmacro-backtrace '-Clink-args=-Wl,-rpath,$ORIGIN/../lib' -Ztls-model=initial-exec -Zunstable-options '-Wrustc::internal' -Cprefer-dynamic -Cllvm-args=-import-instr-limit=10 -Z binary-dep-depinfo` (exit status: 254)
thread 'rustc' panicked at 'index out of bounds: the len is 33 but the index is 33', compiler/rustc_middle/src/hir/mod.rs:174:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

error: internal compiler error: unexpected panic
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (adab8e63c 2021-09-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', /checkout/compiler/rustc_data_structures/src/sync.rs:423:16
stack backtrace:
   0:     0x7fd24acc3d0c - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::ha79b5e4378aa5cd1
   1:     0x7fd24ad260ac - core::fmt::write::ha1a4a55045432609
   2:     0x7fd24acb4815 - std::io::Write::write_fmt::ha77b5b5476173c3d
   3:     0x7fd24acc774c - std::panicking::default_hook::{{closure}}::h237ac32d61b17a78
   4:     0x7fd24acc722e - std::panicking::default_hook::hc15ca670fb008621
   5:     0x7fd24b7df5b1 - rustc_driver::DEFAULT_HOOK::{{closure}}::{{closure}}::h75fb797b45db1776
   6:     0x7fd24acc7f58 - std::panicking::rust_panic_with_hook::hb2f60e2e73732381
   7:     0x7fd24acc7a60 - std::panicking::begin_panic_handler::{{closure}}::h4208554e88c83c09
   8:     0x7fd24acc41b4 - std::sys_common::backtrace::__rust_end_short_backtrace::hb521b50c6eaccfc2
   9:     0x7fd24acc79c9 - rust_begin_unwind
  10:     0x7fd24ac88a81 - core::panicking::panic_fmt::hfbe17c5f03585ac9
  11:     0x7fd24ac88c33 - core::result::unwrap_failed::h752962d7f626a669
  12:     0x7fd24cb5ebd1 - rustc_query_system::query::plumbing::try_execute_query::hc1a3e74b02ac7c3c
  13:     0x7fd24cc385f4 - rustc_query_system::query::plumbing::get_query::h169eed2183cfd7f9
  14:     0x7fd24dc04f8e - rustc_middle::hir::map::Map::find::hae8a3decc89ed46f
  15:     0x7fd24dc095c5 - rustc_middle::hir::map::Map::opt_span::h7644d3b95c6c2898
  16:     0x7fd24dbfc7e5 - core::ops::function::FnOnce::call_once::h0ebc3ef60304533c
  17:     0x7fd24cb285fc - rustc_query_system::query::plumbing::try_execute_query::h2b2c51d7c9d33180
  18:     0x7fd24cc8fe98 - rustc_query_system::query::plumbing::get_query::hdca99159f5eaf717
  19:     0x7fd24ce132f6 - <rustc_span::def_id::DefId as rustc_query_impl::keys::Key>::default_span::h16de62d2c2630ceb
  20:     0x7fd24ce13117 - <rustc_span::def_id::LocalDefId as rustc_query_impl::keys::Key>::default_span::hd24eaaf3619e37fe
  21:     0x7fd24cd38a30 - rustc_query_impl::make_query::hir_owner::h0ef52c2ddc17fbaf
  22:     0x7fd24cc02fca - rustc_query_system::query::plumbing::QueryState<D,K>::try_collect_active_jobs::h35758ab4fb53d24c
  23:     0x7fd24ce1ad1d - rustc_query_impl::Queries::try_collect_active_jobs::hcad663d2880ea3e9
  24:     0x7fd24ccf46d1 - rustc_query_system::query::job::print_query_stack::h3758d174de5ceac6
  25:     0x7fd24b9af832 - rustc_interface::interface::try_print_query_stack::hb3277d3ec655aecb
  26:     0x7fd24b7dfe49 - rustc_driver::report_ice::hcd4fc540c8f17698
  27:     0x7fd24acc7f58 - std::panicking::rust_panic_with_hook::hb2f60e2e73732381
  28:     0x7fd24acc7a60 - std::panicking::begin_panic_handler::{{closure}}::h4208554e88c83c09
  29:     0x7fd24acc41b4 - std::sys_common::backtrace::__rust_end_short_backtrace::hb521b50c6eaccfc2
  30:     0x7fd24acc79c9 - rust_begin_unwind
  31:     0x7fd24ac88a81 - core::panicking::panic_fmt::hfbe17c5f03585ac9
  32:     0x7fd24ac88a42 - core::panicking::panic_bounds_check::h8a5383bf4f7d511b
  33:     0x7fd24dbfddea - core::ops::function::FnOnce::call_once::hfe34fa842dc5246d
  34:     0x7fd24cb5e563 - rustc_query_system::query::plumbing::try_execute_query::hc1a3e74b02ac7c3c
  35:     0x7fd24cc385f4 - rustc_query_system::query::plumbing::get_query::h169eed2183cfd7f9
  36:     0x7fd24dc04f8e - rustc_middle::hir::map::Map::find::hae8a3decc89ed46f
  37:     0x7fd24dc0436a - rustc_middle::hir::map::Map::opt_def_kind::h4289cdbdeb6bb909
  38:     0x7fd24dbfd2fa - core::ops::function::FnOnce::call_once::h58eee2a5ac74208b
  39:     0x7fd24cb53f4a - rustc_query_system::query::plumbing::try_execute_query::ha5c547753a037e3a
  40:     0x7fd24cc70266 - rustc_query_system::query::plumbing::get_query::ha046709acccc9f49
  41:     0x7fd24d2a6e02 - rustc_metadata::rmeta::encoder::EncodeContext::encode_def_ids::h802795e6c1aff5f2
  42:     0x7fd24d2a02b6 - rustc_metadata::rmeta::encoder::EncodeContext::encode_crate_root::h8a4884cf1c15b3f3
  43:     0x7fd24d2b655d - rustc_metadata::rmeta::encoder::encode_metadata_impl::h36ee9259f3c51122
  44:     0x7fd24d313541 - rustc_data_structures::sync::join::h5776fb7fa6fd9dc1
  45:     0x7fd24d2b5dfb - rustc_metadata::rmeta::encoder::encode_metadata::h6e11fcbf726743cf
  46:     0x7fd24d2e243c - rustc_metadata::rmeta::decoder::cstore_impl::<impl rustc_middle::middle::cstore::CrateStore for rustc_metadata::creader::CStore>::encode_metadata::he993bb6adaa63ab7
  47:     0x7fd24db100f9 - rustc_middle::ty::context::TyCtxt::encode_metadata::hd43b1d377204080e
  48:     0x7fd24b909a37 - rustc_interface::queries::Queries::ongoing_codegen::ha4e02a81b8f89348
  49:     0x7fd24b822419 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::hbeb50213beb70b36
  50:     0x7fd24b7e66c0 - rustc_span::with_source_map::hd8dd5f54a8af4509
  51:     0x7fd24b8245ed - std::sys_common::backtrace::__rust_begin_short_backtrace::h7725144654d299c0
  52:     0x7fd24b84e5a6 - std::panicking::try::h8f4a65864319a59a
  53:     0x7fd24b7d1e43 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h0ecb08b50610b0a6
  54:     0x7fd24acd5423 - std::sys::unix::thread::Thread::new::thread_start::h3b142aabf8e36292
  55:     0x7fd24598b6db - start_thread
  56:     0x7fd24a95771f - __clone
  57:                0x0 - <unknown>
error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.


note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.57.0-nightly (adab8e63c 2021-09-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z macro-backtrace -Z tls-model=initial-exec -Z unstable-options -Z binary-dep-depinfo -Z force-unstable-if-unmarked -C opt-level=3 -C embed-bitcode=no -C debuginfo=0 -C debug-assertions=on -C overflow-checks=off -C link-args=-Wl,-rpath,$ORIGIN/../lib -C prefer-dynamic -C llvm-args=-import-instr-limit=10 --crate-type lib
note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
end of query stack
thread panicked while panicking. aborting.
rustc exited with signal: 4 (core dumped)
Build completed unsuccessfully in 0:05:00
