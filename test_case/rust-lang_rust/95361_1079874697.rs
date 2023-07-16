plain

---- [ui] ui/consts/std/alloc.rs stdout ----
diff of 64bit.stderr:

- error[E0080]: it is undefined behavior to use this value
-   --> $DIR/alloc.rs:8:1
+ error[E0080]: evaluation of constant value failed
+   --> $SRC_DIR/core/src/mem/valid_align.rs:LL:COL
3    |
+ LL |         debug_assert!(align.is_power_of_two());
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |         |
+    |         |
+    |         the evaluated program panicked at 'assertion failed: align.is_power_of_two()', $SRC_DIR/core/src/mem/valid_align.rs:LL:COL
+    |         inside `mem::valid_align::ValidAlign::new_unchecked` at $SRC_DIR/core/src/macros/mod.rs:LL:COL
+   ::: $SRC_DIR/core/src/alloc/layout.rs:LL:COL
+    |
+    |
+ LL |         Layout { size_: size, align_: unsafe { ValidAlign::new_unchecked(align) } }
+    |                                                -------------------------------- inside `Layout::from_size_align_unchecked` at $SRC_DIR/core/src/alloc/layout.rs:LL:COL
+   ::: $DIR/alloc.rs:8:41
+    |
+    |
4 LL | const LAYOUT_INVALID: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x00) };
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ type validation failed at .align_: encountered 0, but expected something greater or equal to 1
+    |                                         ----------------------------------------------- inside `LAYOUT_INVALID` at $DIR/alloc.rs:8:41
6    |
-    = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
-    = note: the raw bytes of the constant (size: 16, align: 8) {
-            }
+    = note: this error originates in the macro `$crate::assert` (in Nightly builds, run with -Z macro-backtrace for more info)
11 
12 error: aborting due to previous error
12 error: aborting due to previous error
13 


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/alloc/alloc.64bit.stderr
To only update this specific test, also pass `--test-args consts/std/alloc.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/std/alloc.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/alloc" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/std/alloc/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/mem/valid_align.rs:25:9
   |
LL |         debug_assert!(align.is_power_of_two());
   |         |
   |         |
   |         the evaluated program panicked at 'assertion failed: align.is_power_of_two()', /checkout/library/core/src/mem/valid_align.rs:25:9
   |         inside `mem::valid_align::ValidAlign::new_unchecked` at /checkout/library/core/src/macros/mod.rs:219:13
  ::: /checkout/library/core/src/alloc/layout.rs:100:48
   |
   |
LL |         Layout { size_: size, align_: unsafe { ValidAlign::new_unchecked(align) } }
   |                                                -------------------------------- inside `Layout::from_size_align_unchecked` at /checkout/library/core/src/alloc/layout.rs:100:48
  ::: /checkout/src/test/ui/consts/std/alloc.rs:8:41
   |
   |
LL | const LAYOUT_INVALID: Layout = unsafe { Layout::from_size_align_unchecked(0x1000, 0x00) };
   |                                         ----------------------------------------------- inside `LAYOUT_INVALID` at /checkout/src/test/ui/consts/std/alloc.rs:8:41
   = note: this error originates in the macro `$crate::assert` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error


For more information about this error, try `rustc --explain E0080`.
------------------------------------------


---- [ui] ui/try-trait/bad-interconversion.rs stdout ----
diff of stderr:

12              <u8 as From<bool>>
13              <f32 as From<i16>>
14              <f32 as From<i8>>
+            and 72 others
+            and 72 others
16    = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, i32>>` for `Result<u64, u8>`
17 
18 error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/bad-interconversion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args try-trait/bad-interconversion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/try-trait/bad-interconversion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/try-trait/bad-interconversion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: `?` couldn't convert the error to `u8`
   |
   |
LL | fn result_to_result() -> Result<u64, u8> {
   |                          --------------- expected `u8` because of this
LL |     Ok(Err(123_i32)?)
   |                    ^ the trait `From<i32>` is not implemented for `u8`
   |
   = note: the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
   = help: the following implementations were found:
             <u8 as From<NonZeroU8>>
             <u8 as From<bool>>
             <f32 as From<i16>>
             <f32 as From<i8>>
           and 72 others
   = note: required because of the requirements on the impl of `FromResidual<Result<Infallible, i32>>` for `Result<u64, u8>`

error[E0277]: the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
   |
   |
LL | / fn option_to_result() -> Result<u64, String> {
LL | |     Some(3)?;
   | |            ^ use `.ok_or(...)?` to provide an error compatible with `Result<u64, String>`
LL | |     //~^ ERROR the `?` operator can only be used on `Result`s, not `Option`s, in a function that returns `Result`
LL | |     Ok(10)
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `Result<u64, String>`

error[E0277]: the `?` operator can only be used on `Result`s in a function that returns `Result`
   |
   |
LL | / fn control_flow_to_result() -> Result<u64, String> {
LL | |     Ok(ControlFlow::Break(123)?)
   | |                               ^ this `?` produces `ControlFlow<{integer}, Infallible>`, which is incompatible with `Result<u64, String>`
LL | |     //~^ ERROR the `?` operator can only be used on `Result`s in a function that returns `Result`
LL | | }
   | |_- this function returns a `Result`
   |
   = help: the trait `FromResidual<ControlFlow<{integer}, Infallible>>` is not implemented for `Result<u64, String>`

error[E0277]: the `?` operator can only be used on `Option`s, not `Result`s, in a function that returns `Option`
   |
LL | / fn result_to_option() -> Option<u16> {
LL | / fn result_to_option() -> Option<u16> {
LL | |     Some(Err("hello")?)
   | |                      ^ use `.ok()?` if you want to discard the `Result<Infallible, &str>` error information
LL | |     //~^ ERROR the `?` operator can only be used on `Option`s, not `Result`s, in a function that returns `Option`
LL | | }
   | |_- this function returns an `Option`
   |
   = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `Option<u16>`

error[E0277]: the `?` operator can only be used on `Option`s in a function that returns `Option`
   |
LL | / fn control_flow_to_option() -> Option<u64> {
LL | / fn control_flow_to_option() -> Option<u64> {
LL | |     Some(ControlFlow::Break(123)?)
   | |                                 ^ this `?` produces `ControlFlow<{integer}, Infallible>`, which is incompatible with `Option<u64>`
LL | |     //~^ ERROR the `?` operator can only be used on `Option`s in a function that returns `Option`
LL | | }
   | |_- this function returns an `Option`
   |
   = help: the trait `FromResidual<ControlFlow<{integer}, Infallible>>` is not implemented for `Option<u64>`

error[E0277]: the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
   |
   |
LL | / fn result_to_control_flow() -> ControlFlow<String> {
LL | |     ControlFlow::Continue(Err("hello")?)
   | |                                       ^ this `?` produces `Result<Infallible, &str>`, which is incompatible with `ControlFlow<String>`
LL | |     //~^ ERROR the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<Result<Infallible, &str>>` is not implemented for `ControlFlow<String>`

error[E0277]: the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
   |
   |
LL | / fn option_to_control_flow() -> ControlFlow<u64> {
LL | |     Some(3)?;
   | |            ^ this `?` produces `Option<Infallible>`, which is incompatible with `ControlFlow<u64>`
LL | |     //~^ ERROR the `?` operator can only be used on `ControlFlow`s in a function that returns `ControlFlow`
LL | |     ControlFlow::Break(10)
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<Option<Infallible>>` is not implemented for `ControlFlow<u64>`

error[E0277]: the `?` operator in a function that returns `ControlFlow<B, _>` can only be used on other `ControlFlow<B, _>`s (with the same Break type)
   |
   |
LL | / fn control_flow_to_control_flow() -> ControlFlow<i64> {
LL | |     ControlFlow::Break(4_u8)?;
   | |                             ^ this `?` produces `ControlFlow<u8, Infallible>`, which is incompatible with `ControlFlow<i64>`
LL | |     //~^ ERROR the `?` operator in a function that returns `ControlFlow<B, _>` can only be used on other `ControlFlow<B, _>`s
LL | |     ControlFlow::Continue(())
LL | | }
   | |_- this function returns a `ControlFlow`
   |
   = help: the trait `FromResidual<ControlFlow<u8, Infallible>>` is not implemented for `ControlFlow<i64>`
   = note: unlike `Result`, there's no `From`-conversion performed for `ControlFlow`
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
