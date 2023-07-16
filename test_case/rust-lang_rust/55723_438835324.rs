console
$ RUST_BACKTRACE=1 cargo +other2 doc
thread '<unnamed>' panicked at 'assertion failed: bpos.to_u32() >= mbc.pos.to_u32() + mbc.bytes as u32', libsyntax/source_map.rs:842:17
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: std::panicking::rust_panic_with_hook
             at libstd/panicking.rs:476
   5: std::panicking::begin_panic
             at /home/misdreavus/git/rust-builds/src/libstd/panicking.rs:410
   6: syntax::source_map::SourceMap::bytepos_to_file_charpos
             at libsyntax/source_map.rs:842
   7: syntax::source_map::SourceMap::lookup_char_pos
             at libsyntax/source_map.rs:326
   8: <syntax::source_map::SourceMap as rustc_errors::SourceMapper>::lookup_char_pos
             at libsyntax/source_map.rs:955
   9: rustc_errors::emitter::EmitterWriter::get_multispan_max_line_num                                                                                              at librustc_errors/emitter.rs:736
  10: <rustc_errors::emitter::EmitterWriter as rustc_errors::emitter::Emitter>::emit
             at librustc_errors/emitter.rs:759                                                                                                 [80/136]
             at librustc_errors/emitter.rs:1330
             at librustc_errors/emitter.rs:81
  11: rustc_errors::Handler::emit_db
             at librustc_errors/lib.rs:722
  12: rustc_errors::diagnostic_builder::DiagnosticBuilder::emit
             at librustc_errors/diagnostic_builder.rs:98
  13: rustdoc::passes::collect_intra_doc_links::resolution_failure
             at librustdoc/passes/collect_intra_doc_links.rs:573
  14: <rustdoc::passes::collect_intra_doc_links::LinkCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at librustdoc/passes/collect_intra_doc_links.rs:432
  15: <alloc::vec::Vec<T> as alloc::vec::SpecExtend<T, I>>::from_iter
             at librustdoc/fold.rs:110
             at /home/misdreavus/git/rust-builds/src/libcore/iter/mod.rs:1640
             at /home/misdreavus/git/rust-builds/src/liballoc/vec.rs:1788
  16: rustdoc::fold::DocFolder::fold_inner_recur
             at /home/misdreavus/git/rust-builds/src/liballoc/vec.rs:1700
             at /home/misdreavus/git/rust-builds/src/libcore/iter/iterator.rs:1476
             at librustdoc/fold.rs:110
             at librustdoc/fold.rs:37
  17: rustdoc::fold::DocFolder::fold_item_recur
             at librustdoc/fold.rs:100
  18: <rustdoc::passes::collect_intra_doc_links::LinkCollector<'a, 'tcx, 'rcx, 'cstore> as rustdoc::fold::DocFolder>::fold_item
             at librustdoc/passes/collect_intra_doc_links.rs:461
  19: rustdoc::passes::collect_intra_doc_links::collect_intra_doc_links
             at librustdoc/fold.rs:115
             at /home/misdreavus/git/rust-builds/src/libcore/option.rs:632
             at librustdoc/fold.rs:115
             at librustdoc/passes/collect_intra_doc_links.rs:41
  20: rustdoc::core::run_core::{{closure}}::{{closure}}
             at librustdoc/core.rs:613
  21: rustc::ty::context::tls::enter_global
             at /home/misdreavus/git/rust-builds/src/librustc_driver/driver.rs:1355
             at /home/misdreavus/git/rust-builds/src/librustc/ty/context.rs:2082
             at /home/misdreavus/git/rust-builds/src/librustc/ty/context.rs:2050
             at /home/misdreavus/git/rust-builds/src/librustc/ty/context.rs:1989
             at /home/misdreavus/git/rust-builds/src/librustc/ty/context.rs:2049
             at /home/misdreavus/git/rust-builds/src/librustc/ty/context.rs:2081                                                                                    at /home/misdreavus/git/rust-builds/src/librustc/ty/context.rs:2039
             at /home/misdreavus/git/rust-builds/src/libstd/thread/local.rs:309
             at /home/misdreavus/git/rust-builds/src/libstd/thread/local.rs:255                                                                [40/136]
             at /home/misdreavus/git/rust-builds/src/librustc/ty/context.rs:2031
             at /home/misdreavus/git/rust-builds/src/libstd/thread/local.rs:309
             at /home/misdreavus/git/rust-builds/src/libstd/thread/local.rs:255
             at /home/misdreavus/git/rust-builds/src/librustc/ty/context.rs:2023
             at /home/misdreavus/git/rust-builds/src/librustc/ty/context.rs:2061
  22: rustc::ty::context::TyCtxt::create_and_enter
             at /home/misdreavus/git/rust-builds/src/librustc/ty/context.rs:1247
  23: rustdoc::core::run_core::{{closure}}
             at /home/misdreavus/git/rust-builds/src/librustc_driver/driver.rs:1263
             at librustdoc/core.rs:495
  24: syntax::with_globals
             at /home/misdreavus/git/rust-builds/src/librustc_driver/driver.rs:76
             at /home/misdreavus/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at /home/misdreavus/git/rust-builds/src/librustc_driver/driver.rs:75
             at librustdoc/core.rs:402
             at librustdoc/lib.rs:413
             at /home/misdreavus/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at /home/misdreavus/git/rust-builds/src/libsyntax/lib.rs:123
             at /home/misdreavus/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at /home/misdreavus/git/rust-builds/src/libsyntax/lib.rs:122
  25: std::panicking::try::do_call
             at librustdoc/lib.rs:410
             at /home/misdreavus/git/rust-builds/src/librustc_driver/lib.rs:1640
             at /home/misdreavus/git/rust-builds/src/libstd/panic.rs:319
             at /home/misdreavus/git/rust-builds/src/libstd/panicking.rs:310
  26: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  27: std::panicking::try
             at /home/misdreavus/git/rust-builds/src/libstd/panicking.rs:289
  28: rustc_driver::monitor
             at /home/misdreavus/git/rust-builds/src/libstd/panic.rs:398
             at /home/misdreavus/git/rust-builds/src/librustc_driver/lib.rs:1554
             at /home/misdreavus/git/rust-builds/src/librustc_driver/lib.rs:1565
             at /home/misdreavus/git/rust-builds/src/librustc_driver/lib.rs:1639
  29: rustdoc::main_args
             at librustdoc/lib.rs:410
             at librustdoc/lib.rs:385                                                                                                                    30: syntax::with_globals
             at librustdoc/lib.rs:108
             at /home/misdreavus/git/rust-builds/src/libcore/option.rs:424                                                                      [0/136]
             at librustdoc/lib.rs:108
             at /home/misdreavus/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at /home/misdreavus/git/rust-builds/src/libsyntax/lib.rs:123
             at /home/misdreavus/.cargo/registry/src/github.com-1ecc6299db9ec823/scoped-tls-0.1.2/src/lib.rs:155
             at /home/misdreavus/git/rust-builds/src/libsyntax/lib.rs:122
  31: std::panicking::try::do_call
             at /home/misdreavus/git/rust-builds/src/libstd/thread/mod.rs:477
             at /home/misdreavus/git/rust-builds/src/libstd/panic.rs:319
             at /home/misdreavus/git/rust-builds/src/libstd/panicking.rs:310
  32: __rust_maybe_catch_panic
             at libpanic_unwind/lib.rs:102
  33: std::panicking::try
             at /home/misdreavus/git/rust-builds/src/libstd/panicking.rs:289
  34: <F as alloc::boxed::FnBox<A>>::call_box
             at /home/misdreavus/git/rust-builds/src/libstd/panic.rs:398
             at /home/misdreavus/git/rust-builds/src/libstd/thread/mod.rs:476
             at /home/misdreavus/git/rust-builds/src/liballoc/boxed.rs:672
  35: std::sys_common::thread::start_thread
             at /home/misdreavus/git/rust-builds/src/liballoc/boxed.rs:682
             at libstd/sys_common/thread.rs:24
  36: std::sys::unix::thread::Thread::new::thread_start
             at libstd/sys/unix/thread.rs:90
  37: start_thread
  38: clone

error: internal compiler error: unexpected panic

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: rustc 1.32.0-dev running on x86_64-unknown-linux-gnu

error: Could not document `test_ice`.

Caused by:
  process didn't exit successfully: `rustdoc --edition=2018 --crate-name test_ice src/lib.rs --color always -o /home/misdreavus/clones/test_ice/target/doc -L dependency=/home/misdreavus/clones/test_ice/target/debug/deps` (exit code: 1)
