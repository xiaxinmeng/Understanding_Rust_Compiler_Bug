
failures:
    [compile-fail] compile-fail/asm-bad-clobber.rs
    [compile-fail] compile-fail/asm-in-bad-modifier.rs
    [compile-fail] compile-fail/asm-misplaced-option.rs
    [compile-fail] compile-fail/asm-out-assign-imm.rs
    [compile-fail] compile-fail/asm-out-no-modifier.rs
    [compile-fail] compile-fail/asm-out-read-uninit.rs

test result: FAILED. 2569 passed; 6 failed; 13 ignored; 0 measured

thread 'main' panicked at 'Some tests failed', /root/rustc-1.14.0+dfsg1/src/tools/compiletest/src/main.rs:304
stack backtrace:
   1:     0x3fffb6d33c13 - std::sys::imp::backtrace::tracing::imp::write::h917062bce4ff48c3
                        at /root/rustc-1.14.0+dfsg1/src/libstd/sys/unix/backtrace/tracing/gcc_s.rs:42
   2:     0x3fffb6d74a5f - std::panicking::default_hook::{{closure}}::h0bacac31b5ed1870
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:247
   3:     0x3fffb6d61df7 - std::panicking::default_hook::h5897799da33ece67
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:263
   4:     0x3fffb6d62a3f - std::panicking::rust_panic_with_hook::h109e116a3a861224
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:451
   5:         0x591ccbb3 - std::panicking::begin_panic::h634e2b37a96f78d4
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:413
   6:         0x59270543 - compiletest::run_tests::hd41b3b3a007825c7
                        at /root/rustc-1.14.0+dfsg1/src/tools/compiletest/src/main.rs:304
   7:         0x5926a0fb - compiletest::main::he8a84d59ab8da8df
                        at /root/rustc-1.14.0+dfsg1/src/tools/compiletest/src/main.rs:72
   8:     0x3fffb6d74baf - core::ops::FnOnce::call_once::h346b3ee6a997d309
   9:     0x3fffb6d623ef - std::panicking::try::do_call::h006598d33c40d6c2
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:356
  10:     0x3fffb6d7e197 - __rust_try
  11:     0x3fffb6d7de73 - __rust_maybe_catch_panic
                        at /root/rustc-1.14.0+dfsg1/src/libpanic_unwind/lib.rs:97
  12:     0x3fffb6d62123 - std::panicking::try::hb2dca1165100b5a8
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panicking.rs:332
  13:     0x3fffb6d0ffef - std::panic::catch_unwind::h7cf531dfc5b2ca07
                        at /root/rustc-1.14.0+dfsg1/src/libstd/panic.rs:351
  14:     0x3fffb6d63c47 - std::rt::lang_start::hd661476ce2fc2931
                        at /root/rustc-1.14.0+dfsg1/src/libstd/rt.rs:57
  15:         0x5928397b - main
  16:     0x3fffb6a2289b - <unknown>
/root/rustc-1.14.0+dfsg1/mk/tests.mk:771: recipe for target 'tmp/check-stage2-T-powerpc64le-unknown-linux-gnu-H-powerpc64le-unknown-linux-gnu-cfail.ok' failed
make[2]: *** [tmp/check-stage2-T-powerpc64le-unknown-linux-gnu-H-powerpc64le-unknown-linux-gnu-cfail.ok] Error 101
