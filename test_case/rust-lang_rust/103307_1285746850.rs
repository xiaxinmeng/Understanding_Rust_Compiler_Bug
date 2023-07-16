plain
.......i................................................................................ 968/13689
........................................................................................ 1056/13689
.............................FF......................................................... 1144/13689
........................................................................................ 1232/13689
............................F...F..............F........................................ 1320/13689
...............................................................i........................ 1496/13689
............................................F........................................... 1584/13689
........................................................................................ 1672/13689
........................................................................................ 1760/13689
---
........................................................................................ 2288/13689
........................................................................................ 2376/13689
........................................................................................ 2464/13689
........................................................................................ 2552/13689
........F................................F...................F.......................... 2640/13689
.........FF..F........F...............F..........................................F.F.F.F 2728/13689
.F......................F...........F...F............................................... 2816/13689
................................F....................................................... 2904/13689
.........................F.............................................................. 2992/13689
....................F.........................FF....F.............F.................i... 3080/13689
........................................................................................ 3256/13689
........................................................................................ 3344/13689
...........................iiiii........................................................ 3432/13689
........................................................................................ 3520/13689
---
..............................................................................i......... 11000/13689
........................................................................................ 11088/13689
iiiiii.i..iiiiiiiii.i................................................................... 11176/13689
........................................................................................ 11264/13689
...............................................................F.F...........F.......... 11352/13689
.........................................F......F....................................... 11440/13689
........................................................................................ 11528/13689
..F.....................F.....F....F.................................................... 11616/13689
........................................................................................ 11792/13689
........................................................................................ 11880/13689
........................................................................................ 11968/13689
........................................................................................ 12056/13689
---

---- [ui] src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.rs stdout ----
diff of stderr:

4 LL |     let x = defer(&vec!["Goodbye", "world!"]);
5    |                    ^^^^^^^^^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
-    |                    creates a temporary which is freed while still in use
+    |                    creates a temporary value which is freed while still in use
8 LL |     x.x[0];
9    |     ------ borrow later used here
---
To only update this specific test, also pass `--test-args borrowck/borrowck-borrowed-uniq-rvalue-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue-2.rs:20:20
   |
   |
LL |     let x = defer(&vec!["Goodbye", "world!"]); //~ ERROR temporary value dropped while borrowed
   |                    ^^^^^^^^^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                    creates a temporary value which is freed while still in use
LL |     x.x[0];
   |     ------ borrow later used here
   |
---

---- [ui] src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue.rs stdout ----
diff of stderr:

4 LL |     buggy_map.insert(42, &*Box::new(1));
5    |                            ^^^^^^^^^^^ - temporary value is freed at the end of this statement
-    |                            creates a temporary which is freed while still in use
+    |                            creates a temporary value which is freed while still in use
8 ...
8 ...
9 LL |     buggy_map.insert(43, &*tmp);


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrowed-uniq-rvalue/borrowck-borrowed-uniq-rvalue.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrowed-uniq-rvalue/borrowck-borrowed-uniq-rvalue.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/borrowck-borrowed-uniq-rvalue.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrowed-uniq-rvalue" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/borrowck-borrowed-uniq-rvalue/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/borrowck-borrowed-uniq-rvalue.rs:8:28
   |
   |
LL |     buggy_map.insert(42, &*Box::new(1)); //~ ERROR temporary value dropped while borrowed
   |                            ^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                            creates a temporary value which is freed while still in use
...
...
LL |     buggy_map.insert(43, &*tmp);
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~     let binding = Box::new(1);
LL ~     let binding = Box::new(1);
LL ~     buggy_map.insert(42, &*binding); //~ ERROR temporary value dropped while borrowed

error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
For more information about this error, try `rustc --explain E0716`.
------------------------------------------


---- [ui] src/test/ui/borrowck/issue-11493.rs stdout ----
diff of stderr:

4 LL |     let y = x.as_ref().unwrap_or(&id(5));
5    |                                   ^^^^^ - temporary value is freed at the end of this statement
-    |                                   creates a temporary which is freed while still in use
+    |                                   creates a temporary value which is freed while still in use
8 LL |     let _ = &y;
9    |             -- borrow later used here
---
To only update this specific test, also pass `--test-args borrowck/issue-11493.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-11493.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-11493" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-11493/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/issue-11493.rs:6:35
   |
   |
LL |     let y = x.as_ref().unwrap_or(&id(5));  //~ ERROR
   |                                   ^^^^^ - temporary value is freed at the end of this statement
   |                                   creates a temporary value which is freed while still in use
LL |     let _ = &y;
   |             -- borrow later used here
   |
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~     let binding = id(5);
LL ~     let y = x.as_ref().unwrap_or(&binding);  //~ ERROR

error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
For more information about this error, try `rustc --explain E0716`.
------------------------------------------


---- [ui] src/test/ui/borrowck/issue-17545.rs stdout ----
diff of stderr:

5    |              -- lifetime `'a` defined here
6 LL | /     bar.call((
7 LL | |         &id(()),
-    | |          ^^^^^^ creates a temporary which is freed while still in use
+    | |          ^^^^^^ creates a temporary value which is freed while still in use
9 LL | |     ));
11    | |______|


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-17545/issue-17545.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args borrowck/issue-17545.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-17545.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-17545" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-17545/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/issue-17545.rs:7:10
   |
   |
LL |   pub fn foo<'a, F: Fn(&'a ())>(bar: F) {
   |              -- lifetime `'a` defined here
LL | /     bar.call((
LL | |         &id(()), //~ ERROR temporary value dropped while borrowed
   | |          ^^^^^^ creates a temporary value which is freed while still in use
LL | |     ));
   | |______|
   | |______|
   |        argument requires that borrow lasts for `'a`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/borrowck/issue-36082.rs stdout ----
diff of stderr:

4 LL |     let val: &_ = x.borrow().0;
5    |                   ^^^^^^^^^^  - temporary value is freed at the end of this statement
-    |                   creates a temporary which is freed while still in use
+    |                   creates a temporary value which is freed while still in use
8 ...
9 LL |     println!("{}", val);
---
To only update this specific test, also pass `--test-args borrowck/issue-36082.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/borrowck/issue-36082.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-36082" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/borrowck/issue-36082/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/borrowck/issue-36082.rs:9:19
   |
   |
LL |     let val: &_ = x.borrow().0;
   |                   ^^^^^^^^^^  - temporary value is freed at the end of this statement
   |                   creates a temporary value which is freed while still in use
...
LL |     println!("{}", val);
   |                    --- borrow later used here
   |                    --- borrow later used here
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~     let binding = x.borrow();
LL ~     let val: &_ = binding.0;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
For more information about this error, try `rustc --explain E0716`.
------------------------------------------


---- [ui] src/test/ui/cleanup-rvalue-scopes-cf.rs stdout ----
diff of stderr:

4 LL |     let x1 = arg(&AddFlags(1));
5    |                   ^^^^^^^^^^^ - temporary value is freed at the end of this statement
-    |                   creates a temporary which is freed while still in use
+    |                   creates a temporary value which is freed while still in use
8 ...
9 LL |     (x1, x2, x3, x4, x5, x6, x7);
---
25 ...
26 LL |     (x1, x2, x3, x4, x5, x6, x7);
27    |          -- borrow later used here

38 LL |     let x3 = &*arg(&AddFlags(1));
39    |                     ^^^^^^^^^^^ - temporary value is freed at the end of this statement
-    |                     creates a temporary which is freed while still in use
+    |                     creates a temporary value which is freed while still in use
42 ...
43 LL |     (x1, x2, x3, x4, x5, x6, x7);
43 LL |     (x1, x2, x3, x4, x5, x6, x7);
44    |              -- borrow later used here

55 LL |     let ref x4 = *arg(&AddFlags(1));
56    |                        ^^^^^^^^^^^ - temporary value is freed at the end of this statement
-    |                        creates a temporary which is freed while still in use
+    |                        creates a temporary value which is freed while still in use
59 ...
60 LL |     (x1, x2, x3, x4, x5, x6, x7);
60 LL |     (x1, x2, x3, x4, x5, x6, x7);
61    |                  -- borrow later used here

72 LL |     let &ref x5 = arg(&AddFlags(1));
73    |                        ^^^^^^^^^^^ - temporary value is freed at the end of this statement
-    |                        creates a temporary which is freed while still in use
+    |                        creates a temporary value which is freed while still in use
76 ...
77 LL |     (x1, x2, x3, x4, x5, x6, x7);
---
93 ...
94 LL |     (x1, x2, x3, x4, x5, x6, x7);
95    |                          -- borrow later used here

106 LL |     let StackBox { f: x7 } = StackBox { f: AddFlags(1).get() };
108    |                                            |
-    |                                            creates a temporary which is freed while still in use
+    |                                            creates a temporary value which is freed while still in use
110 LL |
---
To only update this specific test, also pass `--test-args cleanup-rvalue-scopes-cf.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cleanup-rvalue-scopes-cf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cleanup-rvalue-scopes-cf" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cleanup-rvalue-scopes-cf/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/cleanup-rvalue-scopes-cf.rs:26:19
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     let x1 = arg(&AddFlags(1)); //~ ERROR temporary value dropped while borrowed
   |                   ^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                   creates a temporary value which is freed while still in use
...
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |      -- borrow later used here
   |      -- borrow later used here
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~     let binding = AddFlags(1);
LL ~     let x1 = arg(&binding); //~ ERROR temporary value dropped while borrowed

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/cleanup-rvalue-scopes-cf.rs:27:14
   |
   |
LL |     let x2 = AddFlags(1).get(); //~ ERROR temporary value dropped while borrowed
   |              |
   |              creates a temporary value which is freed while still in use
...
LL |     (x1, x2, x3, x4, x5, x6, x7);
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |          -- borrow later used here
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~     let binding = AddFlags(1);
LL ~     let x2 = binding.get(); //~ ERROR temporary value dropped while borrowed

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/cleanup-rvalue-scopes-cf.rs:28:21
   |
   |
LL |     let x3 = &*arg(&AddFlags(1)); //~ ERROR temporary value dropped while borrowed
   |                     ^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                     creates a temporary value which is freed while still in use
...
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |              -- borrow later used here
   |              -- borrow later used here
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~     let binding = AddFlags(1);
LL ~     let x3 = &*arg(&binding); //~ ERROR temporary value dropped while borrowed

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/cleanup-rvalue-scopes-cf.rs:29:24
   |
   |
LL |     let ref x4 = *arg(&AddFlags(1)); //~ ERROR temporary value dropped while borrowed
   |                        ^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                        creates a temporary value which is freed while still in use
...
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |                  -- borrow later used here
   |                  -- borrow later used here
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~     let binding = AddFlags(1);
LL ~     let ref x4 = *arg(&binding); //~ ERROR temporary value dropped while borrowed

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/cleanup-rvalue-scopes-cf.rs:30:24
   |
   |
LL |     let &ref x5 = arg(&AddFlags(1)); //~ ERROR temporary value dropped while borrowed
   |                        ^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                        creates a temporary value which is freed while still in use
...
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |                      -- borrow later used here
   |                      -- borrow later used here
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~     let binding = AddFlags(1);
LL ~     let &ref x5 = arg(&binding); //~ ERROR temporary value dropped while borrowed

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/cleanup-rvalue-scopes-cf.rs:31:14
   |
   |
LL |     let x6 = AddFlags(1).get(); //~ ERROR temporary value dropped while borrowed
   |              |
   |              creates a temporary value which is freed while still in use
...
LL |     (x1, x2, x3, x4, x5, x6, x7);
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |                          -- borrow later used here
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~     let binding = AddFlags(1);
LL ~     let x6 = binding.get(); //~ ERROR temporary value dropped while borrowed

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/cleanup-rvalue-scopes-cf.rs:32:44
   |
   |
LL |     let StackBox { f: x7 } = StackBox { f: AddFlags(1).get() };
   |                                            |
   |                                            creates a temporary value which is freed while still in use
   |                                            creates a temporary value which is freed while still in use
LL |     //~^ ERROR temporary value dropped while borrowed
LL |     (x1, x2, x3, x4, x5, x6, x7);
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~     let binding = AddFlags(1);
LL ~     let binding = AddFlags(1);
LL ~     let StackBox { f: x7 } = StackBox { f: binding.get() };

error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0716`.
For more information about this error, try `rustc --explain E0716`.
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/const-eval-intrinsic-promotion.rs stdout ----
diff of stderr:

4 LL |     let x: &'static usize =
5    |            -------------- type annotation requires that borrow lasts for `'static`
6 LL |         &std::intrinsics::size_of::<i32>();
-    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
8 LL | }
9    | - temporary value is freed at the end of this statement


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-intrinsic-promotion/const-eval-intrinsic-promotion.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-intrinsic-promotion/const-eval-intrinsic-promotion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/const-eval-intrinsic-promotion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/const-eval-intrinsic-promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-intrinsic-promotion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/const-eval-intrinsic-promotion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/const-eval-intrinsic-promotion.rs:5:10
   |
LL |     let x: &'static usize =
LL |     let x: &'static usize =
   |            -------------- type annotation requires that borrow lasts for `'static`
LL |         &std::intrinsics::size_of::<i32>(); //~ ERROR temporary value dropped while borrowed
   |          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs stdout ----
diff of stderr:

10   --> $DIR/dont_promote_unstable_const_fn.rs:17:28
11    |
12 LL |     let _: &'static u32 = &foo();
-    |            ------------    ^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^ creates a temporary value which is freed while still in use
14    |            |
15    |            type annotation requires that borrow lasts for `'static`
16 LL | }
20   --> $DIR/dont_promote_unstable_const_fn.rs:21:28
21    |
21    |
22 LL |     let _: &'static u32 = &meh();
-    |            ------------    ^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^ creates a temporary value which is freed while still in use
24    |            |
25    |            type annotation requires that borrow lasts for `'static`

31   --> $DIR/dont_promote_unstable_const_fn.rs:22:26
32    |
32    |
33 LL |     let x: &'static _ = &std::time::Duration::from_millis(42).subsec_millis();
-    |            ----------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ----------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
35    |            |
36    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn/dont_promote_unstable_const_fn.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn/dont_promote_unstable_const_fn.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/dont_promote_unstable_const_fn.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn/auxiliary"
stdout: none
--- stderr -------------------------------
error: `foo` is not yet stable as a const fn
   |
   |
LL | const fn bar() -> u32 { foo() } //~ ERROR `foo` is not yet stable as a const fn
   |
   |
   = help: add `#![feature(foo)]` to the crate attributes to enable
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs:17:28
   |
   |
LL |     let _: &'static u32 = &foo(); //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs:21:28
   |
   |
LL |     let _: &'static u32 = &meh(); //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn.rs:22:26
   |
LL |     let x: &'static _ = &std::time::Duration::from_millis(42).subsec_millis();
   |            ----------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate.rs stdout ----
diff of stderr:

2   --> $DIR/dont_promote_unstable_const_fn_cross_crate.rs:8:28
3    |
4 LL |     let _: &'static u32 = &foo();
-    |            ------------    ^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`
8 LL |     let _x: &'static u32 = &foo();
13   --> $DIR/dont_promote_unstable_const_fn_cross_crate.rs:9:29
14    |
14    |
15 LL |     let _x: &'static u32 = &foo();
-    |             ------------    ^^^^^ creates a temporary which is freed while still in use
+    |             ------------    ^^^^^ creates a temporary value which is freed while still in use
17    |             |
18    |             type annotation requires that borrow lasts for `'static`
19 LL | }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate/dont_promote_unstable_const_fn_cross_crate.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/dont_promote_unstable_const_fn_cross_crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate.rs:8:28
   |
   |
LL |     let _: &'static u32 = &foo(); //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     let _x: &'static u32 = &foo(); //~ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/dont_promote_unstable_const_fn_cross_crate.rs:9:29
   |
   |
LL |     let _x: &'static u32 = &foo(); //~ ERROR temporary value dropped while borrowed
   |             ------------    ^^^^^ creates a temporary value which is freed while still in use
   |             |
   |             type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/promoted_const_fn_fail.rs stdout ----
diff of stderr:

2   --> $DIR/promoted_const_fn_fail.rs:17:27
3    |
4 LL |     let x: &'static u8 = &(bar() + 1);
-    |            -----------    ^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            -----------    ^^^^^^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail/promoted_const_fn_fail.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail/promoted_const_fn_fail.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/promoted_const_fn_fail.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail.rs:17:27
   |
   |
LL |     let x: &'static u8 = &(bar() + 1); //~ ERROR temporary value dropped while borrowed
   |            -----------    ^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error: aborting due to previous error
---
diff of stderr:

2   --> $DIR/promoted_const_fn_fail_deny_const_err.rs:18:27
3    |
4 LL |     let x: &'static u8 = &(bar() + 1);
-    |            -----------    ^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            -----------    ^^^^^^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err/promoted_const_fn_fail_deny_const_err.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err/promoted_const_fn_fail_deny_const_err.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/promoted_const_fn_fail_deny_const_err.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/promoted_const_fn_fail_deny_const_err.rs:18:27
   |
   |
LL |     let x: &'static u8 = &(bar() + 1);
   |            -----------    ^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error: aborting due to previous error
---
diff of stderr:

2   --> $DIR/promoted_raw_ptr_ops.rs:2:29
3    |
4 LL |     let x: &'static bool = &(42 as *const i32 == 43 as *const i32);
-    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`

13   --> $DIR/promoted_raw_ptr_ops.rs:4:30
14    |
14    |
15 LL |     let y: &'static usize = &(&1 as *const i32 as usize + 1);
-    |            --------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            --------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
17    |            |
18    |            type annotation requires that borrow lasts for `'static`

24   --> $DIR/promoted_raw_ptr_ops.rs:6:28
25    |
25    |
26 LL |     let z: &'static i32 = &(unsafe { *(42 as *const i32) });
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
28    |            |
29    |            type annotation requires that borrow lasts for `'static`

35   --> $DIR/promoted_raw_ptr_ops.rs:8:29
36    |
36    |
37 LL |     let a: &'static bool = &(main as fn() == main as fn());
-    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
39    |            |
40    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_raw_ptr_ops/promoted_raw_ptr_ops.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_raw_ptr_ops/promoted_raw_ptr_ops.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/promoted_raw_ptr_ops.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/promoted_raw_ptr_ops.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_raw_ptr_ops" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/promoted_raw_ptr_ops/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/promoted_raw_ptr_ops.rs:2:29
   |
   |
LL |     let x: &'static bool = &(42 as *const i32 == 43 as *const i32);
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/promoted_raw_ptr_ops.rs:4:30
   |
LL |     let y: &'static usize = &(&1 as *const i32 as usize + 1);
   |            --------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/promoted_raw_ptr_ops.rs:6:28
   |
LL |     let z: &'static i32 = &(unsafe { *(42 as *const i32) });
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-eval/promoted_raw_ptr_ops.rs:8:29
   |
LL |     let a: &'static bool = &(main as fn() == main as fn());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/transmute-const-promotion.rs stdout ----
diff of stderr:

2   --> $DIR/transmute-const-promotion.rs:4:37
3    |
4 LL |     let x: &'static u32 = unsafe { &mem::transmute(3.0f32) };
-    |            ------------             ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------             ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const-promotion/transmute-const-promotion.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const-promotion/transmute-const-promotion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/transmute-const-promotion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/transmute-const-promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const-promotion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/transmute-const-promotion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/transmute-const-promotion.rs:4:37
   |
   |
LL |     let x: &'static u32 = unsafe { &mem::transmute(3.0f32) };
   |            ------------             ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-eval/union_promotion.rs stdout ----
diff of stderr:

7    | |            type annotation requires that borrow lasts for `'static`
8 LL | |         Foo { a: &1 }.b == Foo { a: &2 }.b
9 LL | |     };
-    | |_____^ creates a temporary which is freed while still in use
+    | |_____^ creates a temporary value which is freed while still in use
11 LL |   }
13 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union_promotion/union_promotion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/union_promotion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-eval/union_promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union_promotion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/union_promotion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-eval/union_promotion.rs:8:29
   |
   |
LL |       let x: &'static bool = &unsafe { //~ temporary value dropped while borrowed
   |  ____________-------------____^
   | |            |
   | |            type annotation requires that borrow lasts for `'static`
LL | |         Foo { a: &1 }.b == Foo { a: &2 }.b
LL | |     };
   | |_____^ creates a temporary value which is freed while still in use
LL |   }

error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
---
diff of stderr:

2   --> $DIR/const-int-conversion.rs:2:28
3    |
4 LL |     let x: &'static i32 = &(5_i32.reverse_bits());
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`

13   --> $DIR/const-int-conversion.rs:4:28
14    |
14    |
15 LL |     let y: &'static i32 = &(i32::from_be_bytes([0x12, 0x34, 0x56, 0x78]));
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
17    |            |
18    |            type annotation requires that borrow lasts for `'static`

24   --> $DIR/const-int-conversion.rs:6:28
25    |
25    |
26 LL |     let z: &'static i32 = &(i32::from_le_bytes([0x12, 0x34, 0x56, 0x78]));
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
28    |            |
29    |            type annotation requires that borrow lasts for `'static`

35   --> $DIR/const-int-conversion.rs:8:28
36    |
36    |
37 LL |     let a: &'static i32 = &(i32::from_be(i32::from_ne_bytes([0x80, 0, 0, 0])));
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
39    |            |
40    |            type annotation requires that borrow lasts for `'static`

46   --> $DIR/const-int-conversion.rs:10:29
47    |
47    |
48 LL |     let b: &'static [u8] = &(0x12_34_56_78_i32.to_be_bytes());
-    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
50    |            |
51    |            type annotation requires that borrow lasts for `'static`

57   --> $DIR/const-int-conversion.rs:12:29
58    |
58    |
59 LL |     let c: &'static [u8] = &(0x12_34_56_78_i32.to_le_bytes());
-    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
61    |            |
62    |            type annotation requires that borrow lasts for `'static`

68   --> $DIR/const-int-conversion.rs:14:29
69    |
69    |
70 LL |     let d: &'static [u8] = &(i32::MIN.to_be().to_ne_bytes());
-    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
72    |            |
73    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion/const-int-conversion.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion/const-int-conversion.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-int-conversion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-conversion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-conversion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:2:28
   |
   |
LL |     let x: &'static i32 = &(5_i32.reverse_bits());
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:4:28
   |
LL |     let y: &'static i32 = &(i32::from_be_bytes([0x12, 0x34, 0x56, 0x78]));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:6:28
   |
LL |     let z: &'static i32 = &(i32::from_le_bytes([0x12, 0x34, 0x56, 0x78]));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:8:28
   |
LL |     let a: &'static i32 = &(i32::from_be(i32::from_ne_bytes([0x80, 0, 0, 0])));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:10:29
   |
LL |     let b: &'static [u8] = &(0x12_34_56_78_i32.to_be_bytes());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:12:29
   |
LL |     let c: &'static [u8] = &(0x12_34_56_78_i32.to_le_bytes());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-conversion.rs:14:29
   |
LL |     let d: &'static [u8] = &(i32::MIN.to_be().to_ne_bytes());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |         //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-int-overflowing.rs stdout ----
diff of stderr:

2   --> $DIR/const-int-overflowing.rs:2:36
3    |
4 LL |     let x: &'static (i32, bool) = &(5_i32.overflowing_add(3));
-    |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`

13   --> $DIR/const-int-overflowing.rs:4:36
14    |
14    |
15 LL |     let y: &'static (i32, bool) = &(5_i32.overflowing_sub(3));
-    |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
17    |            |
18    |            type annotation requires that borrow lasts for `'static`

24   --> $DIR/const-int-overflowing.rs:6:36
25    |
25    |
26 LL |     let z: &'static (i32, bool) = &(5_i32.overflowing_mul(3));
-    |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
28    |            |
29    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing/const-int-overflowing.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing/const-int-overflowing.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-int-overflowing.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-overflowing.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-overflowing/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-int-overflowing.rs:2:36
   |
   |
LL |     let x: &'static (i32, bool) = &(5_i32.overflowing_add(3));
   |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-overflowing.rs:4:36
   |
LL |     let y: &'static (i32, bool) = &(5_i32.overflowing_sub(3));
   |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-overflowing.rs:6:36
   |
LL |     let z: &'static (i32, bool) = &(5_i32.overflowing_mul(3));
   |            --------------------    ^^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-int-rotate.rs stdout ----
diff of stderr:

2   --> $DIR/const-int-rotate.rs:2:28
3    |
4 LL |     let x: &'static i32 = &(5_i32.rotate_left(3));
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`

13   --> $DIR/const-int-rotate.rs:4:28
14    |
14    |
15 LL |     let y: &'static i32 = &(5_i32.rotate_right(3));
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
17    |            |
18    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate/const-int-rotate.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate/const-int-rotate.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-int-rotate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-rotate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-rotate/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-int-rotate.rs:2:28
   |
   |
LL |     let x: &'static i32 = &(5_i32.rotate_left(3));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-rotate.rs:4:28
   |
LL |     let y: &'static i32 = &(5_i32.rotate_right(3));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-int-sign.rs stdout ----
diff of stderr:

2   --> $DIR/const-int-sign.rs:2:29
3    |
4 LL |     let x: &'static bool = &(5_i32.is_negative());
-    |            -------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            -------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`

13   --> $DIR/const-int-sign.rs:4:29
14    |
14    |
15 LL |     let y: &'static bool = &(5_i32.is_positive());
-    |            -------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            -------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
17    |            |
18    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-sign/const-int-sign.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-sign/const-int-sign.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-int-sign.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-sign.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-sign" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-sign/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-int-sign.rs:2:29
   |
   |
LL |     let x: &'static bool = &(5_i32.is_negative());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-sign.rs:4:29
   |
LL |     let y: &'static bool = &(5_i32.is_positive());
   |            -------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-int-wrapping.rs stdout ----
diff of stderr:

2   --> $DIR/const-int-wrapping.rs:2:28
3    |
4 LL |     let x: &'static i32 = &(5_i32.wrapping_add(3));
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`

13   --> $DIR/const-int-wrapping.rs:4:28
14    |
14    |
15 LL |     let y: &'static i32 = &(5_i32.wrapping_sub(3));
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
17    |            |
18    |            type annotation requires that borrow lasts for `'static`

24   --> $DIR/const-int-wrapping.rs:6:28
25    |
25    |
26 LL |     let z: &'static i32 = &(5_i32.wrapping_mul(3));
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
28    |            |
29    |            type annotation requires that borrow lasts for `'static`

35   --> $DIR/const-int-wrapping.rs:8:28
36    |
36    |
37 LL |     let a: &'static i32 = &(5_i32.wrapping_shl(3));
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
39    |            |
40    |            type annotation requires that borrow lasts for `'static`

46   --> $DIR/const-int-wrapping.rs:10:28
47    |
47    |
48 LL |     let b: &'static i32 = &(5_i32.wrapping_shr(3));
-    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
50    |            |
51    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-wrapping/const-int-wrapping.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-wrapping/const-int-wrapping.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-int-wrapping.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-int-wrapping.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-wrapping" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-int-wrapping/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-int-wrapping.rs:2:28
   |
   |
LL |     let x: &'static i32 = &(5_i32.wrapping_add(3));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-wrapping.rs:4:28
   |
LL |     let y: &'static i32 = &(5_i32.wrapping_sub(3));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-wrapping.rs:6:28
   |
LL |     let z: &'static i32 = &(5_i32.wrapping_mul(3));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-wrapping.rs:8:28
   |
LL |     let a: &'static i32 = &(5_i32.wrapping_shl(3));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-int-wrapping.rs:10:28
   |
LL |     let b: &'static i32 = &(5_i32.wrapping_shr(3));
   |            ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 5 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
---
12    |                              |         | |
13    |                              |         | temporary value is freed at the end of this statement
-    |                              |         creates a temporary which is freed while still in use
+    |                              |         creates a temporary value which is freed while still in use
15    |                              using this value as a constant requires that borrow lasts for `'static`
17 error[E0716]: temporary value dropped while borrowed

21    |                              ------------^^-
22    |                              |           | |
22    |                              |           | |
23    |                              |           | temporary value is freed at the end of this statement
-    |                              |           creates a temporary which is freed while still in use
+    |                              |           creates a temporary value which is freed while still in use
25    |                              using this value as a constant requires that borrow lasts for `'static`
27 error[E0716]: temporary value dropped while borrowed

31    |                                  -------------------------------^^--
32    |                                  |                              |  |
32    |                                  |                              |  |
33    |                                  |                              |  temporary value is freed at the end of this statement
-    |                                  |                              creates a temporary which is freed while still in use
+    |                                  |                              creates a temporary value which is freed while still in use
35    |                                  using this value as a constant requires that borrow lasts for `'static`
37 error[E0716]: temporary value dropped while borrowed

41    |                                    -------------------------------^^--
42    |                                    |                              |  |
42    |                                    |                              |  |
43    |                                    |                              |  temporary value is freed at the end of this statement
-    |                                    |                              creates a temporary which is freed while still in use
+    |                                    |                              creates a temporary value which is freed while still in use
45    |                                    using this value as a static requires that borrow lasts for `'static`
47 error[E0716]: temporary value dropped while borrowed

51    |                                        -------------------------------^^--
52    |                                        |                              |  |
52    |                                        |                              |  |
53    |                                        |                              |  temporary value is freed at the end of this statement
-    |                                        |                              creates a temporary which is freed while still in use
+    |                                        |                              creates a temporary value which is freed while still in use
55    |                                        using this value as a static requires that borrow lasts for `'static`
57 error: aborting due to 6 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final/mut_ref_in_final.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-mut-refs/mut_ref_in_final.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-mut-refs/mut_ref_in_final/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0764]: mutable references are not allowed in the final value of constants
   |
   |
LL | const B: *mut i32 = &mut 4; //~ ERROR mutable references are not allowed

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:16:40
   |
   |
LL | const B3: Option<&mut i32> = Some(&mut 42); //~ ERROR temporary value dropped while borrowed
   |                              ----------^^-
   |                              |         | temporary value is freed at the end of this statement
   |                              |         creates a temporary value which is freed while still in use
   |                              |         creates a temporary value which is freed while still in use
   |                              using this value as a constant requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:19:42
   |
   |
LL | const B4: Option<&mut i32> = helper(&mut 42); //~ ERROR temporary value dropped while borrowed
   |                              ------------^^-
   |                              |           | temporary value is freed at the end of this statement
   |                              |           creates a temporary value which is freed while still in use
   |                              |           creates a temporary value which is freed while still in use
   |                              using this value as a constant requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:34:65
   |
   |
LL | const FOO: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                  -------------------------------^^--
   |                                  |                              |  temporary value is freed at the end of this statement
   |                                  |                              creates a temporary value which is freed while still in use
   |                                  |                              creates a temporary value which is freed while still in use
   |                                  using this value as a constant requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:37:67
   |
   |
LL | static FOO2: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                    -------------------------------^^--
   |                                    |                              |  temporary value is freed at the end of this statement
   |                                    |                              creates a temporary value which is freed while still in use
   |                                    |                              creates a temporary value which is freed while still in use
   |                                    using this value as a static requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-mut-refs/mut_ref_in_final.rs:40:71
   |
   |
LL | static mut FOO3: NotAMutex<&mut i32> = NotAMutex(UnsafeCell::new(&mut 42));
   |                                        -------------------------------^^--
   |                                        |                              |  temporary value is freed at the end of this statement
   |                                        |                              creates a temporary value which is freed while still in use
   |                                        |                              creates a temporary value which is freed while still in use
   |                                        using this value as a static requires that borrow lasts for `'static`
error: aborting due to 6 previous errors

Some errors have detailed explanations: E0716, E0764.
For more information about an error, try `rustc --explain E0716`.
---
diff of stderr:

2   --> $DIR/const-ptr-nonnull.rs:4:37
3    |
4 LL |     let x: &'static NonNull<u32> = &(NonNull::dangling());
-    |            ---------------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ---------------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`

13   --> $DIR/const-ptr-nonnull.rs:9:37
14    |
14    |
15 LL |     let x: &'static NonNull<u32> = &(non_null.cast());
-    |            ---------------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            ---------------------    ^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
17    |            |
18    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull/const-ptr-nonnull.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull/const-ptr-nonnull.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-ptr-nonnull.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-ptr-nonnull.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-nonnull/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-ptr-nonnull.rs:4:37
   |
   |
LL |     let x: &'static NonNull<u32> = &(NonNull::dangling());
   |            ---------------------    ^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/const-ptr-nonnull.rs:9:37
   |
LL |     let x: &'static NonNull<u32> = &(non_null.cast());
   |            ---------------------    ^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/const-ptr-unique.rs stdout ----
diff of stderr:

2   --> $DIR/const-ptr-unique.rs:8:33
3    |
4 LL |     let x: &'static *mut u32 = &(unique.as_ptr());
-    |            -----------------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |            -----------------    ^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique/const-ptr-unique.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique/const-ptr-unique.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-ptr-unique.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/const-ptr-unique.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-ptr-unique/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/const-ptr-unique.rs:8:33
   |
   |
LL |     let x: &'static *mut u32 = &(unique.as_ptr());
   |            -----------------    ^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     //~^ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
---
4 LL |     let x: &'static _ = &X;
-    |            ----------    ^ creates a temporary which is freed while still in use
+    |            ----------    ^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`

13   --> $DIR/interior-mutability.rs:41:26
14    |
14    |
15 LL |     let y: &'static _ = &Y;
-    |            ----------    ^ creates a temporary which is freed while still in use
+    |            ----------    ^ creates a temporary value which is freed while still in use
17    |            |
18    |            type annotation requires that borrow lasts for `'static`
19 LL |     let z: &'static _ = &Z;
24   --> $DIR/interior-mutability.rs:42:26
25    |
25    |
26 LL |     let z: &'static _ = &Z;
-    |            ----------    ^ creates a temporary which is freed while still in use
+    |            ----------    ^ creates a temporary value which is freed while still in use
28    |            |
29    |            type annotation requires that borrow lasts for `'static`
30 LL | }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability/interior-mutability.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/control-flow/interior-mutability.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/control-flow/interior-mutability.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/control-flow/interior-mutability/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/control-flow/interior-mutability.rs:40:26
   |
   |
LL |     let x: &'static _ = &X; //~ ERROR temporary value dropped while borrowed
   |            ----------    ^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/control-flow/interior-mutability.rs:41:26
   |
LL |     let y: &'static _ = &Y; //~ ERROR temporary value dropped while borrowed
   |            ----------    ^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     let z: &'static _ = &Z; //~ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/control-flow/interior-mutability.rs:42:26
   |
   |
LL |     let z: &'static _ = &Z; //~ ERROR temporary value dropped while borrowed
   |            ----------    ^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
---
6    |                                 |     |        |
7    |                                 |     |        temporary value is freed at the end of this statement
-    |                                 |     creates a temporary which is freed while still in use
+    |                                 |     creates a temporary value which is freed while still in use
9    |                                 using this value as a constant requires that borrow lasts for `'static`
11 error[E0716]: temporary value dropped while borrowed

15    |                                          ---------------^^^^^^^^^-
16    |                                          |              |        |
16    |                                          |              |        |
17    |                                          |              |        temporary value is freed at the end of this statement
-    |                                          |              creates a temporary which is freed while still in use
+    |                                          |              creates a temporary value which is freed while still in use
19    |                                          using this value as a constant requires that borrow lasts for `'static`
21 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224/issue-54224.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/issue-54224.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/issue-54224.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/issue-54224/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/issue-54224.rs:1:39
   |
   |
LL | const FOO: Option<&[[u8; 3]]> = Some(&[*b"foo"]); //~ ERROR temporary value dropped while borrowed
   |                                 |     |        |
   |                                 |     |        temporary value is freed at the end of this statement
   |                                 |     creates a temporary value which is freed while still in use
   |                                 |     creates a temporary value which is freed while still in use
   |                                 using this value as a constant requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/issue-54224.rs:9:57
   |
   |
LL | pub const Z: Cow<'static, [ [u8; 3] ]> = Cow::Borrowed(&[*b"ABC"]);
   |                                          |              |        |
   |                                          |              |        temporary value is freed at the end of this statement
   |                                          |              creates a temporary value which is freed while still in use
   |                                          |              creates a temporary value which is freed while still in use
   |                                          using this value as a constant requires that borrow lasts for `'static`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/min_const_fn/promotion.rs stdout ----
diff of stderr:

2   --> $DIR/promotion.rs:11:27
3    |
4 LL |     let x: &'static () = &foo1();
-    |            -----------    ^^^^^^ creates a temporary which is freed while still in use
+    |            -----------    ^^^^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`

13   --> $DIR/promotion.rs:12:28
14    |
14    |
15 LL |     let y: &'static i32 = &foo2(42);
-    |            ------------    ^^^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^^^ creates a temporary value which is freed while still in use
17    |            |
18    |            type annotation requires that borrow lasts for `'static`

24   --> $DIR/promotion.rs:13:28
25    |
25    |
26 LL |     let z: &'static i32 = &foo3();
-    |            ------------    ^^^^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^^^^ creates a temporary value which is freed while still in use
28    |            |
29    |            type annotation requires that borrow lasts for `'static`

35   --> $DIR/promotion.rs:14:34
36    |
36    |
37 LL |     let a: &'static Cell<i32> = &foo4();
-    |            ------------------    ^^^^^^ creates a temporary which is freed while still in use
+    |            ------------------    ^^^^^^ creates a temporary value which is freed while still in use
39    |            |
40    |            type annotation requires that borrow lasts for `'static`

46   --> $DIR/promotion.rs:15:42
47    |
47    |
48 LL |     let a: &'static Option<Cell<i32>> = &foo5();
-    |            --------------------------    ^^^^^^ creates a temporary which is freed while still in use
+    |            --------------------------    ^^^^^^ creates a temporary value which is freed while still in use
50    |            |
51    |            type annotation requires that borrow lasts for `'static`
52 LL |     let a: &'static Option<Cell<i32>> = &foo6();
57   --> $DIR/promotion.rs:16:42
58    |
58    |
59 LL |     let a: &'static Option<Cell<i32>> = &foo6();
-    |            --------------------------    ^^^^^^ creates a temporary which is freed while still in use
+    |            --------------------------    ^^^^^^ creates a temporary value which is freed while still in use
61    |            |
62    |            type annotation requires that borrow lasts for `'static`
63 LL | }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/promotion/promotion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/min_const_fn/promotion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/min_const_fn/promotion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/promotion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/min_const_fn/promotion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:11:27
   |
   |
LL |     let x: &'static () = &foo1(); //~ ERROR temporary value dropped while borrowed
   |            -----------    ^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:12:28
   |
LL |     let y: &'static i32 = &foo2(42); //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:13:28
   |
LL |     let z: &'static i32 = &foo3(); //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:14:34
   |
LL |     let a: &'static Cell<i32> = &foo4();  //~ ERROR temporary value dropped while borrowed
   |            ------------------    ^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:15:42
   |
LL |     let a: &'static Option<Cell<i32>> = &foo5(); //~ ERROR temporary value dropped while borrowed
   |            --------------------------    ^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     let a: &'static Option<Cell<i32>> = &foo6(); //~ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/min_const_fn/promotion.rs:16:42
   |
   |
LL |     let a: &'static Option<Cell<i32>> = &foo6(); //~ ERROR temporary value dropped while borrowed
   |            --------------------------    ^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 6 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/promote_const_let.rs stdout ----
diff of stderr:

19 LL | |         let y = 42;
20 LL | |         y
21 LL | |     };
-    | |_____^ creates a temporary which is freed while still in use
+    | |_____^ creates a temporary value which is freed while still in use
23 LL |   }
25 


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote_const_let/promote_const_let.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/promote_const_let.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promote_const_let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote_const_let" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote_const_let/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `y` does not live long enough
   |
LL |     let x: &'static u32 = {
LL |     let x: &'static u32 = {
   |            ------------ type annotation requires that `y` is borrowed for `'static`
LL |         let y = 42;
LL |         &y //~ ERROR does not live long enough
   |         ^^ borrowed value does not live long enough
LL |     };
   |     - `y` dropped here while still borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote_const_let.rs:6:28
   |
   |
LL |       let x: &'static u32 = &{ //~ ERROR temporary value dropped while borrowed
   |  ____________------------____^
   | |            |
   | |            type annotation requires that borrow lasts for `'static`
LL | |         let y = 42;
LL | |         y
LL | |     };
   | |_____^ creates a temporary value which is freed while still in use
LL |   }

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0597, E0716.
---
6    |                                        |         |        |
7    |                                        |         |        temporary value is freed at the end of this statement
-    |                                        |         creates a temporary which is freed while still in use
+    |                                        |         creates a temporary value which is freed while still in use
9    |                                        using this value as a static requires that borrow lasts for `'static`
11 error[E0716]: temporary value dropped while borrowed

12   --> $DIR/promote-not.rs:11:18
13    |
13    |
14 LL |     let x = &mut [1,2,3];
-    |                  ^^^^^^^ creates a temporary which is freed while still in use
+    |                  ^^^^^^^ creates a temporary value which is freed while still in use
16 LL |     x
17    |     - using this value as a static requires that borrow lasts for `'static`
18 LL | };
22   --> $DIR/promote-not.rs:20:32
23    |
23    |
24 LL |         let _x: &'static () = &foo();
-    |                 -----------    ^^^^^ creates a temporary which is freed while still in use
+    |                 -----------    ^^^^^ creates a temporary value which is freed while still in use
26    |                 |
27    |                 type annotation requires that borrow lasts for `'static`

32   --> $DIR/promote-not.rs:28:29
33    |
33    |
34 LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x };
-    |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
36    |             |
37    |             type annotation requires that borrow lasts for `'static`
38 LL | }
42   --> $DIR/promote-not.rs:33:29
43    |
43    |
44 LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x };
-    |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
46    |             |
47    |             type annotation requires that borrow lasts for `'static`
48 LL | };
52   --> $DIR/promote-not.rs:39:29
53    |
53    |
54 LL |     let _val: &'static _ = &(Cell::new(1), 2).1;
-    |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
56    |               |
57    |               type annotation requires that borrow lasts for `'static`
58 LL | };
62   --> $DIR/promote-not.rs:46:29
63    |
63    |
64 LL |     let _val: &'static _ = &(Cell::new(1), 2).0;
-    |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
66    |               |
67    |               type annotation requires that borrow lasts for `'static`

73   --> $DIR/promote-not.rs:47:29
74    |
74    |
75 LL |     let _val: &'static _ = &(Cell::new(1), 2).1;
-    |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
77    |               |
78    |               type annotation requires that borrow lasts for `'static`

84   --> $DIR/promote-not.rs:50:29
85    |
85    |
86 LL |     let _val: &'static _ = &(1/0);
-    |               ----------    ^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^ creates a temporary value which is freed while still in use
88    |               |
89    |               type annotation requires that borrow lasts for `'static`

95   --> $DIR/promote-not.rs:51:29
96    |
96    |
97 LL |     let _val: &'static _ = &(1/(1-1));
-    |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^^^^^ creates a temporary value which is freed while still in use
99    |               |
100    |               type annotation requires that borrow lasts for `'static`

106   --> $DIR/promote-not.rs:52:29
107    |
107    |
108 LL |     let _val: &'static _ = &(1%0);
-    |               ----------    ^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^ creates a temporary value which is freed while still in use
110    |               |
111    |               type annotation requires that borrow lasts for `'static`

117   --> $DIR/promote-not.rs:53:29
118    |
118    |
119 LL |     let _val: &'static _ = &(1%(1-1));
-    |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^^^^^ creates a temporary value which is freed while still in use
121    |               |
122    |               type annotation requires that borrow lasts for `'static`

128   --> $DIR/promote-not.rs:54:29
129    |
129    |
130 LL |     let _val: &'static _ = &([1,2,3][4]+1);
-    |               ----------    ^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
132    |               |
133    |               type annotation requires that borrow lasts for `'static`

139   --> $DIR/promote-not.rs:57:29
140    |
140    |
141 LL |     let _val: &'static _ = &TEST_DROP;
-    |               ----------    ^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^^^^^ creates a temporary value which is freed while still in use
143    |               |
144    |               type annotation requires that borrow lasts for `'static`

150   --> $DIR/promote-not.rs:59:29
151    |
151    |
152 LL |     let _val: &'static _ = &&TEST_DROP;
-    |               ----------    ^^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^^^^^^ creates a temporary value which is freed while still in use
154    |               |
155    |               type annotation requires that borrow lasts for `'static`

161   --> $DIR/promote-not.rs:59:30
162    |
162    |
163 LL |     let _val: &'static _ = &&TEST_DROP;
-    |               ----------     ^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------     ^^^^^^^^^ creates a temporary value which is freed while still in use
165    |               |
166    |               type annotation requires that borrow lasts for `'static`

172   --> $DIR/promote-not.rs:62:29
173    |
173    |
174 LL |     let _val: &'static _ = &(&TEST_DROP,);
-    |               ----------    ^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
176    |               |
177    |               type annotation requires that borrow lasts for `'static`

183   --> $DIR/promote-not.rs:62:31
184    |
184    |
185 LL |     let _val: &'static _ = &(&TEST_DROP,);
-    |               ----------      ^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------      ^^^^^^^^^ creates a temporary value which is freed while still in use
187    |               |
188    |               type annotation requires that borrow lasts for `'static`

194   --> $DIR/promote-not.rs:65:29
195    |
195    |
196 LL |     let _val: &'static _ = &[&TEST_DROP; 1];
-    |               ----------    ^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |               ----------    ^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
198    |               |
199    |               type annotation requires that borrow lasts for `'static`


207 LL |     let _val: &'static _ = &[&TEST_DROP; 1];
209    |               |               |
-    |               |               creates a temporary which is freed while still in use
+    |               |               creates a temporary value which is freed while still in use
+    |               |               creates a temporary value which is freed while still in use
211    |               type annotation requires that borrow lasts for `'static`
213 error: aborting due to 20 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote-not/promote-not.stderr
To only update this specific test, also pass `--test-args consts/promote-not.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promote-not.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote-not" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promote-not/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/promote-not.rs:8:50
   |
   |
LL | static mut TEST1: Option<&mut [i32]> = Some(&mut [1, 2, 3]); //~ ERROR temporary value dropped while borrowed
   |                                        |         |        |
   |                                        |         |        temporary value is freed at the end of this statement
   |                                        |         creates a temporary value which is freed while still in use
   |                                        |         creates a temporary value which is freed while still in use
   |                                        using this value as a static requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:11:18
   |
   |
LL |     let x = &mut [1,2,3]; //~ ERROR temporary value dropped while borrowed
   |                  ^^^^^^^ creates a temporary value which is freed while still in use
LL |     x
   |     - using this value as a static requires that borrow lasts for `'static`
LL | };
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:20:32
   |
   |
LL |         let _x: &'static () = &foo(); //~ ERROR temporary value dropped while borrowed
   |                 -----------    ^^^^^ creates a temporary value which is freed while still in use
   |                 |
   |                 type annotation requires that borrow lasts for `'static`
   |     - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:28:29
  --> /checkout/src/test/ui/consts/promote-not.rs:28:29
   |
LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x }; //~ ERROR temporary value dropped while borrowed
   |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |             |
   |             type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:33:29
   |
   |
LL |     let _x: &'static i32 = &unsafe { U { x: 0 }.x }; //~ ERROR temporary value dropped while borrowed
   |             ------------    ^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |             |
   |             type annotation requires that borrow lasts for `'static`
LL | };
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:39:29
   |
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).1; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | };
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:46:29
   |
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).0; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:47:29
   |
LL |     let _val: &'static _ = &(Cell::new(1), 2).1; //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:50:29
   |
LL |     let _val: &'static _ = &(1/0); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:51:29
   |
LL |     let _val: &'static _ = &(1/(1-1)); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:52:29
   |
LL |     let _val: &'static _ = &(1%0); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:53:29
   |
LL |     let _val: &'static _ = &(1%(1-1)); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:54:29
   |
LL |     let _val: &'static _ = &([1,2,3][4]+1); //~ ERROR temporary value dropped while borrowed
   |               ----------    ^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:57:29
   |
LL |     let _val: &'static _ = &TEST_DROP;
   |               ----------    ^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:59:29
   |
LL |     let _val: &'static _ = &&TEST_DROP;
   |               ----------    ^^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:59:30
   |
LL |     let _val: &'static _ = &&TEST_DROP;
   |               ----------     ^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:62:29
   |
LL |     let _val: &'static _ = &(&TEST_DROP,);
   |               ----------    ^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:62:31
   |
LL |     let _val: &'static _ = &(&TEST_DROP,);
   |               ----------      ^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:65:29
   |
LL |     let _val: &'static _ = &[&TEST_DROP; 1];
   |               ----------    ^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promote-not.rs:65:31
   |
LL |     let _val: &'static _ = &[&TEST_DROP; 1];
   |               |               |
   |               |               creates a temporary value which is freed while still in use
   |               |               creates a temporary value which is freed while still in use
   |               type annotation requires that borrow lasts for `'static`
error: aborting due to 20 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/promoted-const-drop.rs stdout ----
diff of stderr:

2   --> $DIR/promoted-const-drop.rs:13:26
3    |
4 LL |     let _: &'static A = &A();
-    |            ----------    ^^^ creates a temporary which is freed while still in use
+    |            ----------    ^^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`
8 LL |     let _: &'static [A] = &[C];
13   --> $DIR/promoted-const-drop.rs:14:28
14    |
14    |
15 LL |     let _: &'static [A] = &[C];
-    |            ------------    ^^^ creates a temporary which is freed while still in use
+    |            ------------    ^^^ creates a temporary value which is freed while still in use
17    |            |
18    |            type annotation requires that borrow lasts for `'static`
19 LL | }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promoted-const-drop/promoted-const-drop.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/promoted-const-drop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/promoted-const-drop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promoted-const-drop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/promoted-const-drop/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/consts/promoted-const-drop.rs:13:26
   |
   |
LL |     let _: &'static A = &A(); //~ ERROR temporary value dropped while borrowed
   |            ----------    ^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL |     let _: &'static [A] = &[C]; //~ ERROR temporary value dropped while borrowed
LL | }
   | - temporary value is freed at the end of this statement
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/consts/promoted-const-drop.rs:14:28
   |
   |
LL |     let _: &'static [A] = &[C]; //~ ERROR temporary value dropped while borrowed
   |            ------------    ^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/consts/qualif-union.rs stdout ----
diff of stderr:

2   --> $DIR/qualif-union.rs:28:26
3    |
4 LL |     let _: &'static _ = &C1;
-    |            ----------    ^^ creates a temporary which is freed while still in use
+    |            ----------    ^^ creates a temporary value which is freed while still in use
6    |            |
7    |            type annotation requires that borrow lasts for `'static`

13   --> $DIR/qualif-union.rs:29:26
14    |
14    |
15 LL |     let _: &'static _ = &C2;
-    |            ----------    ^^ creates a temporary which is freed while still in use
+    |            ----------    ^^ creates a temporary value which is freed while still in use
17    |            |
18    |            type annotation requires that borrow lasts for `'static`

24   --> $DIR/qualif-union.rs:30:26
25    |
25    |
26 LL |     let _: &'static _ = &C3;
-    |            ----------    ^^ creates a temporary which is freed while still in use
+    |            ----------    ^^ creates a temporary value which is freed while still in use
28    |            |
29    |            type annotation requires that borrow lasts for `'static`

35   --> $DIR/qualif-union.rs:31:26
36    |
36    |
37 LL |     let _: &'static _ = &C4;
---
4 LL |     let slice = &mut ();
-    |                      ^^ creates a temporary which is freed while still in use
+    |                      ^^ creates a temporary value which is freed while still in use
6 ...
7 LL |     print_items::<WindowsMut<'_>>(windows);
8    |     -------------------------------------- argument requires that borrow lasts for `'static`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/hrtb-implied-1/hrtb-implied-1.stderr
To only update this specific test, also pass `--test-args generic-associated-types/bugs/hrtb-implied-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/bugs/hrtb-implied-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/hrtb-implied-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/hrtb-implied-1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/generic-associated-types/bugs/hrtb-implied-1.rs:31:22
   |
LL |     let slice = &mut ();
   |                      ^^ creates a temporary value which is freed while still in use
   |                      ^^ creates a temporary value which is freed while still in use
...
LL |     print_items::<WindowsMut<'_>>(windows);
   |     -------------------------------------- argument requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
   |
note: due to current limitations in the borrow checker, this implies a `'static` lifetime
   |
   |
LL |     for<'a> I::Item<'a>: Debug,

error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
For more information about this error, try `rustc --explain E0716`.
------------------------------------------


---- [ui] src/test/ui/issues/issue-47184.rs stdout ----
diff of stderr:

4 LL |     let _vec: Vec<&'static String> = vec![&String::new()];
5    |               --------------------         ^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
-    |               |                            creates a temporary which is freed while still in use
+    |               |                            creates a temporary value which is freed while still in use
+    |               |                            creates a temporary value which is freed while still in use
8    |               type annotation requires that borrow lasts for `'static`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47184/issue-47184.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-47184.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-47184.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47184" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-47184/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-47184.rs:2:44
   |
   |
LL |     let _vec: Vec<&'static String> = vec![&String::new()];
   |               --------------------         ^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |               |                            creates a temporary value which is freed while still in use
   |               |                            creates a temporary value which is freed while still in use
   |               type annotation requires that borrow lasts for `'static`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/issues/issue-52049.rs stdout ----
diff of stderr:

4 LL |     foo(&unpromotable(5u32));
6    |     |    |
-    |     |    creates a temporary which is freed while still in use
+    |     |    creates a temporary value which is freed while still in use
+    |     |    creates a temporary value which is freed while still in use
8    |     argument requires that borrow lasts for `'static`
9 LL | }
10    | - temporary value is freed at the end of this statement

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049/issue-52049.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-52049.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-52049.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-52049/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-52049.rs:6:10
   |
   |
LL |     foo(&unpromotable(5u32));
   |     |    |
   |     |    creates a temporary value which is freed while still in use
   |     |    creates a temporary value which is freed while still in use
   |     argument requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/lifetimes/borrowck-let-suggestion.rs stdout ----
diff of stderr:

4 LL |     let mut x = vec![1].iter();
6    |                 |
-    |                 creates a temporary which is freed while still in use
+    |                 creates a temporary value which is freed while still in use
8 LL |
---
To only update this specific test, also pass `--test-args lifetimes/borrowck-let-suggestion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/borrowck-let-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/borrowck-let-suggestion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/borrowck-let-suggestion/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/lifetimes/borrowck-let-suggestion.rs:2:17
   |
   |
LL |     let mut x = vec![1].iter();
   |                 |
   |                 creates a temporary value which is freed while still in use
   |                 creates a temporary value which is freed while still in use
LL |     //~^ ERROR temporary value dropped while borrowed
LL |     x.use_mut();
   |
   = note: consider using a `let` binding to create a longer lived value
   = note: this error originates in the macro `vec` (in Nightly builds, run with -Z macro-backtrace for more info)

---
To only update this specific test, also pass `--test-args nll/borrowed-temporary-error.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/borrowed-temporary-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/borrowed-temporary-error" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/borrowed-temporary-error/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/nll/borrowed-temporary-error.rs:8:10
   |
LL |         &(v,)
   |          ^^^^ creates a temporary value which is freed while still in use
   |          ^^^^ creates a temporary value which is freed while still in use
LL |         //~^ ERROR temporary value dropped while borrowed [E0716]
   |       - temporary value is freed at the end of this statement
LL |     println!("{:?}", x);
   |                      - borrow later used here
   |
---

---- [ui] src/test/ui/nll/issue-57265-return-type-wf-check.rs stdout ----
diff of stderr:

4 LL |     let (_, z) = foo(&"hello".to_string());
5    |                  -----^^^^^^^^^^^^^^^^^^^-- temporary value is freed at the end of this statement
-    |                  |    creates a temporary which is freed while still in use
+    |                  |    creates a temporary value which is freed while still in use
+    |                  |    creates a temporary value which is freed while still in use
8    |                  argument requires that borrow lasts for `'static`
10 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-57265-return-type-wf-check/issue-57265-return-type-wf-check.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/issue-57265-return-type-wf-check.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/issue-57265-return-type-wf-check.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-57265-return-type-wf-check" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/issue-57265-return-type-wf-check/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/nll/issue-57265-return-type-wf-check.rs:20:23
   |
   |
LL |     let (_, z) = foo(&"hello".to_string());
   |                  -----^^^^^^^^^^^^^^^^^^^-- temporary value is freed at the end of this statement
   |                  |    creates a temporary value which is freed while still in use
   |                  |    creates a temporary value which is freed while still in use
   |                  argument requires that borrow lasts for `'static`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/nll/user-annotations/patterns.rs stdout ----
diff of stderr:

76 LL |     let _: Vec<&'static String> = vec![&String::new()];
77    |            --------------------         ^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
-    |            |                            creates a temporary which is freed while still in use
+    |            |                            creates a temporary value which is freed while still in use
+    |            |                            creates a temporary value which is freed while still in use
80    |            type annotation requires that borrow lasts for `'static`
82 error[E0716]: temporary value dropped while borrowed


85 LL |     let (_, a): (Vec<&'static String>, _) = (vec![&String::new()], 44);
87    |                 |                                  |
-    |                 |                                  creates a temporary which is freed while still in use
+    |                 |                                  creates a temporary value which is freed while still in use
+    |                 |                                  creates a temporary value which is freed while still in use
89    |                 type annotation requires that borrow lasts for `'static`
91 error[E0716]: temporary value dropped while borrowed


94 LL |     let (_a, b): (Vec<&'static String>, _) = (vec![&String::new()], 44);
96    |                  |                                  |
-    |                  |                                  creates a temporary which is freed while still in use
+    |                  |                                  creates a temporary value which is freed while still in use
+    |                  |                                  creates a temporary value which is freed while still in use
98    |                  type annotation requires that borrow lasts for `'static`
99 
100 error[E0597]: `x` does not live long enough

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/patterns/patterns.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/patterns.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/patterns.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/patterns" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/patterns/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `x` does not live long enough
   |
   |
LL |     let y: &'static u32;
   |            ------------ type annotation requires that `x` is borrowed for `'static`
LL |     y = &x; //~ ERROR
   |         ^^ borrowed value does not live long enough
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     let (y, z): (&'static u32, &'static u32);
   |                 ---------------------------- type annotation requires that `x` is borrowed for `'static`
LL |     y = &x; //~ ERROR
   |         ^^ borrowed value does not live long enough
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     let y = &x; //~ ERROR
   |             ^^ borrowed value does not live long enough
LL |     let ref z: &'static u32 = y;
   |                ------------ type annotation requires that `x` is borrowed for `'static`
LL |     **z
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     let Single { value: y }: Single<&'static u32>;
   |                              -------------------- type annotation requires that `x` is borrowed for `'static`
LL |     y = &x; //~ ERROR
   |         ^^ borrowed value does not live long enough
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     let Single2 { value: mut _y }: Single2<StaticU32>;
   |                                    ------------------ type annotation requires that `x` is borrowed for `'static`
LL |     _y = &x; //~ ERROR
   |          ^^ borrowed value does not live long enough
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     let y: &'static u32 = &x; //~ ERROR
   |            ------------   ^^ borrowed value does not live long enough
   |            |
   |            type annotation requires that `x` is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     let _: &'static u32 = &x; //~ ERROR
   |            ------------   ^^ borrowed value does not live long enough
   |            |
   |            type annotation requires that `x` is borrowed for `'static`
LL | }
LL | }
   | - `x` dropped here while still borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/nll/user-annotations/patterns.rs:63:41
   |
   |
LL |     let _: Vec<&'static String> = vec![&String::new()];
   |            --------------------         ^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |            |                            creates a temporary value which is freed while still in use
   |            |                            creates a temporary value which is freed while still in use
   |            type annotation requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/nll/user-annotations/patterns.rs:66:52
   |
   |
LL |     let (_, a): (Vec<&'static String>, _) = (vec![&String::new()], 44);
   |                 |                                  |
   |                 |                                  creates a temporary value which is freed while still in use
   |                 |                                  creates a temporary value which is freed while still in use
   |                 type annotation requires that borrow lasts for `'static`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/nll/user-annotations/patterns.rs:69:53
   |
   |
LL |     let (_a, b): (Vec<&'static String>, _) = (vec![&String::new()], 44);
   |                  |                                  |
   |                  |                                  creates a temporary value which is freed while still in use
   |                  |                                  creates a temporary value which is freed while still in use
   |                  type annotation requires that borrow lasts for `'static`

error[E0597]: `x` does not live long enough
   |
   |
LL |     let (_, _): (&'static u32, u32) = (&x, 44); //~ ERROR
   |                 -------------------    ^^ borrowed value does not live long enough
   |                 |
   |                 type annotation requires that `x` is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     let (y, _): (&'static u32, u32) = (&x, 44); //~ ERROR
   |                 -------------------    ^^ borrowed value does not live long enough
   |                 |
   |                 type annotation requires that `x` is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     let Single { value: y }: Single<&'static u32> = Single { value: &x }; //~ ERROR
   |                              --------------------                   ^^ borrowed value does not live long enough
   |                              |
   |                              type annotation requires that `x` is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     let Single { value: _ }: Single<&'static u32> = Single { value: &x }; //~ ERROR
   |                              --------------------                   ^^ borrowed value does not live long enough
   |                              |
   |                              type annotation requires that `x` is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed

error[E0597]: `x` does not live long enough
   |
   |
LL |     let Double { value1: _, value2: _ }: Double<&'static u32> = Double {
   |                                          -------------------- type annotation requires that `x` is borrowed for `'static`
LL |         value1: &x, //~ ERROR
   |                 ^^ borrowed value does not live long enough
LL | }
LL | }
   | - `x` dropped here while still borrowed
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/patterns.rs:111:5
   |
   |
LL | fn static_to_a_to_static_through_variable<'a>(x: &'a u32) -> &'static u32 {
   |                                           -- lifetime `'a` defined here
...
LL |     y //~ ERROR
   |     ^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/patterns.rs:123:5
   |
   |
LL | fn static_to_a_to_static_through_tuple<'a>(x: &'a u32) -> &'static u32 {
   |                                        -- lifetime `'a` defined here
...
LL |     y //~ ERROR
   |     ^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/patterns.rs:128:5
   |
   |
LL | fn static_to_a_to_static_through_struct<'a>(_x: &'a u32) -> &'static u32 {
   |                                         -- lifetime `'a` defined here
LL |     let Single { value: y }: Single<&'a u32> = Single { value: &22 };
LL |     y //~ ERROR
   |     ^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/patterns.rs:132:18
   |
   |
LL | fn a_to_static_then_static<'a>(x: &'a u32) -> &'static u32 {
   |                            -- lifetime `'a` defined here
LL |     let (y, _z): (&'static u32, u32) = (x, 44); //~ ERROR
   |                  ^^^^^^^^^^^^^^^^^^^ type annotation requires that `'a` must outlive `'static`
error: aborting due to 19 previous errors

Some errors have detailed explanations: E0597, E0716.
For more information about an error, try `rustc --explain E0597`.
For more information about an error, try `rustc --explain E0597`.
------------------------------------------


---- [ui] src/test/ui/pin-macro/lifetime_errors_on_promotion_misusage.rs stdout ----
diff of stderr:

4 LL |     let phantom_pinned = identity(pin!(PhantomPinned));
5    |                                   ^^^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
-    |                                   creates a temporary which is freed while still in use
+    |                                   creates a temporary value which is freed while still in use
8 LL |
8 LL |
9 LL |     stuff(phantom_pinned)


18 LL |     let phantom_pinned = {
19    |         -------------- borrow later stored here
20 LL |         let phantom_pinned = pin!(PhantomPinned);
-    |                              ^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |                              ^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
23 LL |     };
24    |     - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pin-macro/lifetime_errors_on_promotion_misusage/lifetime_errors_on_promotion_misusage.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args pin-macro/lifetime_errors_on_promotion_misusage.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/pin-macro/lifetime_errors_on_promotion_misusage.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pin-macro/lifetime_errors_on_promotion_misusage" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/pin-macro/lifetime_errors_on_promotion_misusage/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/pin-macro/lifetime_errors_on_promotion_misusage.rs:12:35
   |
   |
LL |     let phantom_pinned = identity(pin!(PhantomPinned));
   |                                   ^^^^^^^^^^^^^^^^^^^ - temporary value is freed at the end of this statement
   |                                   creates a temporary value which is freed while still in use
   |                                   creates a temporary value which is freed while still in use
LL |     //~^ ERROR temporary value dropped while borrowed
LL |     stuff(phantom_pinned)
   |
   = note: consider using a `let` binding to create a longer lived value
   = note: this error originates in the macro `pin` (in Nightly builds, run with -Z macro-backtrace for more info)


error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/pin-macro/lifetime_errors_on_promotion_misusage.rs:19:30
   |
LL |     let phantom_pinned = {
   |         -------------- borrow later stored here
LL |         let phantom_pinned = pin!(PhantomPinned);
   |                              ^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
LL |     };
   |     - temporary value is freed at the end of this statement
   |
   = note: consider using a `let` binding to create a longer lived value
---

---- [ui] src/test/ui/regions/regions-free-region-ordering-caller1.rs stdout ----
diff of stderr:

5    |          -- lifetime `'a` defined here
6 ...
7 LL |     let z: &'a & usize = &(&y);
-    |            -----------    ^^^^ creates a temporary which is freed while still in use
+    |            -----------    ^^^^ creates a temporary value which is freed while still in use
9    |            |
10    |            type annotation requires that borrow lasts for `'a`


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-caller1/regions-free-region-ordering-caller1.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-caller1/regions-free-region-ordering-caller1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-free-region-ordering-caller1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-free-region-ordering-caller1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-caller1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-ordering-caller1/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/regions/regions-free-region-ordering-caller1.rs:9:27
   |
   |
LL | fn call1<'a>(x: &'a usize) {
   |          -- lifetime `'a` defined here
...
LL |     let z: &'a & usize = &(&y);
   |            -----------    ^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'a`
LL | }
   | - temporary value is freed at the end of this statement


error[E0597]: `y` does not live long enough
   |
   |
LL | fn call1<'a>(x: &'a usize) {
   |          -- lifetime `'a` defined here
...
LL |     let z: &'a & usize = &(&y);
   |            -----------    ^^^^ borrowed value does not live long enough
   |            |
   |            type annotation requires that `y` is borrowed for `'a`
LL | }
LL | }
   | - `y` dropped here while still borrowed
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0597, E0716.
For more information about an error, try `rustc --explain E0597`.
For more information about an error, try `rustc --explain E0597`.
------------------------------------------


---- [ui] src/test/ui/regions/regions-var-type-out-of-scope.rs stdout ----
diff of stderr:

4 LL |         x = &id(3);
5    |              ^^^^^- temporary value is freed at the end of this statement
-    |              creates a temporary which is freed while still in use
+    |              creates a temporary value which is freed while still in use
+    |              creates a temporary value which is freed while still in use
8 LL |         assert_eq!(*x, 3);
10    |


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-var-type-out-of-scope/regions-var-type-out-of-scope.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args regions/regions-var-type-out-of-scope.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/regions/regions-var-type-out-of-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-var-type-out-of-scope" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-var-type-out-of-scope/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/regions/regions-var-type-out-of-scope.rs:9:14
   |
   |
LL |         x = &id(3); //~ ERROR temporary value dropped while borrowed
   |              ^^^^^- temporary value is freed at the end of this statement
   |              creates a temporary value which is freed while still in use
   |              creates a temporary value which is freed while still in use
LL |         assert_eq!(*x, 3);
   |
   = note: consider using a `let` binding to create a longer lived value

error: aborting due to previous error
---
diff of stderr:

2   --> $DIR/borrowck-ref-into-rvalue.rs:4:11
3    |
4 LL |     match Some("Hello".to_string()) {
-    |           ^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |           ^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
7 LL |     }
8    |     - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-ref-into-rvalue/borrowck-ref-into-rvalue.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/borrowck-ref-into-rvalue.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/borrowck-ref-into-rvalue.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-ref-into-rvalue" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-ref-into-rvalue/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/span/borrowck-ref-into-rvalue.rs:4:11
   |
   |
LL |     match Some("Hello".to_string()) {
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
LL |     }
   |     - temporary value is freed at the end of this statement
LL |     println!("{}", *msg);
   |                    ---- borrow later used here
   |                    ---- borrow later used here
   |
help: consider using a `let` binding to create a longer lived value
   |
LL ~     let binding = Some("Hello".to_string());
LL ~     match binding {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
For more information about this error, try `rustc --explain E0716`.
------------------------------------------


---- [ui] src/test/ui/span/borrowck-let-suggestion-suffixes.rs stdout ----
diff of stderr:

16 LL |     v3.push(&id('x'));           // statement 6
17    |              ^^^^^^^ - temporary value is freed at the end of this statement
-    |              creates a temporary which is freed while still in use
+    |              creates a temporary value which is freed while still in use
20 ...
20 ...
21 LL |     (v1, v2, v3, /* v4 is above. */ v5).use_ref();


33 LL |         v4.push(&id('y'));
34    |                  ^^^^^^^ - temporary value is freed at the end of this statement
-    |                  creates a temporary which is freed while still in use
+    |                  creates a temporary value which is freed while still in use
37 ...
38 LL |         v4.use_ref();
38 LL |         v4.use_ref();
39    |         ------------ borrow later used here

46 LL |     v5.push(&id('z'));
47    |              ^^^^^^^ - temporary value is freed at the end of this statement
-    |              creates a temporary which is freed while still in use
+    |              creates a temporary value which is freed while still in use
50 ...
50 ...
51 LL |     (v1, v2, v3, /* v4 is above. */ v5).use_ref();


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-let-suggestion-suffixes/borrowck-let-suggestion-suffixes.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-let-suggestion-suffixes/borrowck-let-suggestion-suffixes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/borrowck-let-suggestion-suffixes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/borrowck-let-suggestion-suffixes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-let-suggestion-suffixes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/borrowck-let-suggestion-suffixes/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `young[_]` does not live long enough
   |
   |
LL |         v2.push(&young[0]);      // statement 4
   |                 ^^^^^^^^^ borrowed value does not live long enough
...
LL |     } //~ NOTE `young[_]` dropped here while still borrowed
   |     - `young[_]` dropped here while still borrowed
...
LL |     (v1, v2, v3, /* v4 is above. */ v5).use_ref();

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/span/borrowck-let-suggestion-suffixes.rs:19:14
   |
   |
LL |     v3.push(&id('x'));           // statement 6
   |              ^^^^^^^ - temporary value is freed at the end of this statement
   |              creates a temporary value which is freed while still in use
...
...
LL |     (v1, v2, v3, /* v4 is above. */ v5).use_ref();
   |
help: consider using a `let` binding to create a longer lived value
   |
   |
LL ~     let binding = id('x');
LL ~     v3.push(&binding);           // statement 6

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/span/borrowck-let-suggestion-suffixes.rs:29:18
   |
   |
LL |         v4.push(&id('y'));
   |                  ^^^^^^^ - temporary value is freed at the end of this statement
   |                  creates a temporary value which is freed while still in use
...
LL |         v4.use_ref();
   |         ------------ borrow later used here
   |         ------------ borrow later used here
   |
   = note: consider using a `let` binding to create a longer lived value

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/span/borrowck-let-suggestion-suffixes.rs:40:14
   |
LL |     v5.push(&id('z'));
   |              ^^^^^^^ - temporary value is freed at the end of this statement
   |              creates a temporary value which is freed while still in use
...
...
LL |     (v1, v2, v3, /* v4 is above. */ v5).use_ref();
   |
help: consider using a `let` binding to create a longer lived value
   |
   |
LL ~     let binding = id('z');
LL ~     v5.push(&binding);

error: aborting due to 4 previous errors

Some errors have detailed explanations: E0597, E0716.
---
To only update this specific test, also pass `--test-args span/issue-15480.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/issue-15480.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-15480" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/issue-15480/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/span/issue-15480.rs:6:10
   |
LL |         &id(3)
   |          ^^^^^ creates a temporary value which is freed while still in use
   |          ^^^^^ creates a temporary value which is freed while still in use
LL |     ];
   |      - temporary value is freed at the end of this statement
...
LL |     for &&x in &v {
   |
help: consider using a `let` binding to create a longer lived value
   |
   |
LL ~     let binding = id(3);
LL ~     let v = vec![
LL ~         &binding

error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
---
diff of stderr:

2   --> $DIR/regions-close-over-borrowed-ref-in-obj.rs:12:27
3    |
4 LL |         let ss: &isize = &id(1);
-    |                           ^^^^^ creates a temporary which is freed while still in use
+    |                           ^^^^^ creates a temporary value which is freed while still in use
7 LL |     }
8    |     - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-close-over-borrowed-ref-in-obj/regions-close-over-borrowed-ref-in-obj.stderr
To only update this specific test, also pass `--test-args span/regions-close-over-borrowed-ref-in-obj.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/regions-close-over-borrowed-ref-in-obj.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-close-over-borrowed-ref-in-obj" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/regions-close-over-borrowed-ref-in-obj/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/span/regions-close-over-borrowed-ref-in-obj.rs:12:27
   |
   |
LL |         let ss: &isize = &id(1);
   |                           ^^^^^ creates a temporary value which is freed while still in use
LL |     }
   |     - temporary value is freed at the end of this statement
LL | }
LL | }
   | - borrow might be used here, when `blah` is dropped and runs the destructor for type `Box<dyn Foo>`
   = note: consider using a `let` binding to create a longer lived value

error: aborting due to previous error

---
diff of stderr:

2   --> $DIR/slice-borrow.rs:6:28
3    |
4 LL |         let x: &[isize] = &vec![1, 2, 3, 4, 5];
-    |                            ^^^^^^^^^^^^^^^^^^^ creates a temporary which is freed while still in use
+    |                            ^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
7 LL |     }
8    |     - temporary value is freed at the end of this statement



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/slice-borrow/slice-borrow.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args span/slice-borrow.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/span/slice-borrow.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/slice-borrow" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/span/slice-borrow/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/span/slice-borrow.rs:6:28
   |
   |
LL |         let x: &[isize] = &vec![1, 2, 3, 4, 5];
   |                            ^^^^^^^^^^^^^^^^^^^ creates a temporary value which is freed while still in use
LL |     }
   |     - temporary value is freed at the end of this statement
LL |     y.use_ref();
   |     ----------- borrow later used here
---
14    |                                                      |     |       |
15    |                                                      |     |       temporary value is freed at the end of this statement
-    |                                                      |     creates a temporary which is freed while still in use
+    |                                                      |     creates a temporary value which is freed while still in use
17    |                                                      using this value as a static requires that borrow lasts for `'static`
18 
19 error[E0493]: destructor of `WithDtor` cannot be evaluated at compile-time
31    |                                                     ------^^^^^^^^-
32    |                                                     |     |       |
33    |                                                     |     |       temporary value is freed at the end of this statement
-    |                                                     |     creates a temporary which is freed while still in use
-    |                                                     |     creates a temporary which is freed while still in use
+    |                                                     |     creates a temporary value which is freed while still in use
35    |                                                     using this value as a constant requires that borrow lasts for `'static`
36 
37 error[E0493]: destructor of `(WithDtor, i32)` cannot be evaluated at compile-time

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/static-drop-scope.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-drop-scope.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-drop-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-drop-scope/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0493]: destructor of `WithDtor` cannot be evaluated at compile-time
   |
   |
LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                            ^^^^^^^^- value is dropped here
   |                                                            the destructor for this type cannot be evaluated in statics

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-drop-scope.rs:7:60
  --> /checkout/src/test/ui/static/static-drop-scope.rs:7:60
   |
LL | static PROMOTION_FAIL_S: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                      |     |       |
   |                                                      |     |       temporary value is freed at the end of this statement
   |                                                      |     creates a temporary value which is freed while still in use
   |                                                      |     creates a temporary value which is freed while still in use
   |                                                      using this value as a static requires that borrow lasts for `'static`

error[E0493]: destructor of `WithDtor` cannot be evaluated at compile-time
   |
   |
LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                           ^^^^^^^^- value is dropped here
   |                                                           the destructor for this type cannot be evaluated in constants

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-drop-scope.rs:11:59
  --> /checkout/src/test/ui/static/static-drop-scope.rs:11:59
   |
LL | const PROMOTION_FAIL_C: Option<&'static WithDtor> = Some(&WithDtor);
   |                                                     |     |       |
   |                                                     |     |       temporary value is freed at the end of this statement
   |                                                     |     creates a temporary value which is freed while still in use
   |                                                     |     creates a temporary value which is freed while still in use
   |                                                     using this value as a constant requires that borrow lasts for `'static`

error[E0493]: destructor of `(WithDtor, i32)` cannot be evaluated at compile-time
   |
   |
LL | static EARLY_DROP_S: i32 = (WithDtor, 0).1;
   |                            ^^^^^^^^^^^^^ - value is dropped here
   |                            the destructor for this type cannot be evaluated in statics


error[E0493]: destructor of `(WithDtor, i32)` cannot be evaluated at compile-time
   |
   |
LL | const EARLY_DROP_C: i32 = (WithDtor, 0).1;
   |                           ^^^^^^^^^^^^^ - value is dropped here
   |                           the destructor for this type cannot be evaluated in constants


error[E0493]: destructor of `T` cannot be evaluated at compile-time
   |
   |
LL | const fn const_drop<T>(_: T) {}
   |                        ^      - value is dropped here
   |                        the destructor for this type cannot be evaluated in constant functions


error[E0493]: destructor of `(T, ())` cannot be evaluated at compile-time
   |
   |
LL |     (x, ()).1
   |     ^^^^^^^ the destructor for this type cannot be evaluated in constant functions
LL |     //~^ ERROR destructor of
LL | }
   | - value is dropped here

error[E0493]: destructor of `(Option<WithDtor>, i32)` cannot be evaluated at compile-time
   |
   |
LL | const EARLY_DROP_C_OPTION: i32 = (Some(WithDtor), 0).1;
   |                                  ^^^^^^^^^^^^^^^^^^^ - value is dropped here
   |                                  the destructor for this type cannot be evaluated in constants


error[E0493]: destructor of `(Option<WithDtor>, i32)` cannot be evaluated at compile-time
   |
   |
LL | const EARLY_DROP_C_OPTION_CONSTANT: i32 = (HELPER, 0).1;
   |                                           ^^^^^^^^^^^ - value is dropped here
   |                                           the destructor for this type cannot be evaluated in constants

error: aborting due to 10 previous errors

---
diff of stderr:

2   --> $DIR/static-region-bound.rs:10:14
3    |
4 LL |     let x = &id(3);
-    |              ^^^^^ creates a temporary which is freed while still in use
+    |              ^^^^^ creates a temporary value which is freed while still in use
6 LL |     f(x);
7    |     ---- argument requires that borrow lasts for `'static`
8 LL | }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound/static-region-bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args static/static-region-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-region-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-region-bound/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/static/static-region-bound.rs:10:14
   |
   |
LL |     let x = &id(3); //~ ERROR temporary value dropped while borrowed
   |              ^^^^^ creates a temporary value which is freed while still in use
LL |     f(x);
   |     ---- argument requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/static/static-reference-to-fn-2.rs stdout ----
diff of stderr:

6 LL |     self_.statefn = &id(state2 as StateMachineFunc);
7    |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
-    |     |                creates a temporary which is freed while still in use
+    |     |                creates a temporary value which is freed while still in use
+    |     |                creates a temporary value which is freed while still in use
10    |     assignment requires that borrow lasts for `'1`
12 error[E0716]: temporary value dropped while borrowed


17 LL |     self_.statefn = &id(state3 as StateMachineFunc);
18    |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
-    |     |                creates a temporary which is freed while still in use
+    |     |                creates a temporary value which is freed while still in use
+    |     |                creates a temporary value which is freed while still in use
21    |     assignment requires that borrow lasts for `'1`
23 error[E0716]: temporary value dropped while borrowed


28 LL |     self_.statefn = &id(finished as StateMachineFunc);
29    |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
-    |     |                creates a temporary which is freed while still in use
+    |     |                creates a temporary value which is freed while still in use
+    |     |                creates a temporary value which is freed while still in use
32    |     assignment requires that borrow lasts for `'1`
34 error[E0515]: cannot return value referencing temporary value


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-reference-to-fn-2/static-reference-to-fn-2.stderr
To only update this specific test, also pass `--test-args static/static-reference-to-fn-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-reference-to-fn-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-reference-to-fn-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-reference-to-fn-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/static/static-reference-to-fn-2.rs:18:22
   |
   |
LL | fn state1(self_: &mut StateMachineIter) -> Option<&'static str> {
   |           ----- has type `&mut StateMachineIter<'1>`
LL |     self_.statefn = &id(state2 as StateMachineFunc);
   |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
   |     |                creates a temporary value which is freed while still in use
   |     |                creates a temporary value which is freed while still in use
   |     assignment requires that borrow lasts for `'1`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-reference-to-fn-2.rs:24:22
   |
   |
LL | fn state2(self_: &mut StateMachineIter) -> Option<(&'static str)> {
   |           ----- has type `&mut StateMachineIter<'1>`
LL |     self_.statefn = &id(state3 as StateMachineFunc);
   |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
   |     |                creates a temporary value which is freed while still in use
   |     |                creates a temporary value which is freed while still in use
   |     assignment requires that borrow lasts for `'1`
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/static/static-reference-to-fn-2.rs:30:22
   |
   |
LL | fn state3(self_: &mut StateMachineIter) -> Option<(&'static str)> {
   |           ----- has type `&mut StateMachineIter<'1>`
LL |     self_.statefn = &id(finished as StateMachineFunc);
   |     -----------------^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^- temporary value is freed at the end of this statement
   |     |                creates a temporary value which is freed while still in use
   |     |                creates a temporary value which is freed while still in use
   |     assignment requires that borrow lasts for `'1`
error[E0515]: cannot return value referencing temporary value
  --> /checkout/src/test/ui/static/static-reference-to-fn-2.rs:40:5
   |
LL | /     StateMachineIter {
LL | /     StateMachineIter {
LL | |     //~^ ERROR cannot return value referencing temporary value
LL | |         statefn: &id(state1 as StateMachineFunc)
LL | |     }
LL | |     }
   | |_____^ returns a value referencing data owned by the current function
   |
   = help: use `.collect()` to allocate the iterator
error: aborting due to 4 previous errors

Some errors have detailed explanations: E0515, E0716.
For more information about an error, try `rustc --explain E0515`.
---
diff of stderr:

2   --> $DIR/issue-44373.rs:4:42
3    |
4 LL |     let _val: &'static [&'static u32] = &[&FOO];
-    |               -----------------------    ^^^^^^ creates a temporary which is freed while still in use
+    |               -----------------------    ^^^^^^ creates a temporary value which is freed while still in use
6    |               |
7    |               type annotation requires that borrow lasts for `'static`
8 LL | }

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/statics/issue-44373/issue-44373.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args statics/issue-44373.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/statics/issue-44373.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/statics/issue-44373" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/statics/issue-44373/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/statics/issue-44373.rs:4:42
   |
   |
LL |     let _val: &'static [&'static u32] = &[&FOO]; //~ ERROR temporary value dropped while borrowed
   |               -----------------------    ^^^^^^ creates a temporary value which is freed while still in use
   |               |
   |               type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement
error: aborting due to previous error

For more information about this error, try `rustc --explain E0716`.
------------------------------------------
