
   Compiling playground v0.0.1 (/playground)
error: internal compiler error: src/librustc_typeck/collect.rs:1849: unexpected sort of node in fn_sig(): Item(Item { ident: FOO#0, hir_id: HirId { owner: DefIndex(12), local_id: 0 }, attrs: [Attribute { id: AttrId(0), style: Outer, path: path(target_feature), tokens: TokenStream(Some([(Delimited(DelimSpan { open: src/lib.rs:1:17: 1:18, close: src/lib.rs:1:22: 1:23 }, Paren, TokenStream(Some([(Token(Token { kind: Ident(avx2, false), span: src/lib.rs:1:18: 1:22 }), NonJoint)]))), NonJoint)])), is_sugared_doc: false, span: src/lib.rs:1:1: 1:24 }], node: Const(type(usize), BodyId { hir_id: HirId { owner: DefIndex(12), local_id: 3 } }), vis: Spanned { node: Inherited, span: src/lib.rs:2:1: 2:1 }, span: src/lib.rs:2:1: 2:22 })

thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:812:9
stack backtrace:
   0: backtrace::backtrace::libunwind::trace
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1: backtrace::backtrace::trace_unsynchronized
             at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2: std::sys_common::backtrace::_print_fmt
             at src/libstd/sys_common/backtrace.rs:76
   3: <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt
             at src/libstd/sys_common/backtrace.rs:60
   4: core::fmt::write
             at src/libcore/fmt/mod.rs:1030
   5: std::io::Write::write_fmt
             at src/libstd/io/mod.rs:1412
   6: std::sys_common::backtrace::_print
             at src/libstd/sys_common/backtrace.rs:64
   7: std::sys_common::backtrace::print
             at src/libstd/sys_common/backtrace.rs:49
   8: std::panicking::default_hook::{{closure}}
             at src/libstd/panicking.rs:196
   9: std::panicking::default_hook
             at src/libstd/panicking.rs:210
  10: rustc_driver::report_ice
  11: std::panicking::rust_panic_with_hook
             at src/libstd/panicking.rs:477
  12: std::panicking::begin_panic
  13: rustc_errors::HandlerInner::bug
  14: rustc_errors::Handler::bug
  15: rustc::util::bug::opt_span_bug_fmt::{{closure}}
  16: rustc::ty::context::tls::with_opt::{{closure}}
  17: rustc::ty::context::tls::with_context_opt
  18: rustc::ty::context::tls::with_opt
  19: rustc::util::bug::opt_span_bug_fmt
  20: rustc::util::bug::bug_fmt
  21: rustc_typeck::collect::fn_sig
  22: rustc::ty::query::__query_compute::fn_sig
  23: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::fn_sig>::compute
  24: rustc::dep_graph::graph::DepGraph::with_task_impl
  25: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  26: rustc_typeck::collect::codegen_fn_attrs
  27: rustc::ty::query::__query_compute::codegen_fn_attrs
  28: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::codegen_fn_attrs>::compute
  29: rustc::dep_graph::graph::DepGraph::with_task_impl
  30: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  31: <rustc::hir::check_attr::CheckAttrVisitor as rustc::hir::intravisit::Visitor>::visit_item
  32: rustc::hir::map::Map::visit_item_likes_in_module
  33: rustc::hir::check_attr::check_mod_attrs
  34: rustc::ty::query::__query_compute::check_mod_attrs
  35: rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_mod_attrs>::compute
  36: rustc::dep_graph::graph::DepGraph::with_task_impl
  37: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  38: <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
  39: __rust_maybe_catch_panic
             at src/libpanic_unwind/lib.rs:80
  40: rustc_interface::passes::analysis::{{closure}}
  41: rustc::util::common::time
  42: rustc_interface::passes::analysis
  43: rustc::ty::query::__query_compute::analysis
  44: rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query
  45: rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}
  46: rustc_interface::passes::create_global_ctxt::{{closure}}
  47: rustc_interface::interface::run_compiler_in_existing_thread_pool
  48: std::thread::local::LocalKey<T>::with
  49: scoped_tls::ScopedKey<T>::set
  50: syntax::with_globals
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (6ef275e6c 2019-09-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread 'rustc' panicked at 'already borrowed: BorrowMutError', src/libcore/result.rs:1165:5
stack backtrace:
   0:     0x7fa6294cdae4 - backtrace::backtrace::libunwind::trace::hc496b60554206774
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1:     0x7fa6294cdae4 - backtrace::backtrace::trace_unsynchronized::ha20a82d0af0c65c5
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2:     0x7fa6294cdae4 - std::sys_common::backtrace::_print_fmt::h41aff515dcee7039
                               at src/libstd/sys_common/backtrace.rs:76
   3:     0x7fa6294cdae4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h9cece4375f2f7fda
                               at src/libstd/sys_common/backtrace.rs:60
   4:     0x7fa62950612c - core::fmt::write::hecb5092b1fa0aafc
                               at src/libcore/fmt/mod.rs:1030
   5:     0x7fa6294c1d07 - std::io::Write::write_fmt::h3a49fae144e74a2a
                               at src/libstd/io/mod.rs:1412
   6:     0x7fa6294d2315 - std::sys_common::backtrace::_print::h24134367c784cf37
                               at src/libstd/sys_common/backtrace.rs:64
   7:     0x7fa6294d2315 - std::sys_common::backtrace::print::ha95f6cb55d52824a
                               at src/libstd/sys_common/backtrace.rs:49
   8:     0x7fa6294d2315 - std::panicking::default_hook::{{closure}}::hf48b3987cc8265dd
                               at src/libstd/panicking.rs:196
   9:     0x7fa6294d2006 - std::panicking::default_hook::h6d285b42ab8557a8
                               at src/libstd/panicking.rs:210
  10:     0x7fa629a07f83 - rustc_driver::report_ice::h45cf923dd58ab6c9
  11:     0x7fa6294d2afc - std::panicking::rust_panic_with_hook::h694c05b962beffa6
                               at src/libstd/panicking.rs:477
  12:     0x7fa6294d25b2 - std::panicking::continue_panic_fmt::h6d9edb7f6134fa29
                               at src/libstd/panicking.rs:380
  13:     0x7fa6294d24a6 - rust_begin_unwind
                               at src/libstd/panicking.rs:307
  14:     0x7fa6294ffaba - core::panicking::panic_fmt::h06aa024bca353ba1
                               at src/libcore/panicking.rs:85
  15:     0x7fa6294ffcf7 - core::result::unwrap_failed::hef0e5f380db2d8ff
                               at src/libcore/result.rs:1165
  16:     0x7fa62b7dce05 - rustc_errors::Handler::force_print_diagnostic::h7bc5ce1adc02a5e1
  17:     0x7fa62b0bb061 - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::try_print_query_stack::h1f63bbe074ec7299
  18:     0x7fa629a08b95 - rustc_driver::report_ice::h45cf923dd58ab6c9
  19:     0x7fa6294d2afc - std::panicking::rust_panic_with_hook::h694c05b962beffa6
                               at src/libstd/panicking.rs:477
  20:     0x7fa62b7c41fd - std::panicking::begin_panic::h46d3b006bb005b25
  21:     0x7fa62b7dd563 - rustc_errors::HandlerInner::bug::hcc4f7785c0bbe624
  22:     0x7fa62b7dc44a - rustc_errors::Handler::bug::hab74f2c21f11224d
  23:     0x7fa62b0796b3 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h6fc69d466ad28204
  24:     0x7fa62b079183 - rustc::ty::context::tls::with_opt::{{closure}}::hfca6eae075a58800
  25:     0x7fa62b078e53 - rustc::ty::context::tls::with_context_opt::hb524b2b02be1c26b
  26:     0x7fa62b078e97 - rustc::ty::context::tls::with_opt::hd739a53bdacc1686
  27:     0x7fa62b0795c8 - rustc::util::bug::opt_span_bug_fmt::h7313400faf0ca111
  28:     0x7fa62b079532 - rustc::util::bug::bug_fmt::hd38ca659e7f5fbf2
  29:     0x7fa62a13c873 - rustc_typeck::collect::fn_sig::h20f58179ec4318a1
  30:     0x7fa629fd6902 - rustc::ty::query::__query_compute::fn_sig::hbeccaca34ae31f91
  31:     0x7fa62a06b78c - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::fn_sig>::compute::h97584cdb14aac61d
  32:     0x7fa629ffa165 - rustc::dep_graph::graph::DepGraph::with_task_impl::h508085a3626cf040
  33:     0x7fa62a0c60da - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::haf684df70016de02
  34:     0x7fa62a13fc1b - rustc_typeck::collect::codegen_fn_attrs::hdc27001063537d38
  35:     0x7fa62ae94d8b - rustc::ty::query::__query_compute::codegen_fn_attrs::hf3c6682ce4bcd833
  36:     0x7fa62b28a91c - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::codegen_fn_attrs>::compute::h90f0bde18c5eb158
  37:     0x7fa62ade7eb5 - rustc::dep_graph::graph::DepGraph::with_task_impl::h7081619feaaa0626
  38:     0x7fa62b1258dd - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h64b51212fc7051d3
  39:     0x7fa62acd6fb2 - <rustc::hir::check_attr::CheckAttrVisitor as rustc::hir::intravisit::Visitor>::visit_item::ha3c8cc6848202482
  40:     0x7fa62b003563 - rustc::hir::map::Map::visit_item_likes_in_module::h59d6eaaffb83d80a
  41:     0x7fa62acd8194 - rustc::hir::check_attr::check_mod_attrs::h0282581197e2fea2
  42:     0x7fa629b6bffa - rustc::ty::query::__query_compute::check_mod_attrs::hc487391ab24d41d2
  43:     0x7fa629b3aadb - rustc::ty::query::<impl rustc::ty::query::config::QueryAccessors for rustc::ty::query::queries::check_mod_attrs>::compute::h1ed5d5b18ac3eb52
  44:     0x7fa629a742dd - rustc::dep_graph::graph::DepGraph::with_task_impl::hdd2fec7ce1b36e5f
  45:     0x7fa629b5727a - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::hb604f702c9e8257e
  46:     0x7fa629a799af - <std::panic::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once::h92bdfbffd08a62d7
  47:     0x7fa6294e324a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  48:     0x7fa629afc3cf - rustc_interface::passes::analysis::{{closure}}::h4b79cf3bbdffdb9a
  49:     0x7fa629af707e - rustc::util::common::time::hcf0b99042a7de972
  50:     0x7fa629ab749f - rustc_interface::passes::analysis::h8a47af605888fc80
  51:     0x7fa6299a0851 - rustc::ty::query::__query_compute::analysis::hb9ae680a51f13bb9
  52:     0x7fa6299b017e - rustc::ty::query::plumbing::<impl rustc::ty::context::TyCtxt>::get_query::h751f86f55fa3f327
  53:     0x7fa6299d340a - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::h24275668f616227f
  54:     0x7fa629b33aaa - rustc_interface::passes::create_global_ctxt::{{closure}}::h279ea41337b1550b
  55:     0x7fa6299d56be - rustc_interface::interface::run_compiler_in_existing_thread_pool::h8f7b1ed446f84b95
  56:     0x7fa629a09ce2 - std::thread::local::LocalKey<T>::with::h307a7a354e416993
  57:     0x7fa629a0f8ae - scoped_tls::ScopedKey<T>::set::h2f6a18a3def76ab5
  58:     0x7fa629a300a2 - syntax::with_globals::h1fcbc798c727ae20
  59:     0x7fa6299c5310 - std::sys_common::backtrace::__rust_begin_short_backtrace::h1a2ca6bd7bcfc1d4
  60:     0x7fa6294e324a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  61:     0x7fa6299c6939 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h560e3eeff12b2618
  62:     0x7fa6294b41ef - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::hbf2f21e18d2cee36
                               at /rustc/6ef275e6c3cb1384ec78128eceeb4963ff788dca/src/liballoc/boxed.rs:922
  63:     0x7fa6294e1ef0 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h680214b55e06abc5
                               at /rustc/6ef275e6c3cb1384ec78128eceeb4963ff788dca/src/liballoc/boxed.rs:922
  64:     0x7fa6294e1ef0 - std::sys_common::thread::start_thread::ha3ffac07979d9757
                               at src/libstd/sys_common/thread.rs:13
  65:     0x7fa6294e1ef0 - std::sys::unix::thread::Thread::new::thread_start::hbce32216c8c0810e
                               at src/libstd/sys/unix/thread.rs:79
  66:     0x7fa62924e6db - start_thread
  67:     0x7fa628b6b88f - __clone
  68:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.39.0-nightly (6ef275e6c 2019-09-24) running on x86_64-unknown-linux-gnu

note: compiler flags: -C codegen-units=1 -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
thread panicked while processing panic. aborting.
error: could not compile `playground`.

Caused by:
  process didn't exit successfully: `rustc --edition=2018 --crate-name playground src/lib.rs --color never --crate-type lib --emit=dep-info,metadata,link -C codegen-units=1 -C debuginfo=2 -C metadata=df85738dfb3d9f6f -C extra-filename=-df85738dfb3d9f6f --out-dir /playground/target/debug/deps -L dependency=/playground/target/debug/deps --extern adler32=/playground/target/debug/deps/libadler32-f62f9ebd9ae5871d.rmeta --extern aho_corasick=/playground/target/debug/deps/libaho_corasick-ce1f07b6f738e017.rmeta --extern alga=/playground/target/debug/deps/libalga-9d8d020c174aba39.rmeta --extern ansi_term=/playground/target/debug/deps/libansi_term-b82117d597527608.rmeta --extern antidote=/playground/target/debug/deps/libantidote-194e39acba22c9b2.rmeta --extern approx=/playground/target/debug/deps/libapprox-a6b23b6e8f59e47c.rmeta --extern arc_swap=/playground/target/debug/deps/libarc_swap-20e302ae1af82b6a.rmeta --extern arrayref=/playground/target/debug/deps/libarrayref-b3eeca09e6cbd1a8.rmeta --extern arrayvec=/playground/target/debug/deps/libarrayvec-3870dfdd26fa6b9e.rmeta --extern atty=/playground/target/debug/deps/libatty-fdcb65457b32dbe5.rmeta --extern autocfg=/playground/target/debug/deps/libautocfg-6fcbd81cbaa71b42.rmeta --extern backtrace=/playground/target/debug/deps/libbacktrace-e6495e8ce7ed612a.rmeta --extern backtrace_sys=/playground/target/debug/deps/libbacktrace_sys-bfac077106627f58.rmeta --extern base64=/playground/target/debug/deps/libbase64-9d21a25f04561e1a.rmeta --extern bit_set=/playground/target/debug/deps/libbit_set-890a480aef0450fe.rmeta --extern bit_vec=/playground/target/debug/deps/libbit_vec-4ce09cd9ba2441d8.rmeta --extern bitflags=/playground/target/debug/deps/libbitflags-26113fec562de937.rmeta --extern blake2b_simd=/playground/target/debug/deps/libblake2b_simd-c994af97616c932c.rmeta --extern block_buffer=/playground/target/debug/deps/libblock_buffer-eaadbeecde72af42.rmeta --extern bstr=/playground/target/debug/deps/libbstr-b7af945657bc1fc2.rmeta --extern bumpalo=/playground/target/debug/deps/libbumpalo-81875765e464d722.rmeta --extern byte_tools=/playground/target/debug/deps/libbyte_tools-a9f09e024ebd7a44.rmeta --extern byteorder=/playground/target/debug/deps/libbyteorder-4bb136fb2a00b279.rmeta --extern bytes=/playground/target/debug/deps/libbytes-28881e65cfb348a6.rmeta --extern c2_chacha=/playground/target/debug/deps/libc2_chacha-120963ad683f5f2c.rmeta --extern cc=/playground/target/debug/deps/libcc-c16a08ddb6e5593b.rmeta --extern cfg_if=/playground/target/debug/deps/libcfg_if-a4072b414de649be.rmeta --extern chrono=/playground/target/debug/deps/libchrono-54a6587fd5a4af86.rmeta --extern clap=/playground/target/debug/deps/libclap-12a1c0714b711739.rmeta --extern cloudabi=/playground/target/debug/deps/libcloudabi-fa705959987d3351.rmeta --extern color_quant=/playground/target/debug/deps/libcolor_quant-bf8190d74c38dcfc.rmeta --extern constant_time_eq=/playground/target/debug/deps/libconstant_time_eq-07e051792ac6169d.rmeta --extern cookie=/playground/target/debug/deps/libcookie-b180e12784ece2e8.rmeta --extern cookie_store=/playground/target/debug/deps/libcookie_store-a95ae93951d3d381.rmeta --extern crc32fast=/playground/target/debug/deps/libcrc32fast-97683e7313796dcd.rmeta --extern crossbeam=/playground/target/debug/deps/libcrossbeam-a6a40e0948562cb6.rmeta --extern crossbeam_channel=/playground/target/debug/deps/libcrossbeam_channel-86109ec1c3d40a44.rmeta --extern crossbeam_deque=/playground/target/debug/deps/libcrossbeam_deque-a1a44120d607d149.rmeta --extern crossbeam_epoch=/playground/target/debug/deps/libcrossbeam_epoch-78f19b8afb54d643.rmeta --extern crossbeam_queue=/playground/target/debug/deps/libcrossbeam_queue-629e397a636e237d.rmeta --extern crossbeam_utils=/playground/target/debug/deps/libcrossbeam_utils-30f44ba1813ba956.rmeta --extern crypto_mac=/playground/target/debug/deps/libcrypto_mac-f2050eba9673e78b.rmeta --extern csv=/playground/target/debug/deps/libcsv-129d969cf853a819.rmeta --extern csv_core=/playground/target/debug/deps/libcsv_core-97acc149fbc5b2bb.rmeta --extern data_encoding=/playground/target/debug/deps/libdata_encoding-589748cd50e1f80e.rmeta --extern deflate=/playground/target/debug/deps/libdeflate-cb9c9a00d6456304.rmeta --extern digest=/playground/target/debug/deps/libdigest-7074c23250a37fd9.rmeta --extern dirs=/playground/target/debug/deps/libdirs-ce386763ec99a361.rmeta --extern dirs_sys=/playground/target/debug/deps/libdirs_sys-54d5955cd805ae70.rmeta --extern dtoa=/playground/target/debug/deps/libdtoa-47439fce681d62b1.rmeta --extern either=/playground/target/debug/deps/libeither-6e37f0877c52f76a.rmeta --extern encoding_rs=/playground/target/debug/deps/libencoding_rs-90e91fe313e34e1d.rmeta --extern env_logger=/playground/target/debug/deps/libenv_logger-99839048fcc7c7af.rmeta --extern error_chain=/playground/target/debug/deps/liberror_chain-afed50e24c215709.rmeta --extern failure=/playground/target/debug/deps/libfailure-b1a16ce0e430cabb.rmeta --extern failure_derive=/playground/target/debug/deps/libfailure_derive-9065995e9bd47b02.so --extern fake_simd=/playground/target/debug/deps/libfake_simd-e1dc4a8c125419cb.rmeta --extern fallible_iterator=/playground/target/debug/deps/libfallible_iterator-261fbc21fbe857a3.rmeta --extern fallible_streaming_iterator=/playground/target/debug/deps/libfallible_streaming_iterator-b0ecc0cd14d827ee.rmeta --extern filetime=/playground/target/debug/deps/libfiletime-fdd47b8940aa189a.rmeta --extern flate2=/playground/target/debug/deps/libflate2-d46d87ed9bab3933.rmeta --extern fnv=/playground/target/debug/deps/libfnv-fae8ac239943de41.rmeta --extern foreign_types=/playground/target/debug/deps/libforeign_types-73d0200ee868d574.rmeta --extern foreign_types_shared=/playground/target/debug/deps/libforeign_types_shared-a6f58c6234883248.rmeta --extern fuchsia_cprng=/playground/target/debug/deps/libfuchsia_cprng-1f3d3992f0e62d6d.rmeta --extern fuchsia_zircon=/playground/target/debug/deps/libfuchsia_zircon-b92bbe59c640b787.rmeta --extern fuchsia_zircon_sys=/playground/target/debug/deps/libfuchsia_zircon_sys-4a22b445dec91647.rmeta --extern futf=/playground/target/debug/deps/libfutf-6069d11ba0850c19.rmeta --extern futures=/playground/target/debug/deps/libfutures-d7b6a490cfdca625.rmeta --extern futures_cpupool=/playground/target/debug/deps/libfutures_cpupool-37067cb55afbdeaf.rmeta --extern gcc=/playground/target/debug/deps/libgcc-fbc782fa90aeb51b.rmeta --extern generic_array=/playground/target/debug/deps/libgeneric_array-9b3786d2d98c03ac.rmeta --extern getrandom=/playground/target/debug/deps/libgetrandom-ca605ede9d2ecbf6.rmeta --extern gif=/playground/target/debug/deps/libgif-bba003a86c297835.rmeta --extern glob=/playground/target/debug/deps/libglob-d3ca44cdd7314040.rmeta --extern h2=/playground/target/debug/deps/libh2-cddfd8537a4869f7.rmeta --extern heck=/playground/target/debug/deps/libheck-cce649ed13aee3d0.rmeta --extern hex=/playground/target/debug/deps/libhex-5fd6f1983e201b01.rmeta --extern hmac=/playground/target/debug/deps/libhmac-1d60e3413de0ca88.rmeta --extern html5ever=/playground/target/debug/deps/libhtml5ever-9ebac8a7efaff0f3.rmeta --extern http=/playground/target/debug/deps/libhttp-af65af09bb4c1af4.rmeta --extern http_body=/playground/target/debug/deps/libhttp_body-e7601ab785a90b78.rmeta --extern httparse=/playground/target/debug/deps/libhttparse-7a91ea00e2b843f9.rmeta --extern humantime=/playground/target/debug/deps/libhumantime-4c3225842e9f57ca.rmeta --extern hyper=/playground/target/debug/deps/libhyper-c58b9445546907b7.rmeta --extern hyper_tls=/playground/target/debug/deps/libhyper_tls-d022f63947153c82.rmeta --extern idna=/playground/target/debug/deps/libidna-4d1c624fa71cbc65.rmeta --extern image=/playground/target/debug/deps/libimage-cda557fefd536813.rmeta --extern indexmap=/playground/target/debug/deps/libindexmap-2fadb5481324b9e9.rmeta --extern inflate=/playground/target/debug/deps/libinflate-3c1ee76b6a0909d7.rmeta --extern iovec=/playground/target/debug/deps/libiovec-8740188c1a8d6685.rmeta --extern itertools=/playground/target/debug/deps/libitertools-7e344a8a5e5ef223.rmeta --extern itoa=/playground/target/debug/deps/libitoa-7e61d02fea6bcca9.rmeta --extern jpeg_decoder=/playground/target/debug/deps/libjpeg_decoder-ab83a2677642bd48.rmeta --extern js_sys=/playground/target/debug/deps/libjs_sys-97aa68eff3698285.rmeta --extern kernel32=/playground/target/debug/deps/libkernel32-bd371f0265dcc5e5.rmeta --extern lazy_static=/playground/target/debug/deps/liblazy_static-67eebaffc51fa801.rmeta --extern lazycell=/playground/target/debug/deps/liblazycell-6fc2ea950e9e06b4.rmeta --extern libc=/playground/target/debug/deps/liblibc-c5a9cc36e9295717.rmeta --extern libm=/playground/target/debug/deps/liblibm-9d2ed345a3e8a8de.rmeta --extern libsqlite3_sys=/playground/target/debug/deps/liblibsqlite3_sys-81e3c121bedaacb6.rmeta --extern linked_hash_map=/playground/target/debug/deps/liblinked_hash_map-4c6fb70216e8015c.rmeta --extern lock_api=/playground/target/debug/deps/liblock_api-a321cb695b1f9d52.rmeta --extern log=/playground/target/debug/deps/liblog-e7028c47835ce73f.rmeta --extern log_mdc=/playground/target/debug/deps/liblog_mdc-5683548a1b727341.rmeta --extern log4rs=/playground/target/debug/deps/liblog4rs-a18ab1e0d9ab5770.rmeta --extern lru_cache=/playground/target/debug/deps/liblru_cache-a18d7d09a7c4cd84.rmeta --extern lzw=/playground/target/debug/deps/liblzw-280777558838a0f2.rmeta --extern mac=/playground/target/debug/deps/libmac-5f37152f361db590.rmeta --extern markup5ever=/playground/target/debug/deps/libmarkup5ever-3aab5517f8ee4933.rmeta --extern matches=/playground/target/debug/deps/libmatches-b8dc9022c704d4f5.rmeta --extern matrixmultiply=/playground/target/debug/deps/libmatrixmultiply-bdbdfc90ff3d6dbd.rmeta --extern md5=/playground/target/debug/deps/libmd5-e6546afaccafcdb0.rmeta --extern memchr=/playground/target/debug/deps/libmemchr-af13453d88715011.rmeta --extern memmap=/playground/target/debug/deps/libmemmap-a05945e7542b8ae6.rmeta --extern memoffset=/playground/target/debug/deps/libmemoffset-0ce640bc25d582e8.rmeta --extern mime=/playground/target/debug/deps/libmime-2aca94c6968d594c.rmeta --extern mime_guess=/playground/target/debug/deps/libmime_guess-9dfd442f1d5f9164.rmeta --extern miniz_sys=/playground/target/debug/deps/libminiz_sys-4254fabc976aa8a4.rmeta --extern miniz_oxide=/playground/target/debug/deps/libminiz_oxide-3c1b5679e505bd81.rmeta --extern mio=/playground/target/debug/deps/libmio-26b4d3c4efb37257.rmeta --extern miow=/playground/target/debug/deps/libmiow-f0b5bb3f7331dc20.rmeta --extern nalgebra=/playground/target/debug/deps/libnalgebra-9db0e6909a716f4f.rmeta --extern native_tls=/playground/target/debug/deps/libnative_tls-08185821b9971642.rmeta --extern ndarray=/playground/target/debug/deps/libndarray-48144a6f103fd4b6.rmeta --extern net2=/playground/target/debug/deps/libnet2-b0423cc5b4b1a4bd.rmeta --extern debug_unreachable=/playground/target/debug/deps/libdebug_unreachable-a13e5d3357ca9b74.rmeta --extern nodrop=/playground/target/debug/deps/libnodrop-351f8388198774c3.rmeta --extern nom=/playground/target/debug/deps/libnom-7f91bd937e55937d.rmeta --extern num=/playground/target/debug/deps/libnum-2192245b507c5768.rmeta --extern num_bigint=/playground/target/debug/deps/libnum_bigint-6c5c2f11af92c829.rmeta --extern num_complex=/playground/target/debug/deps/libnum_complex-a6160c7282f42eb5.rmeta --extern num_derive=/playground/target/debug/deps/libnum_derive-428b1e470f1d04ac.so --extern num_integer=/playground/target/debug/deps/libnum_integer-0b9c5283533d6704.rmeta --extern num_iter=/playground/target/debug/deps/libnum_iter-799d51f234deaea5.rmeta --extern num_rational=/playground/target/debug/deps/libnum_rational-8d6d59aaa845c5a4.rmeta --extern num_traits=/playground/target/debug/deps/libnum_traits-aa1203d3f75300db.rmeta --extern num_cpus=/playground/target/debug/deps/libnum_cpus-e0815c8fc61444d7.rmeta --extern openssl=/playground/target/debug/deps/libopenssl-448a744fa611cc19.rmeta --extern openssl_probe=/playground/target/debug/deps/libopenssl_probe-3516df373a8934a6.rmeta --extern openssl_sys=/playground/target/debug/deps/libopenssl_sys-afea63355b96d109.rmeta --extern ordered_float=/playground/target/debug/deps/libordered_float-08a03512bdfb910f.rmeta --extern owning_ref=/playground/target/debug/deps/libowning_ref-fc616627fa501f45.rmeta --extern parking_lot=/playground/target/debug/deps/libparking_lot-65801e88b4aff713.rmeta --extern parking_lot_core=/playground/target/debug/deps/libparking_lot_core-373c09fd0237aa12.rmeta --extern percent_encoding=/playground/target/debug/deps/libpercent_encoding-0507709d561b51d1.rmeta --extern phf=/playground/target/debug/deps/libphf-cdb9470be04f3f3e.rmeta --extern phf_codegen=/playground/target/debug/deps/libphf_codegen-ccf81a8b7acc22db.rmeta --extern phf_generator=/playground/target/debug/deps/libphf_generator-13bfc6ebbc3117f3.rmeta --extern phf_shared=/playground/target/debug/deps/libphf_shared-954203cb46f5b7b5.rmeta --extern pkg_config=/playground/target/debug/deps/libpkg_config-7ca047d064a2b052.rmeta --extern png=/playground/target/debug/deps/libpng-e8831b1f0b1b6477.rmeta --extern postgres=/playground/target/debug/deps/libpostgres-ab678efd11732dee.rmeta --extern postgres_protocol=/playground/target/debug/deps/libpostgres_protocol-763699c16dd6e850.rmeta --extern postgres_shared=/playground/target/debug/deps/libpostgres_shared-333ae47591ec6d9c.rmeta --extern ppv_lite86=/playground/target/debug/deps/libppv_lite86-462b15320e98c728.rmeta --extern precomputed_hash=/playground/target/debug/deps/libprecomputed_hash-c912a42cc69295d4.rmeta --extern proc_macro2=/playground/target/debug/deps/libproc_macro2-246fc0784048a38f.rmeta --extern publicsuffix=/playground/target/debug/deps/libpublicsuffix-a0ad11a2e8ff982c.rmeta --extern quick_error=/playground/target/debug/deps/libquick_error-950ea3f441176722.rmeta --extern quote=/playground/target/debug/deps/libquote-e96f9d5816ce1906.rmeta --extern rand=/playground/target/debug/deps/librand-3c56afcc12e422d5.rmeta --extern rand_chacha=/playground/target/debug/deps/librand_chacha-e8ddefba70155a24.rmeta --extern rand_core=/playground/target/debug/deps/librand_core-a6da0899cefdc160.rmeta --extern rand_hc=/playground/target/debug/deps/librand_hc-c76b09da1ece63cc.rmeta --extern rand_isaac=/playground/target/debug/deps/librand_isaac-c34df315601e7475.rmeta --extern rand_jitter=/playground/target/debug/deps/librand_jitter-783bc1fdafe17763.rmeta --extern rand_os=/playground/target/debug/deps/librand_os-ed8d3b70922b7c2c.rmeta --extern rand_pcg=/playground/target/debug/deps/librand_pcg-27448d791a9b5c70.rmeta --extern rand_xorshift=/playground/target/debug/deps/librand_xorshift-b4ca89914c79b2d6.rmeta --extern rawpointer=/playground/target/debug/deps/librawpointer-6af5b0931a9a70a4.rmeta --extern rayon=/playground/target/debug/deps/librayon-5aa32b2a1a765836.rmeta --extern rayon_core=/playground/target/debug/deps/librayon_core-a790e2129f69a150.rmeta --extern rdrand=/playground/target/debug/deps/librdrand-17aa6f1894a9be8d.rmeta --extern regex=/playground/target/debug/deps/libregex-cd2b0d6770a422b9.rmeta --extern regex_automata=/playground/target/debug/deps/libregex_automata-d39a7c4b0192352c.rmeta --extern regex_syntax=/playground/target/debug/deps/libregex_syntax-2149a411256a4a59.rmeta --extern remove_dir_all=/playground/target/debug/deps/libremove_dir_all-fa20a0837a64c443.rmeta --extern reqwest=/playground/target/debug/deps/libreqwest-7dc25266336e3639.rmeta --extern ring=/playground/target/debug/deps/libring-857f304afc406614.rmeta --extern rusqlite=/playground/target/debug/deps/librusqlite-be2170d6ad9b2bfc.rmeta --extern argon2=/playground/target/debug/deps/libargon2-85f859d457cedb1e.rmeta --extern rustc_demangle=/playground/target/debug/deps/librustc_demangle-00b0bcc7560bbc33.rmeta --extern rustc_serialize=/playground/target/debug/deps/librustc_serialize-1b1eb1466cffaf72.rmeta --extern rustc_version=/playground/target/debug/deps/librustc_version-b71af371e4293074.rmeta --extern ryu=/playground/target/debug/deps/libryu-61fb413ee9f9fb6b.rmeta --extern safemem=/playground/target/debug/deps/libsafemem-9adf1044fa823e66.rmeta --extern same_file=/playground/target/debug/deps/libsame_file-61ed485fa9f35e17.rmeta --extern schannel=/playground/target/debug/deps/libschannel-7ac9474574cd700a.rmeta --extern scoped_threadpool=/playground/target/debug/deps/libscoped_threadpool-c7f2f8029b9f6e76.rmeta --extern scopeguard=/playground/target/debug/deps/libscopeguard-7369d63f98f8fb6d.rmeta --extern select=/playground/target/debug/deps/libselect-98871a7b3b7f643a.rmeta --extern semver=/playground/target/debug/deps/libsemver-4e03ccd10ea8ae59.rmeta --extern semver_parser=/playground/target/debug/deps/libsemver_parser-169e13d384c3a8a6.rmeta --extern serde=/playground/target/debug/deps/libserde-18ed9ff3a1e96e9a.rmeta --extern serde_value=/playground/target/debug/deps/libserde_value-4177d7545c1d73ed.rmeta --extern serde_derive=/playground/target/debug/deps/libserde_derive-4aaf3163a9ad0d4a.so --extern serde_json=/playground/target/debug/deps/libserde_json-1ece8ba51dad35af.rmeta --extern serde_urlencoded=/playground/target/debug/deps/libserde_urlencoded-55076bd12e54af81.rmeta --extern serde_yaml=/playground/target/debug/deps/libserde_yaml-6cea8bdbe38fbc55.rmeta --extern sha2=/playground/target/debug/deps/libsha2-22f77cd7f1323afd.rmeta --extern siphasher=/playground/target/debug/deps/libsiphasher-92e624454628e0c6.rmeta --extern slab=/playground/target/debug/deps/libslab-9ce65fbcd5dc18db.rmeta --extern smallvec=/playground/target/debug/deps/libsmallvec-1e2ae61dd7fc7771.rmeta --extern socket2=/playground/target/debug/deps/libsocket2-ef9986b31e507dcf.rmeta --extern sourcefile=/playground/target/debug/deps/libsourcefile-bec660447a7b4d2b.rmeta --extern spin=/playground/target/debug/deps/libspin-fd984f0ee1ce744b.rmeta --extern stable_deref_trait=/playground/target/debug/deps/libstable_deref_trait-f675ba384d204775.rmeta --extern string=/playground/target/debug/deps/libstring-b6953a68eba06d90.rmeta --extern string_cache=/playground/target/debug/deps/libstring_cache-c97f9fe705cdd488.rmeta --extern string_cache_codegen=/playground/target/debug/deps/libstring_cache_codegen-d26c343f33f1cd2f.rmeta --extern string_cache_shared=/playground/target/debug/deps/libstring_cache_shared-a4681a156fa5ecba.rmeta --extern stringprep=/playground/target/debug/deps/libstringprep-cbe795d9a3bae27c.rmeta --extern strsim=/playground/target/debug/deps/libstrsim-16f7247814674487.rmeta --extern syn=/playground/target/debug/deps/libsyn-bee8e607f0bb7edf.rmeta --extern synstructure=/playground/target/debug/deps/libsynstructure-e4c0943791b2e898.rmeta --extern syslog=/playground/target/debug/deps/libsyslog-c393311133693ba4.rmeta --extern tar=/playground/target/debug/deps/libtar-bc1d22f037848cf0.rmeta --extern tempdir=/playground/target/debug/deps/libtempdir-2493c77dd8c7a422.rmeta --extern tempfile=/playground/target/debug/deps/libtempfile-9160e77aa6434000.rmeta --extern tendril=/playground/target/debug/deps/libtendril-c1216a15165dd26e.rmeta --extern term=/playground/target/debug/deps/libterm-473783f35b2c7134.rmeta --extern termcolor=/playground/target/debug/deps/libtermcolor-10f38254db5fa95d.rmeta --extern textwrap=/playground/target/debug/deps/libtextwrap-2536ca361e28da3b.rmeta --extern thread_id=/playground/target/debug/deps/libthread_id-48897589691bd86e.rmeta --extern thread_local=/playground/target/debug/deps/libthread_local-699e6b332ae78376.rmeta --extern threadpool=/playground/target/debug/deps/libthreadpool-3376ac0975414cdd.rmeta --extern tiff=/playground/target/debug/deps/libtiff-2fc98c8c49c7c20a.rmeta --extern time=/playground/target/debug/deps/libtime-4a2dea5b837e534d.rmeta --extern tokio=/playground/target/debug/deps/libtokio-6e733dd573746d50.rmeta --extern tokio_buf=/playground/target/debug/deps/libtokio_buf-061abc8dfd65a2bd.rmeta --extern tokio_current_thread=/playground/target/debug/deps/libtokio_current_thread-8e7f194753372a8a.rmeta --extern tokio_executor=/playground/target/debug/deps/libtokio_executor-17338a47e2cefb8a.rmeta --extern tokio_io=/playground/target/debug/deps/libtokio_io-809d355d9afaccdd.rmeta --extern tokio_reactor=/playground/target/debug/deps/libtokio_reactor-9b01a1dcf4ed3f43.rmeta --extern tokio_sync=/playground/target/debug/deps/libtokio_sync-a30bb339eba38d77.rmeta --extern tokio_tcp=/playground/target/debug/deps/libtokio_tcp-b451d781ce17a2ae.rmeta --extern tokio_threadpool=/playground/target/debug/deps/libtokio_threadpool-3f49b9e47370e273.rmeta --extern tokio_timer=/playground/target/debug/deps/libtokio_timer-c8e1084b9a6cea51.rmeta --extern toml=/playground/target/debug/deps/libtoml-1217c4f101ae7b79.rmeta --extern traitobject=/playground/target/debug/deps/libtraitobject-524695d77ba5cd38.rmeta --extern try_lock=/playground/target/debug/deps/libtry_lock-5493f3af7a934147.rmeta --extern try_from=/playground/target/debug/deps/libtry_from-777c5b02e9ea9bc9.rmeta --extern typemap=/playground/target/debug/deps/libtypemap-a883877ce73380ce.rmeta --extern typenum=/playground/target/debug/deps/libtypenum-2d0d2cad35bdbf43.rmeta --extern ucd_util=/playground/target/debug/deps/libucd_util-00597bfb5fc421c2.rmeta --extern unicase=/playground/target/debug/deps/libunicase-18781b6f55ffc689.rmeta --extern unicode_bidi=/playground/target/debug/deps/libunicode_bidi-203aaa7fa40e5366.rmeta --extern unicode_normalization=/playground/target/debug/deps/libunicode_normalization-e4f11dada340352a.rmeta --extern unicode_segmentation=/playground/target/debug/deps/libunicode_segmentation-abcbee0ce978e97f.rmeta --extern unicode_width=/playground/target/debug/deps/libunicode_width-9e05deee5a3a5b5b.rmeta --extern unicode_xid=/playground/target/debug/deps/libunicode_xid-9fb5e14f514db5fa.rmeta --extern unreachable=/playground/target/debug/deps/libunreachable-a2a47c96e9ea5df3.rmeta --extern unsafe_any=/playground/target/debug/deps/libunsafe_any-0537e96f5893d9c1.rmeta --extern untrusted=/playground/target/debug/deps/libuntrusted-627cce7abd790386.rmeta --extern url=/playground/target/debug/deps/liburl-11a0570305f15b8c.rmeta --extern utf8=/playground/target/debug/deps/libutf8-c9fa1ad94c2faf8e.rmeta --extern utf8_ranges=/playground/target/debug/deps/libutf8_ranges-c44b11d82272fab0.rmeta --extern uuid=/playground/target/debug/deps/libuuid-7300f253ba43af1c.rmeta --extern vcpkg=/playground/target/debug/deps/libvcpkg-6765518329110ee5.rmeta --extern vec_map=/playground/target/debug/deps/libvec_map-543dc759865c8c55.rmeta --extern version_check=/playground/target/debug/deps/libversion_check-c7c56c0378ad2756.rmeta --extern void=/playground/target/debug/deps/libvoid-061784514c2fdc13.rmeta --extern walkdir=/playground/target/debug/deps/libwalkdir-7de8ea69363e0550.rmeta --extern want=/playground/target/debug/deps/libwant-876079a1958204e9.rmeta --extern wasi=/playground/target/debug/deps/libwasi-a90e5100d66739e6.rmeta --extern wasm_bindgen=/playground/target/debug/deps/libwasm_bindgen-22f02940009f7b34.rmeta --extern wasm_bindgen_backend=/playground/target/debug/deps/libwasm_bindgen_backend-5e1e71e299b3a911.rmeta --extern wasm_bindgen_macro=/playground/target/debug/deps/libwasm_bindgen_macro-1bc851af8446e620.so --extern wasm_bindgen_macro_support=/playground/target/debug/deps/libwasm_bindgen_macro_support-166e0de4c1c0867a.rmeta --extern wasm_bindgen_shared=/playground/target/debug/deps/libwasm_bindgen_shared-06a67972853feadf.rmeta --extern wasm_bindgen_webidl=/playground/target/debug/deps/libwasm_bindgen_webidl-22584140f3fc1fd9.rmeta --extern web_sys=/playground/target/debug/deps/libweb_sys-8a0635c6097f1ebb.rmeta --extern weedle=/playground/target/debug/deps/libweedle-78488b801d470799.rmeta --extern winapi=/playground/target/debug/deps/libwinapi-a6af4572e0bf8090.rmeta --extern build=/playground/target/debug/deps/libbuild-8b979d5ac18a1dfc.rmeta --extern winapi_i686_pc_windows_gnu=/playground/target/debug/deps/libwinapi_i686_pc_windows_gnu-e0e5e7cf3aeeb125.rmeta --extern winapi_util=/playground/target/debug/deps/libwinapi_util-b2e041768177f19d.rmeta --extern winapi_x86_64_pc_windows_gnu=/playground/target/debug/deps/libwinapi_x86_64_pc_windows_gnu-ee6d93dd8dcc5d8e.rmeta --extern wincolor=/playground/target/debug/deps/libwincolor-2309f3d6d77259f6.rmeta --extern ws2_32=/playground/target/debug/deps/libws2_32-ce428a49a4357f8c.rmeta --extern xattr=/playground/target/debug/deps/libxattr-e040e67e8e910b12.rmeta --extern yaml_rust=/playground/target/debug/deps/libyaml_rust-dd37c43ca8d88df7.rmeta -L native=/playground/target/debug/build/backtrace-sys-206b2aee2ba3fad4/out -L native=/playground/target/debug/build/miniz-sys-804adda12987cfe2/out -L native=/playground/target/debug/build/ring-cdfdc82c4a651ac0/out` (signal: 4, SIGILL: illegal instruction)

Warnings

No main function was detected, so your code was compiled
but not run. If you’d like to execute your code, please
add a main function.
