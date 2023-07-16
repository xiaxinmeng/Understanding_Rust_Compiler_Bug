plain
.................................................................................................... 1900/12417
.................................................................................................... 2000/12417
........................F...................................i....................................... 2100/12417
.................................................................................................... 2200/12417
..............................................................................F..F...F.............. 2300/12417
.............................................................................F...................... 2500/12417
...........................F........................................................................ 2600/12417
.................................................................................................... 2700/12417
...................................................................................................i 2800/12417
---
failures:

---- [ui] ui/const-generics/issues/issue-67739.rs#min stdout ----

error in revision `min`: Error: expected failure status (Some(1)) but received status Some(101).
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/issues/issue-67739.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "min" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739.min" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/issues/issue-67739.min/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:54:32: could not fully normalize `fn() -> usize {std::mem::size_of::<<Self as Trait>::Associated>}`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (7959e68a6 2021-11-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [normalize_generic_arg_after_erasing_regions] normalizing `fn() -> usize {core::mem::size_of::<<Self as Trait>::Associated>}`
#1 [fn_abi_of_instance] computing call ABI of `core::mem::size_of::<<Self as Trait>::Associated>`
error: aborting due to previous error


------------------------------------------
---
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_libcore_bin.rs:8:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
3    |
- LL | const Z: () = panic!("cheese");
-    |               ^^^^^^^^^^^^^^^^ the evaluated program panicked at 'cheese', $DIR/const_panic_libcore_bin.rs:8:15
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `core::panicking::panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         |
+    |         the evaluated program panicked at 'cheese', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic_libcore_bin.rs:8:15
+    |
+    |
+ LL | const Z: () = panic!("cheese");
+    |               ---------------- inside `Z` at $SRC_DIR/core/src/panic.rs:LL:COL
9 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_libcore_bin.rs:11:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
11    |
11    |
- LL | const Y: () = unreachable!();
-    |               ^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore_bin.rs:11:15
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `core::panicking::panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
14    |
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic_libcore_bin.rs:11:15
+    |
+ LL | const Y: () = unreachable!();
+    |               -------------- inside `Y` at $SRC_DIR/core/src/panic.rs:LL:COL
17 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_libcore_bin.rs:14:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
19    |
19    |
- LL | const X: () = unimplemented!();
-    |               ^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic_libcore_bin.rs:14:15
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `core::panicking::panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         |
+    |         the evaluated program panicked at 'not implemented', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic_libcore_bin.rs:14:15
+    |
+    |
+ LL | const X: () = unimplemented!();
+    |               ---------------- inside `X` at $SRC_DIR/core/src/panic.rs:LL:COL
25 error: aborting due to 3 previous errors
26 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_bin/const_panic_libcore_bin.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore_bin.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_bin" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_bin/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `core::panicking::panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         |
   |         the evaluated program panicked at 'cheese', /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs:8:15
   |
   |
LL | const Z: () = panic!("cheese");
   |               ---------------- inside `Z` at /checkout/library/core/src/panic.rs:28:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `core::panicking::panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'internal error: entered unreachable code', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'internal error: entered unreachable code', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs:11:15
   |
LL | const Y: () = unreachable!();
   |               -------------- inside `Y` at /checkout/library/core/src/panic.rs:28:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `core::panicking::panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'not implemented', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'not implemented', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs:14:15
   |
LL | const X: () = unimplemented!();
   |               ---------------- inside `X` at /checkout/library/core/src/panic.rs:28:9
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0080`.

---
17 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic.rs:12:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
19    |
- LL | const Y: () = std::unreachable!();
-    |               ^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic.rs:12:15
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
22    |
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic.rs:12:15
+    |
+ LL | const Y: () = std::unreachable!();
+    |               ------------------- inside `Y` at $SRC_DIR/core/src/panic.rs:LL:COL
25 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic.rs:15:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
27    |
27    |
- LL | const X: () = std::unimplemented!();
-    |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic.rs:15:15
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         |
+    |         the evaluated program panicked at 'not implemented', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic.rs:15:15
+    |
+    |
+ LL | const X: () = std::unimplemented!();
+    |               --------------------- inside `X` at $SRC_DIR/core/src/panic.rs:LL:COL
33 error[E0080]: evaluation of constant value failed
34   --> $DIR/const_panic.rs:18:15

47    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
47    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
48 
49 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic.rs:24:20
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
51    |
- LL | const Z_CORE: () = core::panic!("cheese");
-    |                    ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'cheese', $DIR/const_panic.rs:24:20
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         |
+    |         the evaluated program panicked at 'cheese', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic.rs:24:20
+    |
+    |
+ LL | const Z_CORE: () = core::panic!("cheese");
+    |                    ---------------------- inside `Z_CORE` at $SRC_DIR/core/src/panic.rs:LL:COL
57 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic.rs:27:21
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
59    |
59    |
- LL | const Z2_CORE: () = core::panic!();
-    |                     ^^^^^^^^^^^^^^ the evaluated program panicked at 'explicit panic', $DIR/const_panic.rs:27:21
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         the evaluated program panicked at 'explicit panic', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         the evaluated program panicked at 'explicit panic', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
62    |
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic.rs:27:21
+    |
+ LL | const Z2_CORE: () = core::panic!();
+    |                     -------------- inside `Z2_CORE` at $SRC_DIR/core/src/panic.rs:LL:COL
65 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic.rs:30:20
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
67    |
67    |
- LL | const Y_CORE: () = core::unreachable!();
-    |                    ^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic.rs:30:20
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
70    |
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic.rs:30:20
+    |
+ LL | const Y_CORE: () = core::unreachable!();
+    |                    -------------------- inside `Y_CORE` at $SRC_DIR/core/src/panic.rs:LL:COL
73 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic.rs:33:20
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
75    |
75    |
- LL | const X_CORE: () = core::unimplemented!();
-    |                    ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic.rs:33:20
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         |
+    |         the evaluated program panicked at 'not implemented', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic.rs:33:20
+    |
+    |
+ LL | const X_CORE: () = core::unimplemented!();
+    |                    ---------------------- inside `X_CORE` at $SRC_DIR/core/src/panic.rs:LL:COL
81 error[E0080]: evaluation of constant value failed
82   --> $DIR/const_panic.rs:36:20



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/const_panic.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const_panic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:6:15
   |
LL | const Z: () = std::panic!("cheese");
   |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'cheese', /checkout/src/test/ui/consts/const-eval/const_panic.rs:6:15
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:9:16
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:9:16
   |
LL | const Z2: () = std::panic!();
   |
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'internal error: entered unreachable code', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'internal error: entered unreachable code', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic.rs:12:15
   |
LL | const Y: () = std::unreachable!();
   |               ------------------- inside `Y` at /checkout/library/core/src/panic.rs:28:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'not implemented', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'not implemented', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic.rs:15:15
   |
LL | const X: () = std::unimplemented!();
   |               --------------------- inside `X` at /checkout/library/core/src/panic.rs:28:9
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:18:15
   |
   |
LL | const W: () = std::panic!(MSG);
   |               ^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', /checkout/src/test/ui/consts/const-eval/const_panic.rs:18:15
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:21:16
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:21:16
   |
LL | const W2: () = std::panic!("{}", MSG);
   |                ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', /checkout/src/test/ui/consts/const-eval/const_panic.rs:21:16
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
  --> /checkout/library/core/src/panicking.rs:114:9
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         |
   |         the evaluated program panicked at 'cheese', /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic.rs:24:20
   |
   |
LL | const Z_CORE: () = core::panic!("cheese");
   |                    ---------------------- inside `Z_CORE` at /checkout/library/core/src/panic.rs:28:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'explicit panic', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'explicit panic', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic.rs:27:21
   |
LL | const Z2_CORE: () = core::panic!();
   |                     -------------- inside `Z2_CORE` at /checkout/library/core/src/panic.rs:25:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'internal error: entered unreachable code', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'internal error: entered unreachable code', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic.rs:30:20
   |
LL | const Y_CORE: () = core::unreachable!();
   |                    -------------------- inside `Y_CORE` at /checkout/library/core/src/panic.rs:28:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'not implemented', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'not implemented', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic.rs:33:20
   |
LL | const X_CORE: () = core::unimplemented!();
   |                    ---------------------- inside `X_CORE` at /checkout/library/core/src/panic.rs:28:9
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:36:20
   |
   |
LL | const W_CORE: () = core::panic!(MSG);
   |                    ^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', /checkout/src/test/ui/consts/const-eval/const_panic.rs:36:20
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:39:21
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:39:21
   |
LL | const W2_CORE: () = core::panic!("{}", MSG);
   |                     ^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', /checkout/src/test/ui/consts/const-eval/const_panic.rs:39:21
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 12 previous errors

---
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:6:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
3    |
- LL | const A: () = std::panic!("blåhaj");
-    |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'blåhaj', $DIR/const_panic_2021.rs:6:15
+ LL |         panic_str(msg);
+    |         |
+    |         |
+    |         the evaluated program panicked at 'blåhaj', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic_2021.rs:6:15
+    |
+    |
+ LL | const A: () = std::panic!("blåhaj");
+    |               --------------------- inside `A` at $SRC_DIR/core/src/panic.rs:LL:COL
9 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:9:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
11    |
11    |
- LL | const B: () = std::panic!();
-    |               ^^^^^^^^^^^^^ the evaluated program panicked at 'explicit panic', $DIR/const_panic_2021.rs:9:15
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         the evaluated program panicked at 'explicit panic', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         the evaluated program panicked at 'explicit panic', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
14    |
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic_2021.rs:9:15
+    |
+ LL | const B: () = std::panic!();
+    |               ------------- inside `B` at $SRC_DIR/core/src/panic.rs:LL:COL
17 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:12:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
19    |
19    |
- LL | const C: () = std::unreachable!();
-    |               ^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_2021.rs:12:15
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
22    |
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic_2021.rs:12:15
+    |
+ LL | const C: () = std::unreachable!();
+    |               ------------------- inside `C` at $SRC_DIR/core/src/panic.rs:LL:COL
25 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:15:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
27    |
27    |
- LL | const D: () = std::unimplemented!();
-    |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic_2021.rs:15:15
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         |
+    |         the evaluated program panicked at 'not implemented', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic_2021.rs:15:15
+    |
+    |
+ LL | const D: () = std::unimplemented!();
+    |               --------------------- inside `D` at $SRC_DIR/core/src/panic.rs:LL:COL
33 error[E0080]: evaluation of constant value failed
34   --> $DIR/const_panic_2021.rs:18:15

39    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
39    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
40 
41 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:21:20
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
43    |
- LL | const A_CORE: () = core::panic!("shark");
-    |                    ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'shark', $DIR/const_panic_2021.rs:21:20
+ LL |         panic_str(msg);
+    |         |
+    |         |
+    |         the evaluated program panicked at 'shark', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic_2021.rs:21:20
+    |
+    |
+ LL | const A_CORE: () = core::panic!("shark");
+    |                    --------------------- inside `A_CORE` at $SRC_DIR/core/src/panic.rs:LL:COL
49 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:24:20
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
51    |
51    |
- LL | const B_CORE: () = core::panic!();
-    |                    ^^^^^^^^^^^^^^ the evaluated program panicked at 'explicit panic', $DIR/const_panic_2021.rs:24:20
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         the evaluated program panicked at 'explicit panic', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         the evaluated program panicked at 'explicit panic', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
54    |
-    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic_2021.rs:24:20
+    |
+ LL | const B_CORE: () = core::panic!();
+    |                    -------------- inside `B_CORE` at $SRC_DIR/core/src/panic.rs:LL:COL
57 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:27:20
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
59    |
59    |
- LL | const C_CORE: () = core::unreachable!();
-    |                    ^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_2021.rs:27:20
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         the evaluated program panicked at 'internal error: entered unreachable code', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
62    |
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic_2021.rs:27:20
+    |
+ LL | const C_CORE: () = core::unreachable!();
+    |                    -------------------- inside `C_CORE` at $SRC_DIR/core/src/panic.rs:LL:COL
65 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_panic_2021.rs:30:20
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
67    |
67    |
- LL | const D_CORE: () = core::unimplemented!();
-    |                    ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic_2021.rs:30:20
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         |
+    |         the evaluated program panicked at 'not implemented', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/const_panic_2021.rs:30:20
+    |
+    |
+ LL | const D_CORE: () = core::unimplemented!();
+    |                    ---------------------- inside `D_CORE` at $SRC_DIR/core/src/panic.rs:LL:COL
73 error[E0080]: evaluation of constant value failed
74   --> $DIR/const_panic_2021.rs:33:20



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021/const_panic_2021.stderr
To only update this specific test, also pass `--test-args consts/const-eval/const_panic_2021.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_2021.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'blåhaj', /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:6:15
   |
   |
LL | const A: () = std::panic!("blåhaj");
   |               --------------------- inside `A` at /checkout/library/core/src/panic.rs:57:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'explicit panic', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'explicit panic', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:9:15
   |
LL | const B: () = std::panic!();
   |               ------------- inside `B` at /checkout/library/core/src/panic.rs:50:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'internal error: entered unreachable code', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'internal error: entered unreachable code', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:12:15
   |
LL | const C: () = std::unreachable!();
   |               ------------------- inside `C` at /checkout/library/core/src/panic.rs:28:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'not implemented', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'not implemented', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:15:15
   |
LL | const D: () = std::unimplemented!();
   |               --------------------- inside `D` at /checkout/library/core/src/panic.rs:28:9
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:18:15
   |
   |
LL | const E: () = std::panic!("{}", MSG);
   |               ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:18:15
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
  --> /checkout/library/core/src/panicking.rs:114:9
   |
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'shark', /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:21:20
   |
   |
LL | const A_CORE: () = core::panic!("shark");
   |                    --------------------- inside `A_CORE` at /checkout/library/core/src/panic.rs:57:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'explicit panic', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'explicit panic', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:24:20
   |
LL | const B_CORE: () = core::panic!();
   |                    -------------- inside `B_CORE` at /checkout/library/core/src/panic.rs:50:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'internal error: entered unreachable code', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'internal error: entered unreachable code', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:27:20
   |
LL | const C_CORE: () = core::unreachable!();
   |                    -------------------- inside `C_CORE` at /checkout/library/core/src/panic.rs:28:9
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'not implemented', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'not implemented', /checkout/library/core/src/panicking.rs:114:9
   |         inside `const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:30:20
   |
LL | const D_CORE: () = core::unimplemented!();
   |                    ---------------------- inside `D_CORE` at /checkout/library/core/src/panic.rs:28:9
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:33:20
   |
   |
LL | const E_CORE: () = core::panic!("{}", MSG);
   |                    ^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:33:20
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 10 previous errors

---
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/const-unwrap.rs:7:18
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
3    |
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `core::panicking::panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         |
+    |         the evaluated program panicked at 'called `Option::unwrap()` on a `None` value', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `core::panicking::const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
+   ::: $SRC_DIR/core/src/option.rs:LL:COL
+    |
+    |
+ LL |             None => panic!("called `Option::unwrap()` on a `None` value"),
+    |                     ----------------------------------------------------- inside `Option::<i32>::unwrap` at $SRC_DIR/core/src/panic.rs:LL:COL
+   ::: $DIR/const-unwrap.rs:7:18
+    |
+    |
4 LL | const BAR: i32 = Option::<i32>::None.unwrap();
-    |                  ^^^^^^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'called `Option::unwrap()` on a `None` value', $DIR/const-unwrap.rs:7:38
+    |                  ---------------------------- inside `BAR` at $DIR/const-unwrap.rs:7:18
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-unwrap/const-unwrap.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-unwrap.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-unwrap.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-unwrap" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-unwrap/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `core::panicking::panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         |
   |         the evaluated program panicked at 'called `Option::unwrap()` on a `None` value', /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/library/core/src/option.rs:746:21
   |
   |
LL |             None => panic!("called `Option::unwrap()` on a `None` value"),
   |                     ----------------------------------------------------- inside `Option::<i32>::unwrap` at /checkout/library/core/src/panic.rs:28:9
  ::: /checkout/src/test/ui/consts/const-unwrap.rs:7:18
   |
   |
LL | const BAR: i32 = Option::<i32>::None.unwrap();
   |                  ---------------------------- inside `BAR` at /checkout/src/test/ui/consts/const-unwrap.rs:7:18
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.

---
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/assert.rs:5:15
+   --> $SRC_DIR/core/src/panicking.rs:LL:COL
3    |
- LL | const _: () = assert!(false);
-    |               ^^^^^^^^^^^^^^ the evaluated program panicked at 'assertion failed: false', $DIR/assert.rs:5:15
+ LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
+    |     ----------------------------------------------- inside `core::panicking::panic` at $SRC_DIR/core/src/panicking.rs:LL:COL
+ LL |         panic_str(msg);
+    |         ^^^^^^^^^^^^^^
+    |         |
+    |         the evaluated program panicked at 'assertion failed: false', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         the evaluated program panicked at 'assertion failed: false', $SRC_DIR/core/src/panicking.rs:LL:COL
+    |         inside `core::panicking::const_panic_fmt` at $SRC_DIR/core/src/panicking.rs:LL:COL
6    |
-    = note: this error originates in the macro `assert` (in Nightly builds, run with -Z macro-backtrace for more info)
+   ::: $DIR/assert.rs:5:15
+    |
+ LL | const _: () = assert!(false);
+    |               -------------- inside `_` at $DIR/assert.rs:5:15
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/assert/assert.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/control-flow/assert.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/assert.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/assert" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/assert/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/library/core/src/panicking.rs:114:9
   |
LL |     panic_fmt(fmt::Arguments::new_v1(&[expr], &[]));
   |     ----------------------------------------------- inside `core::panicking::panic` at /checkout/library/core/src/panicking.rs:48:5
LL |         panic_str(msg);
   |         ^^^^^^^^^^^^^^
   |         |
   |         the evaluated program panicked at 'assertion failed: false', /checkout/library/core/src/panicking.rs:114:9
   |         the evaluated program panicked at 'assertion failed: false', /checkout/library/core/src/panicking.rs:114:9
   |         inside `core::panicking::const_panic_fmt` at /checkout/library/core/src/panicking.rs:114:9
   |
  ::: /checkout/src/test/ui/consts/control-flow/assert.rs:5:15
   |
LL | const _: () = assert!(false);
   |               -------------- inside `_` at /checkout/src/test/ui/consts/control-flow/assert.rs:5:15
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.


------------------------------------------


---- [ui] ui/mir/issue-67639-normalization-ice.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/mir/issue-67639-normalization-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-67639-normalization-ice" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=4" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/mir/issue-67639-normalization-ice/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: internal compiler error: compiler/rustc_traits/src/normalize_erasing_regions.rs:54:32: could not fully normalize `fn() -> alloc::raw_vec::RawVec<<<I as Foo>::Item as Bar>::Item2> {alloc::raw_vec::RawVec::<<<I as Foo>::Item as Bar>::Item2>::new}`
thread 'rustc' panicked at 'Box<dyn Any>', compiler/rustc_errors/src/lib.rs:1169:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace

note: the compiler unexpectedly panicked. this is a bug.
note: the compiler unexpectedly panicked. this is a bug.

note: we would appreciate a bug report: https://github.com/rust-lang/rust/issues/new?labels=C-bug%2C+I-ICE%2C+T-compiler&template=ice.md

note: rustc 1.59.0-nightly (7959e68a6 2021-11-29) running on x86_64-unknown-linux-gnu

note: compiler flags: -Z threads=1 -Z ui-testing -Z deduplicate-diagnostics=no -Z emit-future-incompat-report -Z mir-opt-level=4 -C codegen-units=1 -C prefer-dynamic -C rpath -C debuginfo=0
query stack during panic:
query stack during panic:
#0 [normalize_generic_arg_after_erasing_regions] normalizing `fn() -> alloc::raw_vec::RawVec<<<I as Foo>::Item as Bar>::Item2> {alloc::raw_vec::RawVec::<<<I as Foo>::Item as Bar>::Item2>::new}`
#1 [fn_abi_of_instance] computing call ABI of `alloc::raw_vec::RawVec::<<<I as Foo>::Item as Bar>::Item2>::new`
error: aborting due to previous error


------------------------------------------
---
test result: FAILED. 12299 passed; 7 failed; 111 ignored; 0 measured; 0 filtered out; finished in 138.05s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:55
