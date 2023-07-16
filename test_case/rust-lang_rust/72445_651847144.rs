
thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:344:22
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
------------------------------------------
stderr:
------------------------------------------
warning: the feature `track_caller` has been stable since 1.46.0 and no longer requires an attribute to enable
  --> /checkout/src/test/ui/asm/sym.rs:6:17
   |
LL | #![feature(asm, track_caller, thread_local)]
   |                 ^^^^^^^^^^^^
   |
   = note: `#[warn(stable_features)]` on by default

warning: 1 warning emitted


------------------------------------------



failures:
    [ui] ui/asm/sym.rs
