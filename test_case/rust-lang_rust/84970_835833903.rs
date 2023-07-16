
thread 'rustc' panicked at 'found unstable fingerprints for native_libraries(helix_syntax[382e]): [NativeLib { kind: StaticBundle, name: Some("tree-sitter-html-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-json-c"), cfg: None, foreign_modu
le: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-toml-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-css-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: Static
Bundle, name: Some("tree-sitter-javascript-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-scala-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-swift-c"), cfg: None,
foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-c-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-python-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib {
kind: StaticBundle, name: Some("tree-sitter-java-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-go-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-php-c"), cfg: None,
 foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-bash-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-python-cpp"), cfg: None, foreign_module: None, wasm_import_module: None }, Native
Lib { kind: StaticBundle, name: Some("tree-sitter-html-cpp"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-bash-cpp"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-php-cpp
"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-ruby-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-rust-c"), cfg: None, foreign_module: None, wasm_import_module: None
 }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-cpp-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-julia-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-r
uby-cpp"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-cpp-cpp"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-c-sharp-c"), cfg: None, foreign_module: None, wasm_import_m
odule: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-agda-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-agda-cpp"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: Unspecified, name: Some("s
tdc++"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-tsx-c"), cfg: None, foreign_module: None, wasm_import_module: None }, NativeLib { kind: StaticBundle, name: Some("tree-sitter-typescript-c"), cfg: None, foreign_module: None, wasm_import_mo
dule: None }]', /rustc/88f19c6dab716c6281af7602e30f413e809c5974/compiler/rustc_query_system/src/query/plumbing.rs:593:5
stack backtrace:
   0:     0x7f1e3a1e79a0 - std::backtrace_rs::backtrace::libunwind::trace::h74532f8b485906a1
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/../../backtrace/src/backtrace/libunwind.rs:90:5
   1:     0x7f1e3a1e79a0 - std::backtrace_rs::backtrace::trace_unsynchronized::h2c8425dd0be5999b
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/../../backtrace/src/backtrace/mod.rs:66:5
   2:     0x7f1e3a1e79a0 - std::sys_common::backtrace::_print_fmt::hd8ce6577f46119ce
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/sys_common/backtrace.rs:67:5
   3:     0x7f1e3a1e79a0 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6289eeebc1f97d57
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/sys_common/backtrace.rs:46:22
   4:     0x7f1e3a255f2f - core::fmt::write::h3d3f8a8bb9d4c367
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/core/src/fmt/mod.rs:1092:17
   5:     0x7f1e3a1dbcc2 - std::io::Write::write_fmt::ha117cbfa72664a7b
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/io/mod.rs:1572:15
   6:     0x7f1e3a1eb7e5 - std::sys_common::backtrace::_print::h112e8a4ac2720a21
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/sys_common/backtrace.rs:49:5
   7:     0x7f1e3a1eb7e5 - std::sys_common::backtrace::print::h2a4ab5824600ebce
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/sys_common/backtrace.rs:36:9
   8:     0x7f1e3a1eb7e5 - std::panicking::default_hook::{{closure}}::h399ab06c3b83a1f2
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/panicking.rs:208:50
   9:     0x7f1e3a1eb293 - std::panicking::default_hook::h7481e93ca9c2d739
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/panicking.rs:225:9
  10:     0x7f1e3a9f97eb - rustc_driver::report_ice::h95067985755a845f
  11:     0x7f1e2a3620d3 - <alloc::boxed::Box<F,A> as core::ops::function::Fn<Args>>::call::h5d01fb651b450bfc
                               at /nix/store/kamyzfvnm1l12i1km5qlqak3md8b86cw-rust-default-1.52.0/lib/rustlib/src/rust/library/alloc/src/boxed.rs:1560:9
  12:     0x7f1e2a3c9816 - proc_macro::bridge::client::<impl proc_macro::bridge::Bridge>::enter::{{closure}}::{{closure}}::h3adc72702b16a2dd
                               at /nix/store/kamyzfvnm1l12i1km5qlqak3md8b86cw-rust-default-1.52.0/lib/rustlib/src/rust/library/proc_macro/src/bridge/client.rs:320:21
  13:     0x7f1e3a1ebf50 - std::panicking::rust_panic_with_hook::h01f9d4e9485ccb8b
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/panicking.rs:595:17
  14:     0x7f1e3a1ebac7 - std::panicking::begin_panic_handler::{{closure}}::h25bff4eb752cc444
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/panicking.rs:497:13
  15:     0x7f1e3a1e7e5c - std::sys_common::backtrace::__rust_end_short_backtrace::h5dfd3f7920e58cc5
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/sys_common/backtrace.rs:141:18
  16:     0x7f1e3a1eba29 - rust_begin_unwind
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/panicking.rs:493:5
  17:     0x7f1e3a1b017b - std::panicking::begin_panic_fmt::h5ea05d30b78d4fb2
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/panicking.rs:435:5
  18:     0x7f1e3ca12208 - rustc_query_system::query::plumbing::incremental_verify_ich::h47a128fd2d21d867
  19:     0x7f1e3ca20940 - rustc_query_system::query::plumbing::load_from_disk_and_cache_in_memory::hf2afe55e6c58afa6
  20:     0x7f1e3c9d1245 - rustc_query_system::query::plumbing::get_query_impl::hf9d3349225b93a37
  21:     0x7f1e3ca9cdef - <rustc_query_impl::Queries as rustc_middle::ty::query::QueryEngine>::native_libraries::haa37d73efd384498
  22:     0x7f1e3ccaa5ce - rustc_codegen_ssa::base::<impl rustc_codegen_ssa::CrateInfo>::new::h3187b5f7ea6bae4c
  23:     0x7f1e3c67bd6e - rustc_codegen_ssa::back::write::start_async_codegen::h1ec7167f2e5faa59
  24:     0x7f1e3c6bd632 - <rustc_codegen_llvm::LlvmCodegenBackend as rustc_codegen_ssa::traits::backend::CodegenBackend>::codegen_crate::hffa2e429d7f8b220
  25:     0x7f1e3c643c81 - rustc_interface::passes::QueryContext::enter::h7609beb470aa8216
  26:     0x7f1e3c64c646 - rustc_interface::queries::Queries::ongoing_codegen::hdae2d9fee8a32b6b
  27:     0x7f1e3c60a213 - rustc_interface::queries::<impl rustc_interface::interface::Compiler>::enter::h1d93706229b66524
  28:     0x7f1e3c604901 - rustc_span::with_source_map::h7c8bdfd885d5eae7
  29:     0x7f1e3c60b1be - rustc_interface::interface::create_compiler_and_run::h3a24cbe4fe4fc490
  30:     0x7f1e3c605798 - scoped_tls::ScopedKey<T>::set::hb080f16fd4dd580e
  31:     0x7f1e3c60b56b - std::sys_common::backtrace::__rust_begin_short_backtrace::h19d7a04d4fdf4bf2
  32:     0x7f1e3c622bb5 - core::ops::function::FnOnce::call_once{{vtable.shim}}::hb375368372f2550c
  33:     0x7f1e3a1fb73a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::h72cc29c4d47b6ef1
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/alloc/src/boxed.rs:1546:9
  34:     0x7f1e3a1fb73a - <alloc::boxed::Box<F,A> as core::ops::function::FnOnce<Args>>::call_once::hb6246b2a6989332b
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/alloc/src/boxed.rs:1546:9
  35:     0x7f1e3a1fb73a - std::sys::unix::thread::Thread::new::thread_start::h09db1c25841995a6
                               at /rustc/88f19c6dab716c6281af7602e30f413e809c5974/library/std/src/sys/unix/thread.rs:71:17
  36:     0x7f1e3a134e9e - start_thread
  37:     0x7f1e3a05449f - __GI___clone
  38:                0x0 - <unknown>

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.52.0 (88f19c6da 2021-05-03) running on x86_64-unknown-linux-gnu

note: compiler flags: -C embed-bitcode=no -C debuginfo=2 -C incremental -C link-arg=-fuse-ld=lld -C target-cpu=native --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
#0 [native_libraries] looking up the native libraries of a linked crate
end of query stack
error: could not compile `helix-core`

To learn more, run the command again with --verbose.
