
{"message":"src/librustc/ty/context.rs:207: node type <T>::Stream (hir_id=HirId { owner: DefIndex(351), local_id: 7 }) with HirId::owner DefId(0:351 ~ tokio_postgres[f364]::config[0]::{{impl}}[1]::connect[0]::{{opaque}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0:350 ~ tokio_postgres[f364]::config[0]::{{impl}}[1]::connect[0])","code":null,"level":"error: internal compiler error","spans":[],"children":[],"rendered":"\u001b[0m\u001b[1m\u001b[38;5;9merror: internal compiler error\u001b[0m\u001b[0m\u001b[1m: src/librustc/ty/context.rs:207: node type <T>::Stream (hir_id=HirId { owner: DefIndex(351), local_id: 7 }) with HirId::owner DefId(0:351 ~ tokio_postgres[f364]::config[0]::{{impl}}[1]::connect[0]::{{opaque}}[0]) cannot be placed in TypeckTables with local_id_root DefId(0:350 ~ tokio_postgres[f364]::config[0]::{{impl}}[1]::connect[0])\u001b[0m\n\n"}
thread 'rustc' panicked at 'Box<Any>', src/librustc_errors/lib.rs:925:9
stack backtrace:
   0:     0x7f2430374fa4 - backtrace::backtrace::libunwind::trace::hc456a3386f914390
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/libunwind.rs:88
   1:     0x7f2430374fa4 - backtrace::backtrace::trace_unsynchronized::h4a2def0c1d55554e
                               at /cargo/registry/src/github.com-1ecc6299db9ec823/backtrace-0.3.37/src/backtrace/mod.rs:66
   2:     0x7f2430374fa4 - std::sys_common::backtrace::_print_fmt::hf14ed68b6789b5fe
                               at src/libstd/sys_common/backtrace.rs:77
   3:     0x7f2430374fa4 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h6c47cffeeb327f7c
                               at src/libstd/sys_common/backtrace.rs:61
   4:     0x7f24303ad43c - core::fmt::write::hc59b451f5f876e7f
                               at src/libcore/fmt/mod.rs:1028
   5:     0x7f2430369247 - std::io::Write::write_fmt::ha233981d002c27e5
                               at src/libstd/io/mod.rs:1412
   6:     0x7f24303797ce - std::sys_common::backtrace::_print::hd01a0af84eb70ff6
                               at src/libstd/sys_common/backtrace.rs:65
   7:     0x7f24303797ce - std::sys_common::backtrace::print::h53f5322cbc293596
                               at src/libstd/sys_common/backtrace.rs:50
   8:     0x7f24303797ce - std::panicking::default_hook::{{closure}}::h0c7c40ba4f45458a
                               at src/libstd/panicking.rs:189
   9:     0x7f24303794d1 - std::panicking::default_hook::h03ff9eee3bf4dec0
                               at src/libstd/panicking.rs:206
  10:     0x7f24308775a3 - rustc_driver::report_ice::h0f63131b378c9f3b
  11:     0x7f2430379fac - std::panicking::rust_panic_with_hook::h07f06d6e0c8f4f10
                               at src/libstd/panicking.rs:473
  12:     0x7f24327759cd - std::panicking::begin_panic::h9d7394ea5657f31a
  13:     0x7f2432772803 - rustc_errors::HandlerInner::bug::h42e6c5a943eba4bc
  14:     0x7f243277149a - rustc_errors::Handler::bug::h376b0abf7c9f7a41
  15:     0x7f2431f297d3 - rustc::util::bug::opt_span_bug_fmt::{{closure}}::h52340496b0034c16
  16:     0x7f2431f19313 - rustc::ty::context::tls::with_opt::{{closure}}::hcc725a99a1df5481
  17:     0x7f2431f19063 - rustc::ty::context::tls::with_context_opt::h60475b0bf5b43fd2
  18:     0x7f2431f190a7 - rustc::ty::context::tls::with_opt::hb075469837d6ec1a
  19:     0x7f2431f296e8 - rustc::util::bug::opt_span_bug_fmt::h23ae4df70c7b8e17
  20:     0x7f2431f29652 - rustc::util::bug::bug_fmt::h77732941ee774d20
  21:     0x7f243237f386 - rustc::ty::context::validate_hir_id_for_typeck_tables::{{closure}}::h1bf14353a76241ad
  22:     0x7f2432394c99 - rustc::ty::context::tls::with::{{closure}}::h089dc1cbbef8a4dc                                                                                  23:     0x7f2432394c70 - rustc::ty::context::tls::with_context::{{closure}}::hda71ce779aa7316b                                                                          24:     0x7f2432393748 - rustc::ty::context::tls::with_context_opt::h20358a2772868641                                                                                   25:     0x7f2432393756 - rustc::ty::context::tls::with_context::hd3735f2af3f9cea3                                                                                       26:     0x7f2432394c86 - rustc::ty::context::tls::with::h30c911537cc4d084                                                                                               27:     0x7f243237fd93 - rustc::ty::context::TypeckTables::qpath_res::h2650b62724d822ec
  28:     0x7f24317fffc7 - rustc_save_analysis::SaveContext::get_path_res::h0334538ea7a16a84
  29:     0x7f243184f32d - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_ty::h620a82e83b71e293
  30:     0x7f243183ba2e - syntax::visit::walk_generic_args::h9977243a2d32688f
  31:     0x7f243184f392 - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_ty::h620a82e83b71e293
  32:     0x7f243183cc89 - syntax::visit::walk_ty::h33897830231a7d8f
  33:     0x7f243183ba2e - syntax::visit::walk_generic_args::h9977243a2d32688f
  34:     0x7f243184f392 - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_ty::h620a82e83b71e293
  35:     0x7f243184334f - rustc_save_analysis::dump_visitor::DumpVisitor::process_method::hd5d868bf2db2b388
  36:     0x7f243184a8c2 - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item::h3387f7ee37ffcd43
  37:     0x7f243184ab0a - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_item::h3387f7ee37ffcd43
  38:     0x7f2431847c9a - <rustc_save_analysis::dump_visitor::DumpVisitor as syntax::visit::Visitor>::visit_mod::h8e63e82b0c6a6f6d
  39:     0x7f24308c5047 - rustc::dep_graph::graph::DepGraph::with_ignore::h5ef3b489b7cf03fd
  40:     0x7f243085a5a0 - rustc_driver::run_compiler::{{closure}}::{{closure}}::{{closure}}::h5a2366d537f2594f
  41:     0x7f243084e785 - rustc_interface::passes::BoxedGlobalCtxt::access::{{closure}}::had036633395585f0
  42:     0x7f243096ec6a - rustc_interface::passes::create_global_ctxt::{{closure}}::h400c23d6ed07e01c
  43:     0x7f243087cee9 - rustc_interface::interface::run_compiler_in_existing_thread_pool::hb3c093a30d377a9b
  44:     0x7f2430890e02 - std::thread::local::LocalKey<T>::with::ha82e71cbf0ba6f43
  45:     0x7f243087968e - scoped_tls::ScopedKey<T>::set::h8a6696f5f7574b9a
  46:     0x7f24308af364 - syntax::with_globals::h9b1a343d709bf576
  47:     0x7f24308b44e2 - std::sys_common::backtrace::__rust_begin_short_backtrace::h510bf3be4fd5ed7b
  48:     0x7f243038a86a - __rust_maybe_catch_panic
                               at src/libpanic_unwind/lib.rs:80
  49:     0x7f24308c1849 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h75ba1b1af60e09af
  50:     0x7f243035b29f - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h7a4c92527aa5f8de
                               at /rustc/7979016aff545f7b41cc517031026020b340989d/src/liballoc/boxed.rs:942
  51:     0x7f2430389270 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h05f01d3e2aed4be6
                               at /rustc/7979016aff545f7b41cc517031026020b340989d/src/liballoc/boxed.rs:942
  51:     0x7f2430389270 - <alloc::boxed::Box<F> as core::ops::function::FnOnce<A>>::call_once::h05f01d3e2aed4be6                                                [0/887]                               at /rustc/7979016aff545f7b41cc517031026020b340989d/src/liballoc/boxed.rs:942
  52:     0x7f2430389270 - std::sys_common::thread::start_thread::heb20c5306076e966
                               at src/libstd/sys_common/thread.rs:13
  53:     0x7f2430389270 - std::sys::unix::thread::Thread::new::thread_start::h2d893a9b84fa7c6f
                               at src/libstd/sys/unix/thread.rs:79
  54:     0x7f24302ccfa3 - start_thread
  55:     0x7f24301e14cf - clone
  56:                0x0 - <unknown>

note: the compiler unexpectedly panicked. this is a bug.                                                                                                                
note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports                                                       
note: rustc 1.40.0-nightly (7979016af 2019-10-20) running on x86_64-unknown-linux-gnu

note: compiler flags: -C debuginfo=2 --crate-type lib

note: some of the compiler flags provided by cargo are hidden

query stack during panic:
end of query stack
{"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"\u001b[0m\u001b[1m\u001b[38;5;9merror\u001b[0m\u001b[0m\u001b[1m: aborting due to previous error\u001b[0m\n\n"}
{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"message":"could not compile `tokio-postgres`.\nprocess didn't exit successfully:
`/home/naim/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu/bin/rls --edition=2018 --crate-name tokio_postgres /home/naim/.cargo/registry/src/github.com-1ecc6299db9ec823/tokio-postgres-0.5.0-alpha.1/src/lib.rs --json=diagnostic-rendered-ansi --crate-type lib --emit=dep-info,metadata -C debuginfo=2 --cfg 'feature=\"default\"' --cfg 'feature=\"runtime\"' -C metadata=aa41584653f31878 -C extra-filename=-aa41584653f31878 --out-dir /home/naim/Projects/test/target/rls/debug/deps -L dependency=/home/naim/Projects/test/target/rls/debug/deps --extern bytes=/home/naim/Projects/test/target/rls/debug/deps/libbytes-c5f3d2710cde8058.rmeta --extern fallible_iterator=/home/naim/Projects/test/target/rls/debug/deps/libfallible_iterator-1e40a46ec2bf9c54.rmeta --extern futures=/home/naim/Projects/test/target/rls/debug/deps/libfutures-0a33e1cbc182f4fc.rmeta --extern log=/home/naim/Projects/test/target/rls/debug/deps/liblog-8cd4873c5e13f87d.rmeta --extern parking_lot=/home/naim/Projects/test/target/rls/debug/deps/libparking_lot-72f4d230f0b923e9.rmeta --extern percent_encoding=/home/naim/Projects/test/target/rls/debug/deps/libpercent_encoding-65edd9af765fbe29.rmeta --extern phf=/home/naim/Projects/test/target/rls/debug/deps/libphf-f0a5b1a9fbac9310.rmeta --extern pin_project=/home/naim/Projects/test/target/rls/debug/deps/libpin_project-a4775134cec7c295.rmeta --extern pin_utils=/home/naim/Projects/test/target/rls/debug/deps/libpin_utils-02a71be11439df54.rmeta --extern postgres_protocol=/home/naim/Projects/test/target/rls/debug/deps/libpostgres_protocol-126cc527cdb3df04.rmeta --extern postgres_types=/home/naim/Projects/test/target/rls/debug/deps/libpostgres_types-ec7a8daa42a58e39.rmeta --extern tokio=/home/naim/Projects/test/target/rls/debug/deps/libtokio-ee1804fafb97bbd6.rmeta --cap-lints allow --error-format=json --sysroot /home/naim/.rustup/toolchains/nightly-x86_64-unknown-linux-gnu` (exit code: 101)","range":{"end":{"character":0,"line":9999},"start":{"character":0,"line":0}},"severity":1}],"uri":"file:///home/naim/Projects/test/Cargo.toml"}}                                                                                                               
