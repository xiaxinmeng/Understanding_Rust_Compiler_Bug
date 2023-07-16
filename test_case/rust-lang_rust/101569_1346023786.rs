plain
diff of stderr:

19   --> $SRC_DIR/core/src/fmt/mod.rs:LL:COL
20    |
21 LL |     arg_new!(new_upper_hex, UpperHex);
-    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `core::fmt::ArgumentV1::<'a>::new_upper_hex`
+    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ArgumentV1::<'a>::new_upper_hex`
23    = note: this error originates in the macro `$crate::__export::format_args` which comes from the expansion of the macro `arg_new` (in Nightly builds, run with -Z macro-backtrace for more info)
25 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl/ifmt-unimpl.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fmt/ifmt-unimpl.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage1/bin/rustc" "/checkout/src/test/ui/fmt/ifmt-unimpl.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fmt/ifmt-unimpl/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: the trait bound `str: UpperHex` is not satisfied
   |
   |
LL |     format!("{:X}", "3");
   |                     ^^^ the trait `UpperHex` is not implemented for `str`
   |
   = help: the following other types implement trait `UpperHex`:
             &T
             NonZeroI128
             NonZeroI16
             NonZeroI32
             NonZeroI64
             NonZeroI64
             NonZeroI8
             NonZeroIsize
           and 21 others
   = note: required for `&str` to implement `UpperHex`
note: required by a bound in `core::fmt::ArgumentV1::<'a>::new_upper_hex`
   |
   |
LL |     arg_new!(new_upper_hex, UpperHex);
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ required by this bound in `ArgumentV1::<'a>::new_upper_hex`
   = note: this error originates in the macro `$crate::__export::format_args` which comes from the expansion of the macro `arg_new` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
