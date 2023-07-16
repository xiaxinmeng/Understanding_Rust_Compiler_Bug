
test [compile-fail] compile-fail/xc-private-method2.rs ... ok
test [compile-fail] compile-fail/xcrate-private-by-default.rs ... ok

failures:

---- [compile-fail] compile-fail/range-1.rs stdout ----

error: expected error on line 19 not found: `core::iter::Iterator` is not implemented for the type `core::ops::Range<f32>`
status: exit code: 101
command: x86_64-unknown-linux-gnu/stage2/bin/rustc /home/emacs/build/rust/src/test/compile-fail/range-1.rs -L x86_64-unknown-linux-gnu/test/compile-fail --target=x86_64-unknown-linux-gnu -L x86_64-unknown-linux-gnu/test/compile-fail/range-1.stage2-x86_64-unknown-linux-gnulibaux -C prefer-dynamic -o x86_64-unknown-linux-gnu/test/compile-fail/range-1.stage2-x86_64-unknown-linux-gnu --cfg rtopt -O -L x86_64-unknown-linux-gnu/rt
stdout:
------------------------------------------

------------------------------------------
stderr:
------------------------------------------
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:15:13: 15:23 error: start and end of range have incompatible types: expected `usize`, found `isize` (expected usize, found isize) [E0308]
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:15     let _ = 0us..10is;
                                                                       ^~~~~~~~~~
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:19:5: 26:8 error: the trait `core::num::Int` is not implemented for the type `f32` [E0277]
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:19     for i in 0f32..42f32 {}
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:20     //~^ ERROR `core::iter::Iterator` is not implemented for the type `core::ops::Range<f32>`
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:21     //~^^ ERROR
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:22     //~^^^ ERROR
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:23     // FIXME(#21528) not fulfilled obligation error should be reported once, not thrice
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:24 
                                                           ...
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:27:17: 27:24 error: the trait `core::marker::Sized` is not implemented for the type `[usize]` [E0277]
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:27     let range = *arr..;
                                                                           ^~~~~~~
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:27:17: 27:24 note: `[usize]` does not have a constant size known at compile-time
/home/emacs/build/rust/src/test/compile-fail/range-1.rs:27     let range = *arr..;
                                                                           ^~~~~~~
error: aborting due to 3 previous errors

------------------------------------------

thread '[compile-fail] compile-fail/range-1.rs' panicked at 'explicit panic', /home/emacs/build/rust/src/compiletest/runtest.rs:1458



failures:
    [compile-fail] compile-fail/range-1.rs

test result: FAILED. 1643 passed; 1 failed; 22 ignored; 0 measured

thread '<main>' panicked at 'Some tests failed', /home/emacs/build/rust/src/compiletest/compiletest.rs:252
/home/emacs/build/rust/mk/tests.mk:729: recipe for target 'tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-cfail.ok' failed
make: *** [tmp/check-stage2-T-x86_64-unknown-linux-gnu-H-x86_64-unknown-linux-gnu-cfail.ok] Error 101


