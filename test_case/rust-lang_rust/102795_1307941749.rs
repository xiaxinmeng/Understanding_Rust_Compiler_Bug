plain
test [assembly] src/test/assembly/asm/nvptx-types.rs ... ok
test [assembly] src/test/assembly/asm/riscv-types.rs#riscv32 ... ok
test [assembly] src/test/assembly/asm/aarch64-modifiers.rs ... ok
test [assembly] src/test/assembly/asm/mips-types.rs#mips32 ... ok
test [assembly] src/test/assembly/is_aligned.rs#opt-size ... ignored
test [assembly] src/test/assembly/is_aligned.rs#opt-speed ... ignored
test [assembly] src/test/assembly/nvptx-arch-default.rs ... ignored
test [assembly] src/test/assembly/nvptx-arch-emit-asm.rs ... ignored
test [assembly] src/test/assembly/nvptx-arch-link-arg.rs ... ignored
test [assembly] src/test/assembly/nvptx-arch-target-cpu.rs ... ignored
---
---- src/ptr/const_ptr.rs - ptr::const_ptr::*constT::is_aligned (line 1392) stdout ----
error[E0080]: evaluation of constant value failed
 --> src/ptr/const_ptr.rs:1398:15
  |
9 | const _: () = assert!(!CONST_PTR.cast::<i64>().is_aligned());
  |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: !CONST_PTR.cast::<i64>().is_aligned()', src/ptr/const_ptr.rs:9:15
  = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> src/ptr/const_ptr.rs:1399:15
  --> src/ptr/const_ptr.rs:1399:15
   |
10 | const _: () = assert!(!CONST_PTR.wrapping_add(1).cast::<i64>().is_aligned());
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: !CONST_PTR.wrapping_add(1).cast::<i64>().is_aligned()', src/ptr/const_ptr.rs:10:15
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0080`.
Couldn't compile the test.
---- src/ptr/const_ptr.rs - ptr::const_ptr::*constT::is_aligned (line 1371) stdout ----
error[E0080]: evaluation of constant value failed
  --> src/ptr/const_ptr.rs:1384:5
   |
16 |     assert!(!ptr1.is_aligned());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: !ptr1.is_aligned()', src/ptr/const_ptr.rs:16:5
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error


For more information about this error, try `rustc --explain E0080`.
Couldn't compile the test.
---- src/ptr/const_ptr.rs - ptr::const_ptr::*constT::is_aligned (line 1412) stdout ----
error[E0080]: evaluation of constant value failed
  --> src/ptr/const_ptr.rs:1425:5
   |
16 |     assert!(!ptr2.is_aligned());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: !ptr2.is_aligned()', src/ptr/const_ptr.rs:16:5
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error


For more information about this error, try `rustc --explain E0080`.
Couldn't compile the test.
---- src/ptr/mut_ptr.rs - ptr::mut_ptr::*mutT::is_aligned (line 1643) stdout ----
error[E0080]: evaluation of constant value failed
  --> src/ptr/mut_ptr.rs:1656:5
   |
16 |     assert!(!ptr1.is_aligned());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: !ptr1.is_aligned()', src/ptr/mut_ptr.rs:16:5
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error


For more information about this error, try `rustc --explain E0080`.
Couldn't compile the test.
---- src/ptr/mut_ptr.rs - ptr::mut_ptr::*mutT::is_aligned (line 1684) stdout ----
error[E0080]: evaluation of constant value failed
  --> src/ptr/mut_ptr.rs:1697:5
   |
16 |     assert!(!ptr2.is_aligned());
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: !ptr2.is_aligned()', src/ptr/mut_ptr.rs:16:5
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error


For more information about this error, try `rustc --explain E0080`.
Couldn't compile the test.
---- src/ptr/mut_ptr.rs - ptr::mut_ptr::*mutT::is_aligned (line 1664) stdout ----
error[E0080]: evaluation of constant value failed
 --> src/ptr/mut_ptr.rs:1670:15
  |
9 | const _: () = assert!(!CONST_PTR.cast::<i64>().is_aligned());
  |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: !CONST_PTR.cast::<i64>().is_aligned()', src/ptr/mut_ptr.rs:9:15
  = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> src/ptr/mut_ptr.rs:1671:15
  --> src/ptr/mut_ptr.rs:1671:15
   |
10 | const _: () = assert!(!CONST_PTR.wrapping_add(1).cast::<i64>().is_aligned());
   |               ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: !CONST_PTR.wrapping_add(1).cast::<i64>().is_aligned()', src/ptr/mut_ptr.rs:10:15
   = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors

---
    src/ptr/mut_ptr.rs - ptr::mut_ptr::*mutT::is_aligned (line 1684)

test result: FAILED. 3920 passed; 6 failed; 36 ignored; 0 measured; 0 filtered out; finished in 67.22s

error: doctest failed, to rerun pass `-p core --doc`
