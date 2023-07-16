plain
Dropping 2
Dropping 3
Dropping 4
------------------------------------------
--- stderr -------------------------------
thread '<unnamed>' panicked at 'Oh no', /checkout/src/test/ui/array-slice-vec/box-of-array-of-drop-1.rs:31:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'expect: 0xffffffff actual: 0x56781234', /checkout/src/test/ui/array-slice-vec/box-of-array-of-drop-1.rs:52:5


---- [ui] ui/array-slice-vec/box-of-array-of-drop-2.rs stdout ----

---
Dropping 2
Dropping 3
Dropping 4
------------------------------------------
--- stderr -------------------------------
thread '<unnamed>' panicked at 'Oh no', /checkout/src/test/ui/array-slice-vec/box-of-array-of-drop-2.rs:31:21
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'expect: 0xffffffff actual: 0x56781234', /checkout/src/test/ui/array-slice-vec/box-of-array-of-drop-2.rs:52:5


---- [ui] ui/array-slice-vec/nested-vec-3.rs stdout ----

---
Dropping 2
Dropping 3
Dropping 4
------------------------------------------
--- stderr -------------------------------
thread '<unnamed>' panicked at 'Oh no', /checkout/src/test/ui/array-slice-vec/nested-vec-3.rs:32:21
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
thread 'main' panicked at 'expect: 0xffffffff actual: 0x56781234', /checkout/src/test/ui/array-slice-vec/nested-vec-3.rs:59:5



failures:
