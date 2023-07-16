 
   Compiling ethcore-stratum v1.5.0 (file:///files/src/parity/stratum)
     Running `/files/src/parity/target/debug/build/ethcore-stratum-2db126478cc7e718/build-script-build`
error: internal compiler error: /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/map/mod.rs:395: node 37599 is inlined but not present in map

       Fresh ethabi v0.2.2
     Running `/files/src/parity/target/debug/build/ethcore-ipc-tests-7dbfe4f9161e4a64/build-script-build`
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports

note: run with `RUST_BACKTRACE=1` for a backtrace

thread "rustc" panicked at "Box<Any>", /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_errors/lib.rs:423
stack backtrace:
   1:     0x7f92daddec2c - std::sys::imp::backtrace::tracing::imp::write::h2a972e172776bc73
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x7f92dadecf5e - std::panicking::default_hook::{{closure}}::h3350be9abe4c8496
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:351
   3:     0x7f92dadecb03 - std::panicking::default_hook::h255964940ef72e84
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:361
   4:     0x7f92daded407 - std::panicking::rust_panic_with_hook::ha02a85ff57cdb669
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:555
   5:     0x7f92d37548ea - std::panicking::begin_panic::h063125b7da6f1fca
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:517
   6:     0x7f92d376979d - rustc_errors::Handler::bug::h563da3c520e22a27
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_errors/lib.rs:423
   7:     0x7f92d813c9f1 - rustc::session::opt_span_bug_fmt::{{closure}}::h5c454616325c5e52
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/session/mod.rs:788
   8:     0x7f92d813c4ce - rustc::session::opt_span_bug_fmt::h3659754ea96d3141
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:1021
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/session/mod.rs:784
   9:     0x7f92d813c132 - rustc::session::bug_fmt::haeec79526b0fafd5
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/session/mod.rs:768
  10:     0x7f92d805897f - rustc::hir::map::Map::read::h0dd82ac56c35db58
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/map/mod.rs:395
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/map/mod.rs:288
  11:     0x7f92d80590b4 - rustc::hir::map::Map::body::hb9371f9755e96bac
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/map/mod.rs:473
  12:     0x7f92d84c0fd4 - rustc_const_eval::eval::eval_const_expr_partial::h5722325983da1967
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_const_eval/eval.rs:904
  13:     0x7f92d84c39a8 - rustc_const_eval::eval::eval_const_expr_partial::h5722325983da1967
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_const_eval/eval.rs:768
  14:     0x7f92d953e020 - <rustc_passes::consts::CheckCrateVisitor<"a, "tcx> as rustc::hir::intravisit::Visitor<"tcx>>::visit_expr::h85d01a7eaaa7ac4a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_passes/consts.rs:222
  15:     0x7f92d953dec5 - <rustc_passes::consts::CheckCrateVisitor<"a, "tcx> as rustc::hir::intravisit::Visitor<"tcx>>::visit_expr::h85d01a7eaaa7ac4a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/<walk_list macros>:2
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/intravisit.rs:254
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/intravisit.rs:968
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_passes/consts.rs:214
  16:     0x7f92d953dec5 - <rustc_passes::consts::CheckCrateVisitor<"a, "tcx> as rustc::hir::intravisit::Visitor<"tcx>>::visit_expr::h85d01a7eaaa7ac4a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/<walk_list macros>:2
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/intravisit.rs:254
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/intravisit.rs:968
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_passes/consts.rs:214
  17:     0x7f92d953db30 - <rustc_passes::consts::CheckCrateVisitor<"a, "tcx> as rustc::hir::intravisit::Visitor<"tcx>>::visit_expr::h85d01a7eaaa7ac4a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/intravisit.rs:962
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_passes/consts.rs:214
  18:     0x7f92d953d9fe - <rustc_passes::consts::CheckCrateVisitor<"a, "tcx> as rustc::hir::intravisit::Visitor<"tcx>>::visit_expr::h85d01a7eaaa7ac4a
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/obj/<walk_list macros>:2
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/intravisit.rs:254
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/intravisit.rs:986
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_passes/consts.rs:214
  19:     0x7f92d953bc79 - <rustc_passes::consts::CheckCrateVisitor<"a, "tcx> as rustc::hir::intravisit::Visitor<"tcx>>::visit_nested_body::h87ecaaaf26707b47
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/intravisit.rs:402
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/intravisit.rs:220
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_passes/consts.rs:141
  20:     0x7f92d953eed2 - rustc_passes::consts::check_crate::hefa7ba1bb2019f02
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/intravisit.rs:293
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/itemlikevisit.rs:89
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/dep_graph/visit.rs:64
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/hir/mod.rs:465
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/dep_graph/visit.rs:75
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/mod.rs:2660
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_passes/consts.rs:440
  21:     0x7f92db18795e - rustc_driver::driver::phase_3_run_analysis_passes::{{closure}}::h21a6521e909907a7
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:891
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/util/common.rs:34
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:889
  22:     0x7f92db17aded - rustc_driver::driver::phase_3_run_analysis_passes::h20919355616dc567
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:992
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/local.rs:253
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:989
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:976
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/local.rs:253
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:973
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc/ty/context.rs:743
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:859
  23:     0x7f92db16a111 - rustc_driver::driver::compile_input::hc831e6818a12286f
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/driver.rs:163
  24:     0x7f92db1b07d8 - rustc_driver::run_compiler::ha4de160a9d909b83
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:221
  25:     0x7f92db0bcd28 - std::panicking::try::do_call::h2ff0d6696d3add9d
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:1117
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:137
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/librustc_driver/lib.rs:1051
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panic.rs:296
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:460
  26:     0x7f92dadf626a - __rust_maybe_catch_panic
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libpanic_unwind/lib.rs:98
  27:     0x7f92db0e6e7b - <F as alloc::boxed::FnBox<A>>::call_box::h81baf0e61cbf35f2
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panicking.rs:436
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/panic.rs:361
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/thread/mod.rs:302
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/liballoc/boxed.rs:605
  28:     0x7f92dadebdc4 - std::sys::imp::thread::Thread::new::thread_start::h27fa49fe81f658e4
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/liballoc/boxed.rs:615
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys_common/thread.rs:21
                        at /buildslave/rust-buildbot/slave/nightly-dist-rustc-linux/build/src/libstd/sys/unix/thread.rs:84
  29:     0x7f92d2f006a9 - start_thread
  30:     0x7f92daaa613c - __clone
  31:                0x0 - <unknown>

error: Could not compile `ethcore-util`.

Caused by:
  process didn"t exit successfully: `rustc --crate-name ethcore_util util/src/lib.rs --crate-type lib -g --cfg feature="default" -C metadata=1d5921a30039d660 -C extra-filename=-1d5921a30039d660 --out-dir /files/src/parity/target/debug/deps --emit=dep-info,link -L dependency=/files/src/parity/target/debug/deps --extern crypto=/files/src/parity/target/debug/deps/libcrypto-84c1bd1acc23cedb.rlib --extern vergen=/files/src/parity/target/debug/deps/libvergen-f4d1784ffca5370d.rlib --extern lru_cache=/files/src/parity/target/debug/deps/liblru_cache-2b7d03af29d0b249.rlib --extern time=/files/src/parity/target/debug/deps/libtime-02018c58facc89ee.rlib --extern table=/files/src/parity/target/debug/deps/libtable-d10b2403a7590532.rlib --extern elastic_array=/files/src/parity/target/debug/deps/libelastic_array-76c7d18bdb1ab8c0.rlib --extern itertools=/files/src/parity/target/debug/deps/libitertools-c611d01ca9f11451.rlib --extern log=/files/src/parity/target/debug/deps/liblog-1ce22d3a92f37841.rlib --extern tiny_keccak=/files/src/parity/target/debug/deps/libtiny_keccak-912f10d04717b632.rlib --extern rlp=/files/src/parity/target/debug/deps/librlp-0a9b69548f7fe9eb.rlib --extern rocksdb=/files/src/parity/target/debug/deps/librocksdb-4610773f70548b2d.rlib --extern ansi_term=/files/src/parity/target/debug/deps/libansi_term-a16cb51797899298.rlib --extern ethcore_devtools=/files/src/parity/target/debug/deps/libethcore_devtools-385d831303c0ca7f.rlib --extern ethcore_bigint=/files/src/parity/target/debug/deps/libethcore_bigint-d5795da4ef5ddb39.rlib --extern rustc_serialize=/files/src/parity/target/debug/deps/librustc_serialize-7afcde5ec1806fb9.rlib --extern using_queue=/files/src/parity/target/debug/deps/libusing_queue-41f98ebaf021b73a.rlib --extern rand=/files/src/parity/target/debug/deps/librand-a266d0434ee969c4.rlib --extern regex=/files/src/parity/target/debug/deps/libregex-40fac7c1de4f07cd.rlib --extern lazy_static=/files/src/parity/target/debug/deps/liblazy_static-1b77b520c29e5601.rlib --extern env_logger=/files/src/parity/target/debug/deps/libenv_logger-8d599e137f4b61eb.rlib --extern libc=/files/src/parity/target/debug/deps/liblibc-8c7226756bef4c8b.rlib --extern arrayvec=/files/src/parity/target/debug/deps/libarrayvec-f895db3c9dd92920.rlib --extern ethcore_bloom_journal=/files/src/parity/target/debug/deps/libethcore_bloom_journal-87e01edf5945e962.rlib --extern target_info=/files/src/parity/target/debug/deps/libtarget_info-699acdb356524e63.rlib --extern sha3=/files/src/parity/target/debug/deps/libsha3-3537757725c038c7.rlib --extern heapsize=/files/src/parity/target/debug/deps/libheapsize-ff63dc1fee64e153.rlib --extern secp256k1=/files/src/parity/target/debug/deps/libsecp256k1-8f4f97d88d5a5434.rlib --extern parking_lot=/files/src/parity/target/debug/deps/libparking_lot-568d257331396c45.rlib -L native=/files/src/parity/target/debug/build/rust-crypto-c0f524fa795e4be8/out -L native=/files/src/parity/target/debug/build/rocksdb-sys-a7b4c59ac47296f6/out -L native=/files/src/parity/target/debug/build/rocksdb-sys-a7b4c59ac47296f6/out -L native=/files/src/parity/target/debug/build/sha3-428e03d3118958a6/out -L native=/files/src/parity/target/debug/build/eth-secp256k1-4244243ecb0de896/out` (exit code: 101)

