plain
diff of stderr:

5    |                    ^
6    |
7    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
-    = note: this error originates in the macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `panic` which eventually calls macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
10 error[E0015]: cannot call non-const formatting macro in constant functions
11   --> $DIR/format.rs:11:22

14    |                      ^
14    |                      ^
15    |
16    = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
-    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `println` which calls macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
18 
19 error: `Arguments::<'a>::new_v1` is not yet stable as a const fn

23    |     ^^^^^^^^^^^^^^^^^^^
24    |
25    = help: add `#![feature(const_fmt_arguments_new)]` to the crate attributes to enable
25    = help: add `#![feature(const_fmt_arguments_new)]` to the crate attributes to enable
-    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `println` which calls macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
27 
28 error[E0015]: cannot call non-const fn `_print` in constant functions

52    |
53    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
54    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
54    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: this error originates in the macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `panic` which eventually calls macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
57 error: erroneous constant used
58   --> $DIR/format.rs:11:14

71    |
71    |
72    = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
73    = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
-    = note: this error originates in the macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `println` which calls macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
76 error: aborting due to 8 previous errors
77 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/format/format.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/format.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/format.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/format" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/format/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0015]: cannot call non-const formatting macro in constant functions
   |
LL |     panic!("{:?}", 0);
   |                    ^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = note: this error originates in the macro `panic` which eventually calls macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
error[E0015]: cannot call non-const formatting macro in constant functions
  --> /checkout/src/test/ui/consts/const-eval/format.rs:11:22
   |
LL |     println!("{:?}", 0);
LL |     println!("{:?}", 0);
   |                      ^
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = note: this error originates in the macro `println` which calls macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error: `Arguments::<'a>::new_v1` is not yet stable as a const fn
   |
LL |     println!("{:?}", 0);
   |     ^^^^^^^^^^^^^^^^^^^
   |
   |
   = help: add `#![feature(const_fmt_arguments_new)]` to the crate attributes to enable
   = note: this error originates in the macro `println` which calls macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0015]: cannot call non-const fn `_print` in constant functions
   |
LL |     println!("{:?}", 0);
   |     ^^^^^^^^^^^^^^^^^^^
   |
   |
   = note: calls in constant functions are limited to constant functions, tuple structs and tuple variants
   = note: this error originates in the macro `println` (in Nightly builds, run with -Z macro-backtrace for more info)
error: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/format.rs:2:12
   |
LL |     panic!("{:?}", 0);
LL |     panic!("{:?}", 0);
   |            ^^^^^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/format.rs:2:20
  --> /checkout/src/test/ui/consts/const-eval/format.rs:2:20
   |
LL |     panic!("{:?}", 0);
   |                    ^ referenced constant has errors
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in the macro `panic` which eventually calls macro `$crate::const_format_args` (in Nightly builds, run with -Z macro-backtrace for more info)
error: erroneous constant used
  --> /checkout/src/test/ui/consts/const-eval/format.rs:11:14
   |
LL |     println!("{:?}", 0);
---
   |                      ^ referenced constant has errors
   |
   = warning: this was previously accepted by the compiler but is being phased out; it will become a hard error in a future release!
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
   = note: this error originates in the macro `println` which calls macro `$crate::format_args_nl` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 8 previous errors

For more information about this error, try `rustc --explain E0015`.
------------------------------------------
