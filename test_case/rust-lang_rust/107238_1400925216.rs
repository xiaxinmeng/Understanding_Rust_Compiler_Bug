plain

---- [ui] tests/ui/range/range-1.rs stdout ----
diff of stderr:

4 LL |     let _ = 0u32..10i32;
5    |                   ^^^^^ expected `u32`, found `i32`
- error[E0277]: the trait bound `bool: Step` is not satisfied
-   --> $DIR/range-1.rs:9:14
-    |
-    |
- LL |     for i in false..true {}
-    |              ^^^^^^^^^^^ the trait `Step` is not implemented for `bool`
-    = help: the following other types implement trait `Step`:
-              char
-              i128
-              i16
-              i16
-              i32
-              i64
-              i8
-              isize
-              u128
-            and 5 others
-    = note: required for `std::ops::Range<bool>` to implement `Iterator`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    = note: required for `std::ops::Range<bool>` to implement `IntoIterator`
- 
26 error[E0277]: the size for values of type `[{integer}]` cannot be known at compilation time
28    |

33 note: required by a bound in `RangeFrom`
34   --> $SRC_DIR/core/src/ops/range.rs:LL:COL
---
39 For more information about an error, try `rustc --explain E0277`.


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range-1/range-1.stderr
To only update this specific test, also pass `--test-args range/range-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/range/range-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/range/range-1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/range/range-1.rs:5:19
   |
   |
LL |     let _ = 0u32..10i32;
   |                   ^^^^^ expected `u32`, found `i32`

error[E0277]: the size for values of type `[{integer}]` cannot be known at compilation time
  --> fake-test-src-base/range/range-1.rs:14:17
   |
LL |     let range = *arr..;
   |
   |
   = help: the trait `Sized` is not implemented for `[{integer}]`
note: required by a bound in `RangeFrom`
  --> /rustc/FAKE_PREFIX/library/core/src/ops/range.rs:187:1
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0308.
For more information about an error, try `rustc --explain E0277`.
