plain
.................................................................................................... 2000/12505
..............................................................................i..................... 2100/12505
.................................................................................................... 2200/12505
.................................................................................................... 2300/12505
......F..F.F........................................................................................ 2400/12505
.................................................................................................... 2600/12505
.................................................................................................... 2700/12505
.................................................................................................... 2800/12505
...............................i.................................................................... 2900/12505
---
..................................................................ii................................ 3800/12505
...........................................................................................i........ 3900/12505
.................................................................................................... 4000/12505
.................................................................................................... 4100/12505
.............................................F..F................................................... 4200/12505
.................................................................................................... 4400/12505
.................................................................................................... 4500/12505
.................................................................................................... 4600/12505
.................................................................................................... 4700/12505
---

---- [ui] ui/consts/const-eval/const_panic_libcore_bin.rs stdout ----
diff of stderr:

12 LL | const Y: () = unreachable!();
13    |               ^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_libcore_bin.rs:11:15
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
16 
17 error[E0080]: evaluation of constant value failed
17 error[E0080]: evaluation of constant value failed
18   --> $DIR/const_panic_libcore_bin.rs:14:15

20 LL | const X: () = unimplemented!();
21    |               ^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic_libcore_bin.rs:14:15
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
24 
25 error: aborting due to 3 previous errors
---
To only update this specific test, also pass `--test-args consts/const-eval/const_panic_libcore_bin.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_bin" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_libcore_bin/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs:8:15
   |
LL | const Z: () = panic!("cheese");
   |               ^^^^^^^^^^^^^^^^ the evaluated program panicked at 'cheese', /checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs:8:15
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs:11:15
  --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs:11:15
   |
LL | const Y: () = unreachable!();
   |               ^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs:11:15
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs:14:15
  --> /checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs:14:15
   |
LL | const X: () = unimplemented!();
   |               ^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic_libcore_bin.rs:14:15
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 3 previous errors

---

---- [ui] ui/consts/const-eval/const_panic.rs stdout ----
diff of stderr:

20 LL | const Y: () = std::unreachable!();
21    |               ^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic.rs:12:15
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
24 
25 error[E0080]: evaluation of constant value failed
25 error[E0080]: evaluation of constant value failed
26   --> $DIR/const_panic.rs:15:15

28 LL | const X: () = std::unimplemented!();
29    |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic.rs:15:15
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
32 
33 error[E0080]: evaluation of constant value failed
33 error[E0080]: evaluation of constant value failed
34   --> $DIR/const_panic.rs:18:15

68 LL | const Y_CORE: () = core::unreachable!();
69    |                    ^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic.rs:30:20
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
72 
73 error[E0080]: evaluation of constant value failed
73 error[E0080]: evaluation of constant value failed
74   --> $DIR/const_panic.rs:33:20

76 LL | const X_CORE: () = core::unimplemented!();
77    |                    ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic.rs:33:20
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
80 
81 error[E0080]: evaluation of constant value failed
---
To only update this specific test, also pass `--test-args consts/const-eval/const_panic.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic/auxiliary"
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
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:12:15
   |
LL | const Y: () = std::unreachable!();
   |               ^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic.rs:12:15
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:15:15
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:15:15
   |
LL | const X: () = std::unimplemented!();
   |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic.rs:15:15
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:18:15
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:18:15
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
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:24:20
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:24:20
   |
LL | const Z_CORE: () = core::panic!("cheese");
   |                    ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'cheese', /checkout/src/test/ui/consts/const-eval/const_panic.rs:24:20
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:27:21
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:27:21
   |
LL | const Z2_CORE: () = core::panic!();
   |
   = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:30:20
   |
LL | const Y_CORE: () = core::unreachable!();
   |                    ^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic.rs:30:20
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:33:20
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:33:20
   |
LL | const X_CORE: () = core::unimplemented!();
   |                    ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic.rs:33:20
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:36:20
  --> /checkout/src/test/ui/consts/const-eval/const_panic.rs:36:20
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

---- [ui] ui/consts/const-eval/const_panic_2021.rs stdout ----
diff of stderr:

20 LL | const C: () = std::unreachable!();
21    |               ^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_2021.rs:12:15
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
24 
25 error[E0080]: evaluation of constant value failed
25 error[E0080]: evaluation of constant value failed
26   --> $DIR/const_panic_2021.rs:15:15

28 LL | const D: () = std::unimplemented!();
29    |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic_2021.rs:15:15
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
32 
33 error[E0080]: evaluation of constant value failed
33 error[E0080]: evaluation of constant value failed
34   --> $DIR/const_panic_2021.rs:18:15

60 LL | const C_CORE: () = core::unreachable!();
61    |                    ^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', $DIR/const_panic_2021.rs:27:20
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
64 
65 error[E0080]: evaluation of constant value failed
65 error[E0080]: evaluation of constant value failed
66   --> $DIR/const_panic_2021.rs:30:20

68 LL | const D_CORE: () = core::unimplemented!();
69    |                    ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', $DIR/const_panic_2021.rs:30:20
-    = note: this error originates in the macro `$crate::panic::panic_2015` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)
72 
73 error[E0080]: evaluation of constant value failed
73 error[E0080]: evaluation of constant value failed
74   --> $DIR/const_panic_2021.rs:33:20


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021/const_panic_2021.stderr
To only update this specific test, also pass `--test-args consts/const-eval/const_panic_2021.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const_panic_2021.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const_panic_2021/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:6:15
   |
LL | const A: () = std::panic!("blåhaj");
   |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'blåhaj', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:6:15
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:9:15
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:9:15
   |
LL | const B: () = std::panic!();
   |
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:12:15
   |
LL | const C: () = std::unreachable!();
   |               ^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:12:15
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:15:15
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:15:15
   |
LL | const D: () = std::unimplemented!();
   |               ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:15:15
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:18:15
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:18:15
   |
LL | const E: () = std::panic!("{}", MSG);
   |               ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:18:15
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:21:20
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:21:20
   |
LL | const A_CORE: () = core::panic!("shark");
   |                    ^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'shark', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:21:20
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:24:20
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:24:20
   |
LL | const B_CORE: () = core::panic!();
   |
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:27:20
   |
LL | const C_CORE: () = core::unreachable!();
   |                    ^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'internal error: entered unreachable code', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:27:20
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:30:20
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:30:20
   |
LL | const D_CORE: () = core::unimplemented!();
   |                    ^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'not implemented', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:30:20
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0080]: evaluation of constant value failed
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:33:20
  --> /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:33:20
   |
LL | const E_CORE: () = core::panic!("{}", MSG);
   |                    ^^^^^^^^^^^^^^^^^^^^^^^ the evaluated program panicked at 'hello', /checkout/src/test/ui/consts/const-eval/const_panic_2021.rs:33:20
   = note: this error originates in the macro `$crate::panic::panic_2021` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 10 previous errors

---

---- [ui] ui/generic-associated-types/issue-87258_a.rs stdout ----
diff of stderr:

4 LL |     fn foo<'a>() -> Self::FooFuture<'a> {
6    |
6    |
-    = note: hidden type `Struct<'_>` captures lifetime '_#7r
+    = note: hidden type `Struct<'_>` captures lifetime '_#38r
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_a/issue-87258_a.stderr
To only update this specific test, also pass `--test-args generic-associated-types/issue-87258_a.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-87258_a.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_a/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
  --> /checkout/src/test/ui/generic-associated-types/issue-87258_a.rs:19:21
   |
LL |     fn foo<'a>() -> Self::FooFuture<'a> { //~ ERROR
   |
   |
   = note: hidden type `Struct<'_>` captures lifetime '_#38r
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.


------------------------------------------


---- [ui] ui/generic-associated-types/issue-87258_b.rs stdout ----
diff of stderr:

4 LL |     fn foo<'a>() -> Self::FooFuture<'a> {
6    |
6    |
-    = note: hidden type `Struct<'_>` captures lifetime '_#7r
+    = note: hidden type `Struct<'_>` captures lifetime '_#38r
9 error: aborting due to previous error
10 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_b/issue-87258_b.stderr
To only update this specific test, also pass `--test-args generic-associated-types/issue-87258_b.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/issue-87258_b.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_b" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/issue-87258_b/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0700]: hidden type for `impl Trait` captures lifetime that does not appear in bounds
   |
   |
LL |     fn foo<'a>() -> Self::FooFuture<'a> { //~ ERROR
   |
   |
   = note: hidden type `Struct<'_>` captures lifetime '_#38r
error: aborting due to previous error

For more information about this error, try `rustc --explain E0700`.

---

36                                                                 lit
37                                                             } else {
38                                                                 {
-                                                                     ::core::panicking::panic("internal error: entered unreachable code")
+                                                                     ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code"],
+                                                                                                                                 &match ()
+                                                                                                                                      _args
+                                                                                                                                      =>
+                                                                                                                                      [],
+                                                                                                                                  }))
---
To only update this specific test, also pass `--test-args proc-macro/quote-debug.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/quote-debug.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/quote-debug" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=expanded" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/quote-debug/auxiliary"
------------------------------------------
#![feature(prelude_import)]
#![no_std]
// check-pass
// check-pass
// force-host
// no-prefer-dynamic
// compile-flags: -Z unpretty=expanded
// This file is not actually used as a proc-macro - instead,
// This file is not actually used as a proc-macro - instead,
// it's just used to show the output of the `quote!` macro

#![feature(proc_macro_quote)]
#![crate_type = "proc-macro"]
#[prelude_import]
use ::std::prelude::rust_2015::*;
extern crate std;

extern crate proc_macro;


fn main() {
    [crate::TokenStream::from(crate::TokenTree::Ident(crate::Ident::new("let",
                                                                        crate::Span::recover_proc_macro_span(0)))),
     crate::TokenStream::from(crate::TokenTree::Ident(crate::Ident::new("hello",
                                                                        crate::Span::recover_proc_macro_span(1)))),
     crate::TokenStream::from(crate::TokenTree::Punct(crate::Punct::new('\u{3d}',
                                                                        crate::Spacing::Alone))),
     crate::TokenStream::from(crate::TokenTree::Literal({
                                                            let mut iter =
                                                                "\"world\"".parse::<crate::TokenStream>().unwrap().into_iter();
                                                            if let (Some(crate::TokenTree::Literal(mut lit)),
                                                                    None) =
                                                                   (iter.next(),
                                                                    iter.next())
                                                               {
                                                                lit.set_span(crate::Span::recover_proc_macro_span(2));
                                                                lit
                                                                {
                                                                {
                                                                    ::core::panicking::panic_fmt(::core::fmt::Arguments::new_v1(&["internal error: entered unreachable code"],
                                                                                                                                &match ()
                                                                                                                                     _args
                                                                                                                                     =>
                                                                                                                                     [],
                                                                                                                                 }))
                                                                                                                                 }))
                                                                }
                                                            }
                                                        })),
     crate::TokenStream::from(crate::TokenTree::Punct(crate::Punct::new('\u{3b}',
                                                                        crate::Spacing::Alone)))].iter().cloned().collect::<crate::TokenStream>()
const _: () =
    {
        extern crate proc_macro;
        #[rustc_proc_macro_decls]
        #[rustc_proc_macro_decls]
        #[allow(deprecated)]
        static _DECLS: &[proc_macro::bridge::client::ProcMacro] = &[];

------------------------------------------
stderr:
------------------------------------------
---
test result: FAILED. 12380 passed; 6 failed; 119 ignored; 0 measured; 0 filtered out; finished in 169.02s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-12/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "12.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine extensions filecheck frontendopenacc frontendopenmp fuzzmutate globalisel hellonew hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interfacestub interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcjit orcshared orctargetprocess passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:14:44
