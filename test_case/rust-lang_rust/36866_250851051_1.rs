
Building stage1 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling core v0.0.0 (file:///home/logic/build/rust/src/libcore)
error: internal compiler error: /home/logic/build/rust/src/librustc_trans/back/symbol_names.rs:127: can't do that here

note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread 'rustc' panicked at 'Box<Any>', /home/logic/build/rust/src/librustc_errors/lib.rs:656
stack backtrace:
   1:     0x7f100ef05a19 - std::sys::backtrace::tracing::imp::write::hb24583f57c381d48
                        at /home/logic/build/rust/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f100ef11d5f - std::panicking::default_hook::{{closure}}::h053e41d48a9d8bdd
                        at /home/logic/build/rust/src/libstd/panicking.rs:247
   3:     0x7f100ef10685 - std::panicking::default_hook::h6c2ff9328b5b71ea
                        at /home/logic/build/rust/src/libstd/panicking.rs:257
   4:     0x7f100ef10cd8 - std::panicking::rust_panic_with_hook::h3a147a19a402009b
                        at /home/logic/build/rust/src/libstd/panicking.rs:451
   5:     0x7f100791d42a - std::panicking::begin_panic::he38469cae18dfcf9
                        at /home/logic/build/rust/src/libstd/panicking.rs:413
   6:     0x7f100792056d - rustc_errors::Handler::bug::hd1eb3a6f9a8caec3
                        at /home/logic/build/rust/src/librustc_errors/lib.rs:656
   7:     0x7f100bea045c - rustc::session::opt_span_bug_fmt::{{closure}}::hc3e364c89c445f7d
                        at /home/logic/build/rust/src/librustc/session/mod.rs:719
   8:     0x7f100be834f9 - rustc::session::opt_span_bug_fmt::hf5158fed8c586ce6
                        at /home/logic/build/rust/src/librustc/ty/context.rs:968
                        at /home/logic/build/rust/src/librustc/ty/context.rs:957
                        at /home/logic/build/rust/src/libstd/thread/local.rs:245
                        at /home/logic/build/rust/src/librustc/ty/context.rs:953
                        at /home/logic/build/rust/src/librustc/ty/context.rs:968
                        at /home/logic/build/rust/src/librustc/session/mod.rs:715
   9:     0x7f100be831e2 - rustc::session::bug_fmt::hde66aa3463c27ccf
                        at /home/logic/build/rust/src/librustc/session/mod.rs:699
  10:     0x7f100d9d0ebf - <rustc_trans::back::symbol_names::Sha256Hasher<'a> as core::hash::Hasher>::write_usize::hac13ff130c0d0f07
                        at /home/logic/build/rust/src/librustc_trans/back/symbol_names.rs:127
  11:     0x7f100d9b3b88 - core::hash::impls::<impl core::hash::Hash for usize>::hash::h3b89fd9180b6d7d3
                        at /home/logic/build/rust/src/libcore/hash/mod.rs:298
  12:     0x7f100d9bf9c1 - rustc::hir::map::definitions::DefPath::deterministic_hash_to::h45cd42ef5632da97
                        at /home/logic/build/rust/src/libcore/hash/mod.rs:382
                        at /home/logic/build/rust/src/libcollections/vec.rs:1295
                        at /home/logic/build/rust/src/librustc/hir/map/definitions.rs:141
  13:     0x7f100d9d10b4 - rustc_trans::back::symbol_names::get_symbol_hash::h903c41caef7f887f
                        at /home/logic/build/rust/src/librustc_trans/back/symbol_names.rs:163
                        at /home/logic/build/rust/src/librustc/util/common.rs:82
                        at /home/logic/build/rust/src/librustc_trans/back/symbol_names.rs:156
  14:     0x7f100d948831 - rustc_trans::back::symbol_names::<impl rustc_trans::monomorphize::Instance<'tcx>>::symbol_name::h37f8eac78e887770
                        at /home/logic/build/rust/src/librustc_trans/back/symbol_names.rs:272
  15:     0x7f100da13289 - rustc_trans::trans_item::TransItem::compute_symbol_name::he7a5a7ea40f995ea
                        at /home/logic/build/rust/src/librustc_trans/trans_item.rs:232
  16:     0x7f100d900ae9 - rustc_trans::base::collect_and_partition_translation_items::h43464d1f035609b2
                        at /home/logic/build/rust/src/librustc_trans/symbol_map.rs:38
                        at /home/logic/build/rust/src/libcore/ops.rs:2644
                        at /home/logic/build/rust/src/libcore/option.rs:383
                        at /home/logic/build/rust/src/libcore/iter/mod.rs:896
                        at /home/logic/build/rust/src/libcollections/vec.rs:1452
                        at /home/logic/build/rust/src/libcore/iter/iterator.rs:1200
                        at /home/logic/build/rust/src/librustc_trans/symbol_map.rs:37
                        at /home/logic/build/rust/src/librustc_trans/base.rs:1870
  17:     0x7f100d8fb281 - rustc_trans::base::trans_crate::hafc3c22a3636de6e
                        at /home/logic/build/rust/src/librustc_trans/base.rs:1633
  18:     0x7f100f2ae22e - rustc_driver::driver::phase_4_translate_to_llvm::h7ef578b7f6d743ab
                        at /home/logic/build/rust/src/librustc_driver/driver.rs:1034
                        at /home/logic/build/rust/src/librustc/util/common.rs:38
                        at /home/logic/build/rust/src/librustc_driver/driver.rs:1032
  19:     0x7f100f2bb8e8 - rustc_driver::driver::compile_input::{{closure}}::ha9905cb0c86acf73
                        at /home/logic/build/rust/src/librustc_driver/driver.rs:206
  20:     0x7f100f2b718a - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::hfa236a7f6e8e4d7e
                        at /home/logic/build/rust/src/librustc_driver/driver.rs:988
  21:     0x7f100f2ac051 - rustc_driver::driver::phase_3_run_analysis_passes::h3fc292c6a6d36760
                        at /home/logic/build/rust/src/librustc/ty/context.rs:941
                        at /home/logic/build/rust/src/libstd/thread/local.rs:245
                        at /home/logic/build/rust/src/librustc/ty/context.rs:938
                        at /home/logic/build/rust/src/librustc/ty/context.rs:925
                        at /home/logic/build/rust/src/libstd/thread/local.rs:245
                        at /home/logic/build/rust/src/librustc/ty/context.rs:922
                        at /home/logic/build/rust/src/librustc/ty/context.rs:717
                        at /home/logic/build/rust/src/librustc_driver/driver.rs:857
  22:     0x7f100f29e504 - rustc_driver::driver::compile_input::h33bc6555b8b491ce
                        at /home/logic/build/rust/src/librustc_driver/driver.rs:172
  23:     0x7f100f1c3882 - rustc_driver::run_compiler::h0222bbb96666f1c6
                        at /home/logic/build/rust/src/librustc_driver/lib.rs:224
  24:     0x7f100f1b4c81 - std::panicking::try::do_call::h18ba198b82f0b751
                        at /home/logic/build/rust/src/librustc_driver/lib.rs:1145
                        at /home/logic/build/rust/src/librustc_driver/lib.rs:137
                        at /home/logic/build/rust/src/librustc_driver/lib.rs:1079
                        at /home/logic/build/rust/src/libstd/panic.rs:255
                        at /home/logic/build/rust/src/libstd/panicking.rs:356
  25:     0x7f100ef18267 - __rust_maybe_catch_panic
                        at /home/logic/build/rust/src/libpanic_unwind/lib.rs:91
  26:     0x7f100f1bea68 - <F as alloc::boxed::FnBox<A>>::call_box::h81f7688a0dfccda8
                        at /home/logic/build/rust/src/libstd/panicking.rs:332
                        at /home/logic/build/rust/src/libstd/panic.rs:311
                        at /home/logic/build/rust/src/libstd/thread/mod.rs:277
                        at /home/logic/build/rust/src/liballoc/boxed.rs:591
  27:     0x7f100eee3f53 - std::sys::thread::Thread::new::thread_start::hc35e66ca5755f7c6
                        at /home/logic/build/rust/src/liballoc/boxed.rs:601
                        at /home/logic/build/rust/src/libstd/sys/common/thread.rs:21
                        at /home/logic/build/rust/src/libstd/sys/unix/thread.rs:71
  28:     0x7f10070bc6f9 - start_thread
  29:     0x7f100ebb6b5c - clone
  30:                0x0 - <unknown>

error: Could not compile `core`.

To learn more, run the command again with --verbose.


command did not execute successfully: "/home/logic/build/rust-out/dev-build/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "-j" "8" "--target" "x86_64-unknown-linux-gnu" "--release" "--features" " jemalloc backtrace" "--manifest-path" "/home/logic/build/rust/src/rustc/std_shim/Cargo.toml"
expected success, got: exit code: 101


logic@sys:~/build/rust-out/dev-build$
