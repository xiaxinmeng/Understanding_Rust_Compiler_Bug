plain
---- [ui] src/test/ui/consts/missing_span_in_backtrace.rs stdout ----
diff of stderr:

1 error[E0080]: evaluation of constant value failed
-   --> /rustc/xyz/library/core/src/ptr/mod.rs:1135:9
+   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
3    |
-    = note: unable to copy parts of a pointer from memory at alloc10
+    = note: unable to copy parts of a pointer from memory at alloc9
6    = help: this code performed an operation that depends on the underlying bytes representing a pointer
6    = help: this code performed an operation that depends on the underlying bytes representing a pointer
7    = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported

8 note: inside `std::ptr::read::<MaybeUninit<MaybeUninit<u8>>>`
-   --> /rustc/xyz/library/core/src/ptr/mod.rs:1135:9
+   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
10 note: inside `mem::swap_simple::<MaybeUninit<MaybeUninit<u8>>>`
-   --> /rustc/xyz/library/core/src/mem/mod.rs:773:17
+   --> $SRC_DIR/core/src/mem/mod.rs:LL:COL
12 note: inside `ptr::swap_nonoverlapping_simple_untyped::<MaybeUninit<u8>>`
-   --> /rustc/xyz/library/core/src/ptr/mod.rs:944:9
+   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
14 note: inside `swap_nonoverlapping::<MaybeUninit<u8>>`
-   --> /rustc/xyz/library/core/src/ptr/mod.rs:925:14
+   --> $SRC_DIR/core/src/ptr/mod.rs:LL:COL
16 note: inside `X`
18    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/missing_span_in_backtrace/missing_span_in_backtrace.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/missing_span_in_backtrace.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/missing_span_in_backtrace.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/missing_span_in_backtrace" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=i586-unknown-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/missing_span_in_backtrace/auxiliary" "-Z" "simulate-remapped-rust-src-base=/rustc/xyz" "-Z" "translate-remapped-path-to-local-path=no"
stdout: none
--- stderr -------------------------------
  --> /rustc/640f34aa1ba9726cefe969a0220a074bb5cdda54/library/core/src/ptr/mod.rs:1135:9
   |
   |
   = note: unable to copy parts of a pointer from memory at alloc9
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: this code performed an operation that depends on the underlying bytes representing a pointer
   = help: the absolute address of a pointer is not known at compile-time, so such operations are not supported
note: inside `std::ptr::read::<MaybeUninit<MaybeUninit<u8>>>`
  --> /rustc/640f34aa1ba9726cefe969a0220a074bb5cdda54/library/core/src/ptr/mod.rs:1135:9
note: inside `mem::swap_simple::<MaybeUninit<MaybeUninit<u8>>>`
  --> /rustc/640f34aa1ba9726cefe969a0220a074bb5cdda54/library/core/src/mem/mod.rs:773:17
note: inside `ptr::swap_nonoverlapping_simple_untyped::<MaybeUninit<u8>>`
  --> /rustc/640f34aa1ba9726cefe969a0220a074bb5cdda54/library/core/src/ptr/mod.rs:944:9
note: inside `swap_nonoverlapping::<MaybeUninit<u8>>`
  --> /rustc/640f34aa1ba9726cefe969a0220a074bb5cdda54/library/core/src/ptr/mod.rs:925:14
note: inside `X`
   |
LL | /         ptr::swap_nonoverlapping(
LL | /         ptr::swap_nonoverlapping(
LL | |             &mut ptr1 as *mut _ as *mut MaybeUninit<u8>,
LL | |             &mut ptr2 as *mut _ as *mut MaybeUninit<u8>,
LL | |             mem::size_of::<&i32>(),
LL | |         );

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
---

7   = help: the trait `std::fmt::Display` is not implemented for `MyError`
8   = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
9 note: required by a bound in `std::error::Error`
-  --> /rustc/xyz/library/core/src/error.rs:31:26
+  --> $SRC_DIR/core/src/error.rs:LL:COL
12   = note: required by this bound in `std::error::Error`
13 

20   = help: the trait `Debug` is not implemented for `MyError`
20   = help: the trait `Debug` is not implemented for `MyError`
21   = note: add `#[derive(Debug)]` to `MyError` or manually `impl Debug for MyError`
22 note: required by a bound in `std::error::Error`
-  --> /rustc/xyz/library/core/src/error.rs:31:18
+  --> $SRC_DIR/core/src/error.rs:LL:COL
25   = note: required by this bound in `std::error::Error`
25   = note: required by this bound in `std::error::Error`
26 help: consider annotating `MyError` with `#[derive(Debug)]`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-71363/issue-71363.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/issue-71363.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-71363.rs" "-Zthreads=1" "--target=i586-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-71363" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i586-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=i586-unknown-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-71363/auxiliary" "-Z" "simulate-remapped-rust-src-base=/rustc/xyz" "-Z" "translate-remapped-path-to-local-path=no" "-Z" "ui-testing=no"
stdout: none
--- stderr -------------------------------
error[E0277]: `MyError` doesn't implement `std::fmt::Display`
  |
4 | impl std::error::Error for MyError {}
4 | impl std::error::Error for MyError {}
  |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted with the default formatter
  = help: the trait `std::fmt::Display` is not implemented for `MyError`
  = note: in format strings you may be able to use `{:?}` (or {:#?} for pretty-print) instead
note: required by a bound in `std::error::Error`
 --> /rustc/640f34aa1ba9726cefe969a0220a074bb5cdda54/library/core/src/error.rs:31:26
 --> /rustc/640f34aa1ba9726cefe969a0220a074bb5cdda54/library/core/src/error.rs:31:26
  |
  = note: required by this bound in `std::error::Error`

error[E0277]: `MyError` doesn't implement `Debug`
  |
4 | impl std::error::Error for MyError {}
4 | impl std::error::Error for MyError {}
  |      ^^^^^^^^^^^^^^^^^ `MyError` cannot be formatted using `{:?}`
  = help: the trait `Debug` is not implemented for `MyError`
  = help: the trait `Debug` is not implemented for `MyError`
  = note: add `#[derive(Debug)]` to `MyError` or manually `impl Debug for MyError`
 --> /rustc/640f34aa1ba9726cefe969a0220a074bb5cdda54/library/core/src/error.rs:31:18
  |
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=i586-unknown-linux-gnu
  = note: required by this bound in `std::error::Error`
  = note: required by this bound in `std::error::Error`
help: consider annotating `MyError` with `#[derive(Debug)]`
3 | #[derive(Debug)]
  |

error: aborting due to 2 previous errors
