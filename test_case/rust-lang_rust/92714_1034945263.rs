plain
..............................i..................................................................... 6900/12644
.................................i...................................................F.............. 7000/12644
.................................................................................................... 7100/12644
.......................................................ii................................ii......... 7200/12644
...............................FF.....F.F.......i................................................... 7300/12644
...............................................................F.................................... 7400/12644
.................................................................................................... 7600/12644
........................................................................ii................i....i..ii 7700/12644
.................................................................................................... 7800/12644
.................................................................................................... 7900/12644
.................................................................................................... 7900/12644
.................................................................................................... 8000/12644
.................................................................................................... 8100/12644
.........................i.ii.............................................................ii........ 8200/12644
..........................................................................iiii...................... 8300/12644
...............................................................................................i.... 8400/12644
...................................i..........F.FFF................................................. 8500/12644
........................................................i........................................... 8700/12644
.................................................................................................... 8800/12644
...................................i................................................................ 8900/12644
.................................................................................................... 9000/12644
---
.................................................................................................... 9700/12644
.................................................................................................... 9800/12644
.................................................................................................... 9900/12644
.................................................................................................... 10000/12644
.........................F....F..................................................................... 10100/12644
.......................................................................................i............ 10300/12644
...................................................................................iiiiii.i..iiiiii. 10400/12644
i................................................................................................... 10500/12644
.................................................................................................... 10600/12644
.................................................................................................... 10600/12644
.................................................................................................... 10700/12644
.................................................................................................... 10800/12644
.................................................................................................... 10900/12644
.................................................................................................... 11000/12644
.................................................................................................... 11100/12644
.................................................................................................... 11200/12644
........................................................ii............F...FFF....FFF...iFF...FFFFFFF 11300/12644
FF...F...F........F.....................................F........................................... 11400/12644
.................................................................................................... 11600/12644
.................................................................................................... 11700/12644
.................................................................................................... 11800/12644
.........................................................i.......................................... 11900/12644
---
---- [ui] ui/attributes/duplicated-attributes.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/attributes/duplicated-attributes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/duplicated-attributes" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/attributes/duplicated-attributes/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | fn f() {}
   | ^^^^^^^^^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | fn f() {}
   | ^^^^^^^^^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | fn f2() {}
   | ^^^^^^^^^^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | fn f3() {}
   | ^^^^^^^^^^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | fn f4(_: &mut Bencher) {}


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | fn f4(_: &mut Bencher) {}

warning: duplicated attribute
  --> /checkout/src/test/ui/attributes/duplicated-attributes.rs:12:1
   |
---

warning: duplicated attribute
  --> /checkout/src/test/ui/attributes/duplicated-attributes.rs:29:1
   |
LL | #[bench]

warning: duplicated attribute
  --> /checkout/src/test/ui/attributes/duplicated-attributes.rs:34:1
   |
---

---- [ui] ui/borrowck/move-error-snippets.rs stdout ----
diff of stderr:

- error[E0507]: cannot move out of static item `D`
-   --> $DIR/move-error-snippets-ext.rs:5:17
+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
3    |
- LL |         let a = $c;
-    |                 ^^
-    |                 |
-    |                 |
-    |                 move occurs because `D` has type `A`, which does not implement the `Copy` trait
-    |                 help: consider borrowing here: `&$c`
+ LL | /         fn fff() {
+ LL | |             static D: A = A;
+ LL | |             aaa!(D);
+ LL | |         }
+ ...
+ ...
+ LL |   sss!();
9    |
-   ::: $DIR/move-error-snippets.rs:21:1
-    |
-    |
- LL | sss!();
-    |
-    |
-    = note: this error originates in the macro `aaa` (in Nightly builds, run with -Z macro-backtrace for more info)
+    = note: this error originates in the macro `sss` (in Nightly builds, run with -Z macro-backtrace for more info)
17 error: aborting due to previous error
18 

- For more information about this error, try `rustc --explain E0507`.
---
To only update this specific test, also pass `--test-args borrowck/move-error-snippets.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/move-error-snippets.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/move-error-snippets/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | /         fn fff() {
LL | |             static D: A = A;
LL | |             aaa!(D);         //~ ERROR cannot move
LL | |         }
...
...
LL |   sss!();
   |
   |
   = note: this error originates in the macro `sss` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error


------------------------------------------
------------------------------------------


---- [ui] ui/custom_test_frameworks/mismatch.rs stdout ----
diff of stderr:

+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
+    |
+    |
+ LL | fn wrong_kind(){}
+ 
+ 
1 error[E0277]: the trait bound `TestDescAndFn: Testable` is not satisfied
3    |

9    = note: required for the cast to the object type `dyn Testable`
10    = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z macro-backtrace for more info)
---
To only update this specific test, also pass `--test-args custom_test_frameworks/mismatch.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/custom_test_frameworks/mismatch.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/custom_test_frameworks/mismatch/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | fn wrong_kind(){}


error[E0277]: the trait bound `TestDescAndFn: Testable` is not satisfied
   |
LL | #[test]
   | ------- in this procedural macro expansion
   | ------- in this procedural macro expansion
LL | fn wrong_kind(){}
   | ^^^^^^^^^^^^^^^^^ the trait `Testable` is not implemented for `TestDescAndFn`
   = note: required for the cast to the object type `dyn Testable`
   = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 2 previous errors
---


---- [ui] ui/generator/smoke.rs#default stdout ----

error in revision `default`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/smoke.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "default" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.default/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.default/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn simple() {
LL | |     let mut foo = || {
LL | |         if false {
LL | |             yield;
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn return_capture() {
LL | / fn return_capture() {
LL | |     let a = String::from("foo");
LL | |     let mut foo = || {
LL | |         if false {
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn simple_yield() {
LL | |     let mut foo = || {
LL | |         yield;
LL | |     };
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn yield_capture() {
LL | |     let b = String::from("foo");
LL | |     let mut foo = || {
LL | |         yield b;
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn simple_yield_value() {
LL | |     let mut foo = || {
LL | |         yield String::from("bar");
LL | |         return String::from("foo")
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn return_after_yield() {
LL | |     let a = String::from("foo");
LL | |     let mut foo = || {
LL | |         yield;
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn send_and_sync() {
LL | |     assert_send_sync(|| {
LL | |         yield
LL | |     });
...  |
LL | |     fn assert_send_sync<T: Send + Sync>(_: T) {}
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn send_over_threads() {
LL | |     let mut foo = || { yield };
LL | |     thread::spawn(move || {
LL | |         match Pin::new(&mut foo).resume(()) {
...  |
LL | |     }).join().unwrap();
LL | | }

error: aborting due to 8 previous errors



------------------------------------------


---- [ui] ui/generator/smoke.rs#nomiropt stdout ----

error in revision `nomiropt`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generator/smoke.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "nomiropt" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.nomiropt/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "mir-opt-level=0" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generator/smoke.nomiropt/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn simple() {
LL | |     let mut foo = || {
LL | |         if false {
LL | |             yield;
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn return_capture() {
LL | / fn return_capture() {
LL | |     let a = String::from("foo");
LL | |     let mut foo = || {
LL | |         if false {
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn simple_yield() {
LL | |     let mut foo = || {
LL | |         yield;
LL | |     };
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn yield_capture() {
LL | |     let b = String::from("foo");
LL | |     let mut foo = || {
LL | |         yield b;
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn simple_yield_value() {
LL | |     let mut foo = || {
LL | |         yield String::from("bar");
LL | |         return String::from("foo")
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn return_after_yield() {
LL | |     let a = String::from("foo");
LL | |     let mut foo = || {
LL | |         yield;
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn send_and_sync() {
LL | |     assert_send_sync(|| {
LL | |         yield
LL | |     });
...  |
LL | |     fn assert_send_sync<T: Send + Sync>(_: T) {}
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn send_over_threads() {
LL | |     let mut foo = || { yield };
LL | |     thread::spawn(move || {
LL | |         match Pin::new(&mut foo).resume(()) {
...  |
LL | |     }).join().unwrap();
LL | | }

error: aborting due to 8 previous errors



------------------------------------------


---- [ui] ui/issues/issue-12997-2.rs stdout ----
diff of stderr:

+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
+    |
+    |
+ LL | fn bar(x: isize) { }
+ 
1 error[E0308]: mismatched types
2   --> $DIR/issue-12997-2.rs:8:1
3    |
3    |

8    |
9    = note: this error originates in the attribute macro `bench` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
12 
13 For more information about this error, try `rustc --explain E0308`.
---
To only update this specific test, also pass `--test-args issues/issue-12997-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-12997-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-12997-2/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | fn bar(x: isize) { }

error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-12997-2.rs:8:1
   |
   |
LL | #[bench]
   | -------- in this procedural macro expansion
LL | fn bar(x: isize) { }
   | ^^^^^^^^^^^^^^^^^^^^ expected `isize`, found `&mut Bencher`
   |
   = note: this error originates in the attribute macro `bench` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0308`.


------------------------------------------


---- [ui] ui/issues/issue-16597.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-16597.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-16597/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL |     pub fn test(){}
   |     ^^^^^^^^^^^^^^^

---
---- [ui] ui/issues/issue-46519.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-46519.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46519/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-O" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-46519/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test() {
LL | / fn test() {
LL | |     FontLanguageOverride::system_font(SystemFont::new());
LL | | }

error: aborting due to previous error



------------------------------------------


---- [ui] ui/issues/issue-52557.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52557.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52557/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52557/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | /     fn foo() {
LL | |         local_name(); // ensure the local name still works
LL | |     }
LL | |     }
   | |_____^

error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL |     fn local_name() {}
   |     ^^^^^^^^^^^^^^^^^^

---

---- [ui] ui/lint/test-inner-fn.rs stdout ----
diff of stderr:

- error: cannot test inner items
-   --> $DIR/test-inner-fn.rs:5:5
+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
3    |
- LL |     #[test]
-    |     ^^^^^^^
+ LL | / fn foo() {
+ LL | / fn foo() {
+ LL | |     #[test]
+ LL | |     fn bar() {}
+ LL | |     bar();
+ LL | | }
+ 
+ 
+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
6    |
6    |
-    = note: requested on the command line with `-D unnameable-test-items`
-    = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z macro-backtrace for more info)
+ LL |     fn bar() {}
9 
- error: cannot test inner items
-   --> $DIR/test-inner-fn.rs:13:9
-   --> $DIR/test-inner-fn.rs:13:9
+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
12    |
- LL |         #[test]
-    |         ^^^^^^^
+ LL | /     fn foo() {
+ LL | /     fn foo() {
+ LL | |         #[test]
+ LL | |         fn bar() {}
+ LL | |         bar();
+ LL | |     }
+ 
+ 
+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
15    |
-    = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z macro-backtrace for more info)
+ LL |         fn bar() {}
+    |         ^^^^^^^^^^^
---
To only update this specific test, also pass `--test-args lint/test-inner-fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lint/test-inner-fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-D" "unnameable_test_items" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lint/test-inner-fn/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn foo() {
LL | / fn foo() {
LL | |     #[test] //~ ERROR cannot test inner items [unnameable_test_items]
LL | |     fn bar() {}
LL | |     bar();
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL |     fn bar() {}
   |     ^^^^^^^^^^^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | /     fn foo() {
LL | /     fn foo() {
LL | |         #[test] //~ ERROR cannot test inner items [unnameable_test_items]
LL | |         fn bar() {}
LL | |         bar();
LL | |     }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL |         fn bar() {}
   |         ^^^^^^^^^^^

---


---- [ui] ui/macros/macro-comma-behavior-rpass.rs#core stdout ----

error in revision `core`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "core" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.core/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.core/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn assert_1arg() {
LL | |     assert!(false,);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn debug_assert_1arg() {
LL | |     debug_assert!(false,);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn to_format_or_not_to_format() {
LL | |     // ("{}" is the easiest string to test because if this gets
LL | |     // sent to format_args!, it'll simply fail to compile.
LL | |     // "{{}}" is an example of an input that could compile and
...  |
LL | |     // writeln!(&mut stdout, "{}",); // see check-fail
LL | | }

error: aborting due to 3 previous errors



------------------------------------------


---- [ui] ui/macros/macro-comma-behavior-rpass.rs#std stdout ----

error in revision `std`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-behavior-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "std" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.std/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-behavior-rpass.std/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn assert_1arg() {
LL | |     assert!(false,);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn debug_assert_1arg() {
LL | |     debug_assert!(false,);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn writeln_1arg() {
LL | |     use fmt::Write;
LL | |
LL | |     let mut s = String::new();
LL | |     writeln!(&mut s,).unwrap();
LL | |     assert_eq!(&s, "\n");
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn to_format_or_not_to_format() {
LL | |     // ("{}" is the easiest string to test because if this gets
LL | |     // sent to format_args!, it'll simply fail to compile.
LL | |     // "{{}}" is an example of an input that could compile and
...  |
LL | |     // writeln!(&mut stdout, "{}",); // see check-fail
LL | | }

error: aborting due to 4 previous errors



------------------------------------------


---- [ui] ui/macros/macro-comma-support-rpass.rs#core stdout ----

error in revision `core`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "core" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.core/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn assert() {
LL | |     assert!(true);
LL | |     assert!(true,);
LL | |     assert!(true, "hello");
...  |
LL | |     assert!(true, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn assert_eq() {
LL | |     assert_eq!(1, 1);
LL | |     assert_eq!(1, 1,);
LL | |     assert_eq!(1, 1, "hello");
...  |
LL | |     assert_eq!(1, 1, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn assert_ne() {
LL | |     assert_ne!(1, 2);
LL | |     assert_ne!(1, 2,);
LL | |     assert_ne!(1, 2, "hello");
...  |
LL | |     assert_ne!(1, 2, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn cfg() {
LL | |     let _ = cfg!(pants);
LL | |     let _ = cfg!(pants,);
LL | |     let _ = cfg!(pants = "pants");
...  |
LL | |     let _ = cfg!(all(pants,),);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn column() {
LL | |     let _ = column!();
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn concat() {
LL | / fn concat() {
LL | |     let _ = concat!();
LL | |     let _ = concat!("hello");
LL | |     let _ = concat!("hello",);
LL | |     let _ = concat!("hello", " world");
LL | |     let _ = concat!("hello", " world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn concat_idents() {
LL | |     fn foo() {}
LL | |     fn foobar() {}
LL | |     fn foobar() {}
LL | |
...  |
LL | |     concat_idents!(foo, bar,)();
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn debug_assert() {
LL | |     debug_assert!(true);
LL | |     debug_assert!(true, );
LL | |     debug_assert!(true, "hello");
...  |
LL | |     debug_assert!(true, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn debug_assert_eq() {
LL | |     debug_assert_eq!(1, 1);
LL | |     debug_assert_eq!(1, 1,);
LL | |     debug_assert_eq!(1, 1, "hello");
...  |
LL | |     debug_assert_eq!(1, 1, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn debug_assert_ne() {
LL | |     debug_assert_ne!(1, 2);
LL | |     debug_assert_ne!(1, 2,);
LL | |     debug_assert_ne!(1, 2, "hello");
...  |
LL | |     debug_assert_ne!(1, 2, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn env() {
LL | |     let _ = env!("PATH");
LL | |     let _ = env!("PATH",);
LL | |     let _ = env!("PATH", "not found");
LL | |     let _ = env!("PATH", "not found",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn file() {
LL | / fn file() {
LL | |     let _ = file!();
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn format_args() {
LL | |     let _ = format_args!("hello");
LL | |     let _ = format_args!("hello");
LL | |     let _ = format_args!("hello",);
LL | |     let _ = format_args!("hello {}", "world");
LL | |     let _ = format_args!("hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn include() {
LL | |     let _ = include!("auxiliary/macro-comma-support.rs");
LL | |     let _ = include!("auxiliary/macro-comma-support.rs",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn include_bytes() {
LL | / fn include_bytes() {
LL | |     let _ = include_bytes!("auxiliary/macro-comma-support.rs");
LL | |     let _ = include_bytes!("auxiliary/macro-comma-support.rs",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn include_str() {
LL | / fn include_str() {
LL | |     let _ = include_str!("auxiliary/macro-comma-support.rs");
LL | |     let _ = include_str!("auxiliary/macro-comma-support.rs",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn line() {
LL | / fn line() {
LL | |     let _ = line!();
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn module_path() {
LL | |     let _ = module_path!();
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn option_env() {
LL | |     let _ = option_env!("PATH");
LL | |     let _ = option_env!("PATH",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn panic() {
LL | / fn panic() {
LL | |     // prevent 'unreachable code' warnings
LL | |     let falsum = || false;
LL | |
...  |
LL | |     if falsum() { panic!("hello {}", "world",); }
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn try() {
LL | |     fn inner() -> Result<(), ()> {
LL | |     fn inner() -> Result<(), ()> {
LL | |         try!(Ok(()));
LL | |         try!(Ok(()),);
LL | |     inner().unwrap();
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn unimplemented() {
LL | / fn unimplemented() {
LL | |     // prevent 'unreachable code' warnings
LL | |     let falsum = || false;
LL | |
...  |
LL | |     if falsum() { unimplemented!("hello {}", "world",); }
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn unreachable() {
LL | / fn unreachable() {
LL | |     // prevent 'unreachable code' warnings
LL | |     let falsum = || false;
LL | |
...  |
LL | |     if falsum() { unreachable!("hello {}", "world",); }
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | /         fn $fname() {
LL | |             struct Struct;
LL | |             struct Struct;
LL | |             impl fmt::Display for Struct {
LL | |                 fn fmt(&self, $f: &mut fmt::Formatter) -> fmt::Result {
...  |
LL | |             assert!(true, "{}", Struct);
LL | |         }
...
...
LL | / test_with_formatter! {
LL | |     #[test]
LL | |     fn write(f: &mut fmt::Formatter) {
LL | |         let _ = write!(f, "hello");
LL | |     }
LL | | }
   | |_- in this macro invocation
   |
   |
   = note: this error originates in the macro `test_with_formatter` (in Nightly builds, run with -Z macro-backtrace for more info)

error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | /         fn $fname() {
LL | |             struct Struct;
LL | |             struct Struct;
LL | |             impl fmt::Display for Struct {
LL | |                 fn fmt(&self, $f: &mut fmt::Formatter) -> fmt::Result {
...  |
LL | |             assert!(true, "{}", Struct);
LL | |         }
...
...
LL | / test_with_formatter! {
LL | |     #[test]
LL | |     fn writeln(f: &mut fmt::Formatter) {
LL | |         let _ = writeln!(f);
LL | |     }
LL | | }
   | |_- in this macro invocation
   |
---


---- [ui] ui/macros/macro-comma-support-rpass.rs#std stdout ----

error in revision `std`: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/macro-comma-support-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--cfg" "std" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-C" "debug_assertions=yes" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/macro-comma-support-rpass.std/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn assert() {
LL | |     assert!(true);
LL | |     assert!(true,);
LL | |     assert!(true, "hello");
...  |
LL | |     assert!(true, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn assert_eq() {
LL | |     assert_eq!(1, 1);
LL | |     assert_eq!(1, 1,);
LL | |     assert_eq!(1, 1, "hello");
...  |
LL | |     assert_eq!(1, 1, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn assert_ne() {
LL | |     assert_ne!(1, 2);
LL | |     assert_ne!(1, 2,);
LL | |     assert_ne!(1, 2, "hello");
...  |
LL | |     assert_ne!(1, 2, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn cfg() {
LL | |     let _ = cfg!(pants);
LL | |     let _ = cfg!(pants,);
LL | |     let _ = cfg!(pants = "pants");
...  |
LL | |     let _ = cfg!(all(pants,),);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn column() {
LL | |     let _ = column!();
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn concat() {
LL | / fn concat() {
LL | |     let _ = concat!();
LL | |     let _ = concat!("hello");
LL | |     let _ = concat!("hello",);
LL | |     let _ = concat!("hello", " world");
LL | |     let _ = concat!("hello", " world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn concat_idents() {
LL | |     fn foo() {}
LL | |     fn foobar() {}
LL | |     fn foobar() {}
LL | |
...  |
LL | |     concat_idents!(foo, bar,)();
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn debug_assert() {
LL | |     debug_assert!(true);
LL | |     debug_assert!(true, );
LL | |     debug_assert!(true, "hello");
...  |
LL | |     debug_assert!(true, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn debug_assert_eq() {
LL | |     debug_assert_eq!(1, 1);
LL | |     debug_assert_eq!(1, 1,);
LL | |     debug_assert_eq!(1, 1, "hello");
...  |
LL | |     debug_assert_eq!(1, 1, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn debug_assert_ne() {
LL | |     debug_assert_ne!(1, 2);
LL | |     debug_assert_ne!(1, 2,);
LL | |     debug_assert_ne!(1, 2, "hello");
...  |
LL | |     debug_assert_ne!(1, 2, "hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn env() {
LL | |     let _ = env!("PATH");
LL | |     let _ = env!("PATH",);
LL | |     let _ = env!("PATH", "not found");
LL | |     let _ = env!("PATH", "not found",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn eprint() {
LL | |     eprint!("hello");
LL | |     eprint!("hello");
LL | |     eprint!("hello",);
LL | |     eprint!("hello {}", "world");
LL | |     eprint!("hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn eprintln() {
LL | |     eprintln!();
LL | |     eprintln!("hello");
LL | |     eprintln!("hello",);
LL | |     eprintln!("hello {}", "world");
LL | |     eprintln!("hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn file() {
LL | / fn file() {
LL | |     let _ = file!();
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn format() {
LL | |     let _ = format!("hello");
LL | |     let _ = format!("hello");
LL | |     let _ = format!("hello",);
LL | |     let _ = format!("hello {}", "world");
LL | |     let _ = format!("hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn format_args() {
LL | |     let _ = format_args!("hello");
LL | |     let _ = format_args!("hello");
LL | |     let _ = format_args!("hello",);
LL | |     let _ = format_args!("hello {}", "world");
LL | |     let _ = format_args!("hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn include() {
LL | |     let _ = include!("auxiliary/macro-comma-support.rs");
LL | |     let _ = include!("auxiliary/macro-comma-support.rs",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn include_bytes() {
LL | / fn include_bytes() {
LL | |     let _ = include_bytes!("auxiliary/macro-comma-support.rs");
LL | |     let _ = include_bytes!("auxiliary/macro-comma-support.rs",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn include_str() {
LL | / fn include_str() {
LL | |     let _ = include_str!("auxiliary/macro-comma-support.rs");
LL | |     let _ = include_str!("auxiliary/macro-comma-support.rs",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn line() {
LL | / fn line() {
LL | |     let _ = line!();
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn module_path() {
LL | |     let _ = module_path!();
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn option_env() {
LL | |     let _ = option_env!("PATH");
LL | |     let _ = option_env!("PATH",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn panic() {
LL | / fn panic() {
LL | |     // prevent 'unreachable code' warnings
LL | |     let falsum = || false;
LL | |
...  |
LL | |     if falsum() { panic!("hello {}", "world",); }
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn print() {
LL | |     print!("hello");
LL | |     print!("hello");
LL | |     print!("hello",);
LL | |     print!("hello {}", "world");
LL | |     print!("hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn println() {
LL | |     println!();
LL | |     println!("hello");
LL | |     println!("hello",);
LL | |     println!("hello {}", "world");
LL | |     println!("hello {}", "world",);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn thread_local() {
LL | / fn thread_local() {
LL | |     // this has an optional trailing *semicolon*
LL | |     thread_local! {
LL | |         #[allow(unused)] pub static A: () = ()
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn try() {
LL | |     fn inner() -> Result<(), ()> {
LL | |     fn inner() -> Result<(), ()> {
LL | |         try!(Ok(()));
LL | |         try!(Ok(()),);
LL | |     inner().unwrap();
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn unimplemented() {
LL | / fn unimplemented() {
LL | |     // prevent 'unreachable code' warnings
LL | |     let falsum = || false;
LL | |
...  |
LL | |     if falsum() { unimplemented!("hello {}", "world",); }
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn unreachable() {
LL | / fn unreachable() {
LL | |     // prevent 'unreachable code' warnings
LL | |     let falsum = || false;
LL | |
...  |
LL | |     if falsum() { unreachable!("hello {}", "world",); }
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn vec() {
LL | |     let _: Vec<()> = vec![];
LL | |     let _ = vec![0];
LL | |     let _ = vec![0,];
LL | |     let _ = vec![0, 1];
LL | |     let _ = vec![0, 1,];
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | /         fn $fname() {
LL | |             struct Struct;
LL | |             struct Struct;
LL | |             impl fmt::Display for Struct {
LL | |                 fn fmt(&self, $f: &mut fmt::Formatter) -> fmt::Result {
...  |
LL | |             assert!(true, "{}", Struct);
LL | |         }
...
...
LL | / test_with_formatter! {
LL | |     #[test]
LL | |     fn write(f: &mut fmt::Formatter) {
LL | |         let _ = write!(f, "hello");
LL | |     }
LL | | }
   | |_- in this macro invocation
   |
   |
   = note: this error originates in the macro `test_with_formatter` (in Nightly builds, run with -Z macro-backtrace for more info)

error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | /         fn $fname() {
LL | |             struct Struct;
LL | |             struct Struct;
LL | |             impl fmt::Display for Struct {
LL | |                 fn fmt(&self, $f: &mut fmt::Formatter) -> fmt::Result {
...  |
LL | |             assert!(true, "{}", Struct);
LL | |         }
...
...
LL | / test_with_formatter! {
LL | |     #[test]
LL | |     fn writeln(f: &mut fmt::Formatter) {
LL | |         let _ = writeln!(f);
LL | |     }
LL | | }
   | |_- in this macro invocation
   |
---
---- [ui] ui/macros/stringify.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/stringify.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/stringify/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/stringify/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test_block() {
LL | / fn test_block() {
LL | |     assert_eq!(stringify_block!({}), "{}");
LL | |     assert_eq!(stringify_block!({ true }), "{ true }");
LL | |     assert_eq!(stringify_block!({ return }), "{ return }");
LL | |     );
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test_expr() {
LL | |     // ExprKind::Box
LL | |     // ExprKind::Box
LL | |     assert_eq!(stringify_expr!(box expr), "box expr");
LL | |
...  |
LL | |     assert_eq!(stringify_expr!(yield true), "yield true");
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn test_item() {
LL | |     // ItemKind::ExternCrate
LL | |     assert_eq!(
LL | |         stringify_item!(
LL | |     );
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn test_meta() {
LL | |     assert_eq!(stringify_meta!(k), "k");
LL | |     assert_eq!(stringify_meta!(k = "v"), "k = \"v\"");
LL | |     assert_eq!(stringify_meta!(list(k1, k2 = "v")), "list(k1, k2 = \"v\")");
LL | |     assert_eq!(stringify_meta!(serde::k), "serde::k");
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test_pat() {
LL | / fn test_pat() {
LL | |     // PatKind::Wild
LL | |     assert_eq!(stringify_pat!(_), "_");
LL | |
...  |
LL | |     assert_eq!(stringify_pat!(mac! { ... }), "mac! { ... }");
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test_path() {
LL | / fn test_path() {
LL | |     assert_eq!(stringify_path!(thing), "thing");
LL | |     assert_eq!(stringify_path!(m::thing), "m::thing");
LL | |     assert_eq!(stringify_path!(self::thing), "self::thing");
...  |
LL | |     assert_eq!(stringify_path!(Self() -> ()), "Self() -> ()");
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn test_stmt() {
LL | |     // StmtKind::Local
LL | |     assert_eq!(stringify_stmt!(let _), "let _;");
LL | |     assert_eq!(stringify_stmt!(let x = true), "let x = true;");
...  |
LL | |     assert_eq!(stringify_stmt!(mac! { ... }), "mac! { ... }");
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn test_ty() {
LL | |     // TyKind::Slice
LL | |     assert_eq!(stringify_ty!([T]), "[T]");
LL | |
...  |
LL | |     assert_eq!(stringify_ty!(mac! { ... }), "mac! { ... }");
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn test_vis() {
LL | |     // VisibilityKind::Public
LL | |     assert_eq!(stringify_vis!(pub), "pub ");
LL | |
...  |
LL | |     assert_eq!(stringify_inherited_vis!(struct), "");
LL | | }

error: aborting due to 9 previous errors



------------------------------------------


---- [ui] ui/panics/test-panic.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/test-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/test-panic/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/test-panic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test_foo() {
LL | |     panic!()
LL | | }
---
---- [ui] ui/panics/test-should-fail-bad-message.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/test-should-fail-bad-message.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/test-should-fail-bad-message/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/test-should-fail-bad-message/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test_foo() {
LL | / fn test_foo() {
LL | |     panic!("blah")
LL | | }

error: aborting due to previous error



------------------------------------------


---- [ui] ui/panics/test-should-panic-no-message.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/test-should-panic-no-message.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/test-should-panic-no-message/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/test-should-panic-no-message/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / pub fn test_explicit() {
LL | |     panic!()
LL | | }
---
---- [ui] ui/panics/test-should-panic-bad-message.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/panics/test-should-panic-bad-message.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/test-should-panic-bad-message/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/panics/test-should-panic-bad-message/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / pub fn test_bar() {
LL | |     panic!("bar")
LL | | }
---
---- [ui] ui/proc-macro/derive-test.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/derive-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-test/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/derive-test/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / pub fn test_derive() {
LL | |     assert!(true);
LL | | }
---
---- [ui] ui/process/core-run-destroy.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/process/core-run-destroy.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/core-run-destroy/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/process/core-run-destroy/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test_destroy_once() {
LL | / fn test_destroy_once() {
LL | |     let mut p = sleeper();
LL | |     t!(p.kill());
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn test_destroy_twice() {
LL | |     let mut p = sleeper();
LL | |     t!(p.kill()); // this shouldn't crash...
LL | |     let _ = p.kill(); // ...and nor should this (and nor should the destructor)
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn test_destroy_actually_kills() {
LL | |     let cmd = if cfg!(windows) {
LL | |         "cmd"
LL | |     } else if cfg!(target_os = "android") {
...  |
LL | |     tx.send(());
LL | | }

error: aborting due to 3 previous errors



------------------------------------------


---- [ui] ui/rfc-1937-termination-trait/termination-trait-in-test.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-in-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-in-test/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn is_a_num() -> Result<(), ParseIntError> {
LL | |     let _: u32 = "22".parse()?;
LL | |     Ok(())
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn test_a_positive_bench(_: &mut Bencher) -> Result<(), ParseIntError> {
LL | |     Ok(())
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn test_a_neg_bench(_: &mut Bencher) -> Result<(), ParseIntError> {
LL | |     let _: u32 = "abc".parse()?;
LL | |     Ok(())
LL | | }

error: aborting due to 3 previous errors



------------------------------------------


---- [ui] ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs stdout ----
diff of stderr:

+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
+    |
+    |
+ LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> {
+ LL | |     "0".parse()
+ LL | | }
+ 
+ 
1 error[E0277]: `main` has invalid return type `Result<f32, ParseFloatError>`
3    |


16    |                              ^^^^^^^^^^^ required by this bound in `assert_test_result`
17    = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z macro-backtrace for more info)
- error: aborting due to previous error
+ error: aborting due to 2 previous errors
20 
21 For more information about this error, try `rustc --explain E0277`.
---
To only update this specific test, also pass `--test-args rfc-1937-termination-trait/termination-trait-test-wrong-type.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-1937-termination-trait/termination-trait-test-wrong-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> { //~ ERROR
LL | |     "0".parse()
LL | | }


error[E0277]: `main` has invalid return type `Result<f32, ParseFloatError>`
   |
LL |   #[test]
   |   ------- in this procedural macro expansion
   |   ------- in this procedural macro expansion
LL | / fn can_parse_zero_as_f32() -> Result<f32, ParseFloatError> { //~ ERROR
LL | |     "0".parse()
LL | | }
   | |_^ `main` can only return types that implement `Termination`
   |
   = help: the trait `Termination` is not implemented for `Result<f32, ParseFloatError>`
note: required by a bound in `assert_test_result`
   |
   |
LL | pub fn assert_test_result<T: Termination>(result: T) {
   |                              ^^^^^^^^^^^ required by this bound in `assert_test_result`
   = note: this error originates in the attribute macro `test` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0277`.


------------------------------------------


---- [ui] ui/test-attrs/decl-macro-test.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/decl-macro-test.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/decl-macro-test" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/decl-macro-test/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL |     fn test() {}
   |     ^^^^^^^^^^^^
...
...
LL | create_test!();
   |
   = note: this error originates in the macro `create_test` (in Nightly builds, run with -Z macro-backtrace for more info)


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL |     fn test() {}
   |     ^^^^^^^^^^^^
...
...
LL | create_test!();
   |
   = note: this error originates in the macro `create_test` (in Nightly builds, run with -Z macro-backtrace for more info)


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL |         fn test() {}
   |         ^^^^^^^^^^^^
...
...
LL | create_module_test!();
   |
   = note: this error originates in the macro `create_module_test` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to 3 previous errors
---

---- [ui] ui/test-attrs/inaccessible-test-modules.rs stdout ----
diff of stderr:

+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
+   --> $DIR/inaccessible-test-modules.rs:9:1
+    |
+ LL | fn baz() {}
+ 
1 error[E0432]: unresolved import `main`
2   --> $DIR/inaccessible-test-modules.rs:5:5
3    |
---
To only update this specific test, also pass `--test-args test-attrs/inaccessible-test-modules.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/inaccessible-test-modules.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/inaccessible-test-modules" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/inaccessible-test-modules/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | fn baz() {}

error[E0432]: unresolved import `main`
  --> /checkout/src/test/ui/test-attrs/inaccessible-test-modules.rs:5:5
   |
   |
LL | use main as x; //~ ERROR unresolved import `main`
   |     |
   |     no `main` in the root
   |     help: a similar name exists in the module: `main`


error[E0432]: unresolved import `test`
  --> /checkout/src/test/ui/test-attrs/inaccessible-test-modules.rs:6:5
   |
LL | use test as y; //~ ERROR unresolved import `test`
   |     |
   |     no `test` in the root
   |     help: a similar name exists in the module: `test`

---
---- [ui] ui/test-attrs/issue-20823.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/issue-20823.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/issue-20823/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/issue-20823/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | pub fn foo() {}
   | ^^^^^^^^^^^^^^^

---
---- [ui] ui/test-attrs/issue-36768.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/issue-36768.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/issue-36768/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/issue-36768/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | #[test] fn foo() {}


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | #[test] fn core() {}

error: aborting due to 2 previous errors


---
16 LL | |     }
17    | |_____^
18 
- error: aborting due to 2 previous errors
+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
+    |
+ LL | / fn test() {
+ LL | / fn test() {
+ LL | |     let _ = A::new();
+ LL | | }
+ 
+ error: aborting due to 3 previous errors
20 
21 
---
To only update this specific test, also pass `--test-args test-attrs/test-attr-non-associated-functions.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-attr-non-associated-functions.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-attr-non-associated-functions" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-attr-non-associated-functions/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: `#[test]` attribute is only allowed on non associated functions
   |
   |
LL | /     fn new() -> A {
LL | |         //~^ ERROR `#[test]` attribute is only allowed on non associated functions
LL | |         A {}
LL | |     }


error: `#[test]` attribute is only allowed on non associated functions
   |
   |
LL | /     fn recovery_witness() -> A {
LL | |         //~^ ERROR `#[test]` attribute is only allowed on non associated functions
LL | |         A {}
LL | |     }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test() {
LL | / fn test() {
LL | |     let _ = A::new();
LL | | }

error: aborting due to 3 previous errors



------------------------------------------


---- [ui] ui/test-attrs/issue-53675-a-test-called-panic.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/issue-53675-a-test-called-panic.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/issue-53675-a-test-called-panic" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/issue-53675-a-test-called-panic/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | /     fn panic() {
LL | |         assert!(true)
LL | |     }
LL | |     }
   | |_____^

error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | /     fn panic() {
LL | |         assert!(true);
LL | |     }
LL | |     }
   | |_____^

error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | /     fn panic() {
LL | /     fn panic() {
LL | |         panic!("in expr")
LL | |     }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | /     fn panic() {
LL | /     fn panic() {
LL | |         panic!("in stmt");
LL | |     }

error: aborting due to 4 previous errors



------------------------------------------


---- [ui] ui/test-attrs/run-unexported-tests.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/run-unexported-tests.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/run-unexported-tests/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/run-unexported-tests/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | /     fn unexported() {
LL | /     fn unexported() {
LL | |         panic!("ran an unexported test");
LL | |     }

error: aborting due to previous error



------------------------------------------


---- [ui] ui/test-attrs/test-filter-multiple.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-filter-multiple.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-filter-multiple/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-filter-multiple/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | fn test1() {}
   | ^^^^^^^^^^^^^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | fn test2() {}
   | ^^^^^^^^^^^^^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test3() {
LL | / fn test3() {
LL | |     panic!("this should not run");
LL | | }

error: aborting due to 3 previous errors



------------------------------------------


---- [ui] ui/test-attrs/test-fn-signature-verification-for-explicit-return-type.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-fn-signature-verification-for-explicit-return-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-fn-signature-verification-for-explicit-return-type/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-fn-signature-verification-for-explicit-return-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | pub fn bench_explicit_return_type(_: &mut ::test::Bencher) -> () {}


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | pub fn test_explicit_return_type() -> () {}
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

---
---- [ui] ui/test-attrs/test-panic-while-printing.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-panic-while-printing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-while-printing/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-while-printing/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn main() {
LL | |     let result = std::panic::catch_unwind(|| {
LL | |     let result = std::panic::catch_unwind(|| {
LL | |         let a = A(vec![]);
LL | |         eprintln!("{}", a);
LL | |     });
LL | |     assert!(result.is_err());
LL | | }

error: aborting due to previous error



------------------------------------------


---- [ui] ui/test-attrs/test-should-fail-good-message.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-should-fail-good-message.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-fail-good-message/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / pub fn test_foo() {
LL | / pub fn test_foo() {
LL | |     panic!("foo bar")
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / pub fn test_foo_dynamic() {
LL | |     panic!("{} bar", "foo")
LL | | }

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/test-attrs/test-panic-abort-nocapture.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-panic-abort-nocapture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-nocapture/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-Cpanic=abort" "-Zpanic_abort_tests" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-nocapture/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_works() {
LL | |     println!("about to succeed");
LL | |     assert_eq!(1 + 1, 2);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_panics() {
LL | |     println!("about to panic");
LL | |     assert_eq!(1 + 1, 4);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_fails() {
LL | |     println!("about to fail");
LL | |     assert_eq!(1 + 1, 4);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_writes_to_stdio() {
LL | |     println!("hello, world");
LL | |     writeln!(std::io::stdout(), "testing123").unwrap();
LL | |     writeln!(std::io::stderr(), "testing321").unwrap();
LL | | }

error: aborting due to 4 previous errors



------------------------------------------


---- [ui] ui/test-attrs/test-should-panic-attr.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-should-panic-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-panic-attr" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-should-panic-attr/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test1() {
LL | |     panic!();
LL | | }
LL | | }
   | |_^

error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test2() {
LL | |     panic!();
LL | | }
LL | | }
   | |_^

warning: argument must be of the form: `expected = "error message"`
   |
   |
LL | #[should_panic(expected)]
   |
   = note: errors in this attribute were erroneously allowed and will become a hard error in a future release


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test3() {
LL | |     panic!();
LL | | }
LL | | }
   | |_^

warning: argument must be of the form: `expected = "error message"`
   |
   |
LL | #[should_panic(expect)]
   |
   = note: errors in this attribute were erroneously allowed and will become a hard error in a future release


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test4() {
LL | |     panic!();
LL | | }
LL | | }
   | |_^

warning: argument must be of the form: `expected = "error message"`
   |
   |
LL | #[should_panic(expected(foo, bar))]
   |
   = note: errors in this attribute were erroneously allowed and will become a hard error in a future release


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test5() {
LL | |     panic!();
LL | | }
LL | | }
   | |_^

warning: argument must be of the form: `expected = "error message"`
   |
   |
LL | #[should_panic(expected = "foo", bar)]
   |
   = note: errors in this attribute were erroneously allowed and will become a hard error in a future release

error: aborting due to 5 previous errors; 4 warnings emitted
---

---- [ui] ui/test-attrs/test-panic-abort-disabled.rs stdout ----
diff of stderr:

+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
+    |
+    |
+ LL | / fn it_works() {
+ LL | |     assert_eq!(1 + 1, 2);
+ LL | | }
+ 
+ 
+ error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
+    |
+    |
+ LL | / fn it_panics() {
+ LL | |     assert_eq!(1 + 1, 4);
+ LL | | }
+ 
+ 
1 error: building tests with panic=abort is not supported without `-Zpanic_abort_tests`
- error: aborting due to previous error
+ error: aborting due to 3 previous errors
4 
5 
---
To only update this specific test, also pass `--test-args test-attrs/test-panic-abort-disabled.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-panic-abort-disabled.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-disabled" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-Cpanic=abort" "-Zpanic-abort-tests=no" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort-disabled/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_works() {
LL | |     assert_eq!(1 + 1, 2);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_panics() {
LL | |     assert_eq!(1 + 1, 4);
LL | | }


error: building tests with panic=abort is not supported without `-Zpanic_abort_tests`
error: aborting due to 3 previous errors


------------------------------------------
------------------------------------------


---- [ui] ui/test-attrs/test-panic-abort.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-panic-abort.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-Cpanic=abort" "-Zpanic_abort_tests" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-panic-abort/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_works() {
LL | |     assert_eq!(1 + 1, 2);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_panics() {
LL | |     assert_eq!(1 + 1, 4);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_fails() {
LL | |     println!("hello, world");
LL | |     writeln!(std::io::stdout(), "testing123").unwrap();
LL | |     writeln!(std::io::stderr(), "testing321").unwrap();
LL | |     assert_eq!(1 + 1, 5);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_exits() {
LL | |     std::process::exit(123);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn no_residual_environment() {
LL | |     for (key, _) in env::vars() {
LL | |         // Look for keys like __RUST_TEST_INVOKE.
LL | |         if key.contains("TEST_INVOKE") {
LL | |     }
LL | | }
   | |_^

---
---- [ui] ui/test-attrs/test-cant-be-shadowed.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-cant-be-shadowed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-cant-be-shadowed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-cant-be-shadowed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | fn foo(){}
   | ^^^^^^^^^^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | fn bar() {}
   | ^^^^^^^^^^^

---
---- [ui] ui/test-attrs/test-passed.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-passed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-passed/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-passed/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_works() {
LL | |     assert_eq!(1 + 1, 2);
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn it_works_too() {
LL | |     assert_eq!(1 * 0, 0);
LL | | }

error: aborting due to 2 previous errors



------------------------------------------


---- [ui] ui/test-attrs/test-thread-capture.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-thread-capture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-capture/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-capture/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn thready_pass() {
LL | |     println!("fee");
LL | |     std::thread::spawn(|| {
LL | |         println!("fie");
...  |
LL | |     println!("fum");
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn thready_fail() {
LL | |     println!("fee");
LL | |     std::thread::spawn(|| {
LL | |         println!("fie");
LL | |     panic!();
LL | | }
   | |_^

---
---- [ui] ui/test-attrs/test-type.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-type.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-type/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-type/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn test_ok() {
LL | |     let _a = true;
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test_panic() {
LL | |     panic!();
LL | | }
---
---- [ui] ui/test-attrs/test-thread-nocapture.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/test-attrs/test-thread-nocapture.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-nocapture/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/test-attrs/test-thread-nocapture/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn thready_pass() {
LL | |     println!("fee");
LL | |     std::thread::spawn(|| {
LL | |         println!("fie");
...  |
LL | |     println!("fum");
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn thready_fail() {
LL | |     println!("fee");
LL | |     std::thread::spawn(|| {
LL | |         println!("fie");
LL | |     panic!();
LL | | }
   | |_^

---
---- [ui] ui/threads-sendsync/mpsc_stress.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/mpsc_stress.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/mpsc_stress/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/mpsc_stress/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn shared_close_sender_does_not_lose_messages() {
LL | |     for _ in 0..10000 {
LL | |         shared_close_sender_does_not_lose_messages_iter();
LL | |     }
LL | | }


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | / fn concurrent_recv_timeout_and_upgrade() {
LL | |     // FIXME: fix and enable
LL | |     if true { return }
LL | |
LL | |     }
LL | | }
   | |_^


error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn concurrent_writes() {
LL | / fn concurrent_writes() {
LL | |     for _ in 0..100 {
LL | |         concurrent_writes_iter();
LL | |     }
LL | | }

error: aborting due to 3 previous errors



------------------------------------------


---- [ui] ui/threads-sendsync/test-tasks-invalid-value.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/threads-sendsync/test-tasks-invalid-value.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/test-tasks-invalid-value/a" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/threads-sendsync/test-tasks-invalid-value/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
   |
LL | fn do_nothing() {}

error: aborting due to previous error



------------------------------------------


---- [ui] ui/unused-crate-deps/test-use-ok.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/unused-crate-deps/test-use-ok.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/test-use-ok" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "--test" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/test-use-ok/auxiliary" "--extern" "bar=/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/unused-crate-deps/test-use-ok/auxiliary/libbar.so"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error: ignore a test should use `#[ignore]` or `#[ignore = "message"]`
   |
LL | / fn test_bar() {
LL | / fn test_bar() {
LL | |     assert_eq!(bar::BAR, "bar");
LL | | }

error: aborting due to previous error


