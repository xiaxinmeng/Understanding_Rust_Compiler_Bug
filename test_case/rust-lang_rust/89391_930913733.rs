plain
.................................................................................................... 5500/12219
.................................................................................................... 5600/12219
.................................................................................................... 5700/12219
.................................................................................................... 5800/12219
.....................F.............................................................................. 5900/12219
.........................................i....................................F............F........ 6000/12219
.......................................i............................................................ 6200/12219
...........i........................................................................................ 6300/12219
..........F...........................................................ii.ii.......i...i............. 6400/12219
.................................................................................................... 6500/12219
---
diff of stderr:

2   --> $DIR/array-break-length.rs:3:17
3    |
4 LL |         |_: [_; break]| {}
-    |                 ^^^^^ cannot `break` outside of a loop
+    |                 |
+    |                 |
+    |                 cannot `break` outside of a loop
+    |                 help: consider removing the: `break`
6 
7 error[E0268]: `continue` outside of a loop

9    |
9    |
10 LL |         |_: [_; continue]| {}
-    |                 ^^^^^^^^ cannot `continue` outside of a loop
+    |                 |
+    |                 |
+    |                 cannot `continue` outside of a loop
+    |                 help: consider removing the: `continue`
13 error: aborting due to 2 previous errors
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/array-break-length/array-break-length.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args array-slice-vec/array-break-length.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/array-slice-vec/array-break-length.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/array-break-length" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/array-slice-vec/array-break-length/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0268]: `break` outside of a loop
   |
   |
LL |         |_: [_; break]| {} //~ ERROR: `break` outside of a loop
   |                 |
   |                 |
   |                 cannot `break` outside of a loop
   |                 help: consider removing the: `break`

error[E0268]: `continue` outside of a loop
   |
   |
LL |         |_: [_; continue]| {} //~ ERROR: `continue` outside of a loop
   |                 |
   |                 |
   |                 cannot `continue` outside of a loop
   |                 help: consider removing the: `continue`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0268`.

---
diff of stderr:

13   --> $DIR/break-outside-loop.rs:10:15
14    |
15 LL |     let pth = break;
-    |               ^^^^^ cannot `break` outside of a loop
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |               |
+    |               |
+    |               cannot `break` outside of a loop
+    |               help: consider removing the: `break`
17 
18 error[E0268]: `continue` outside of a loop

20    |
21 LL |     if cond() { continue }
21 LL |     if cond() { continue }
-    |                 ^^^^^^^^ cannot `continue` outside of a loop
+    |                 |
+    |                 |
+    |                 cannot `continue` outside of a loop
+    |                 help: consider removing the: `continue`
23 
24 error[E0267]: `break` inside of a closure

42   --> $DIR/break-outside-loop.rs:24:25
43    |
43    |
44 LL |     let unconstrained = break;
-    |                         ^^^^^ cannot `break` outside of a loop
+    |                         |
+    |                         |
+    |                         cannot `break` outside of a loop
+    |                         help: consider removing the: `break`
46 
47 error[E0267]: `break` inside of a closure


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/break-outside-loop/break-outside-loop.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/break-outside-loop/break-outside-loop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args break-outside-loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/break-outside-loop.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/break-outside-loop" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/break-outside-loop/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0767]: use of unreachable label `'lab`
   |
   |
LL |     'lab: loop {
   |     ---- unreachable label defined here
LL |         || {
LL |             break 'lab;
   |                   ^^^^ unreachable label `'lab`
   |
   = note: labels are unreachable through functions, closures, async blocks and modules

error[E0268]: `break` outside of a loop
   |
   |
LL |     let pth = break; //~ ERROR: `break` outside of a loop
   |               |
   |               |
   |               cannot `break` outside of a loop
   |               help: consider removing the: `break`

error[E0268]: `continue` outside of a loop
   |
   |
LL |     if cond() { continue } //~ ERROR: `continue` outside of a loop
   |                 |
   |                 |
   |                 cannot `continue` outside of a loop
   |                 help: consider removing the: `continue`

error[E0267]: `break` inside of a closure
   |
LL |         foo(|| {
   |             -- enclosing closure
   |             -- enclosing closure
LL |             if cond() { break } //~ ERROR: `break` inside of a closure
   |                         ^^^^^ cannot `break` inside of a closure

error[E0267]: `continue` inside of a closure
   |
LL |         foo(|| {
   |             -- enclosing closure
   |             -- enclosing closure
LL |             if cond() { break } //~ ERROR: `break` inside of a closure
LL |             if cond() { continue } //~ ERROR: `continue` inside of a closure
   |                         ^^^^^^^^ cannot `continue` inside of a closure

error[E0268]: `break` outside of a loop
   |
   |
LL |     let unconstrained = break; //~ ERROR: `break` outside of a loop
   |                         |
   |                         |
   |                         cannot `break` outside of a loop
   |                         help: consider removing the: `break`

error[E0267]: `break` inside of a closure
   |
LL |         || {
   |         -- enclosing closure
   |         -- enclosing closure
LL |             break 'lab;
   |             ^^^^^^^^^^ cannot `break` inside of a closure
error: aborting due to 7 previous errors

Some errors have detailed explanations: E0267, E0268, E0767.
For more information about an error, try `rustc --explain E0267`.
---
diff of stderr:

2   --> $DIR/closure-array-break-length.rs:2:13
3    |
4 LL |     |_: [_; continue]| {};
-    |             ^^^^^^^^ cannot `continue` outside of a loop
+    |             |
+    |             |
+    |             cannot `continue` outside of a loop
+    |             help: consider removing the: `continue`
6 
7 error[E0268]: `continue` outside of a loop

9    |
9    |
10 LL |     while |_: [_; continue]| {} {}
-    |                   ^^^^^^^^ cannot `continue` outside of a loop
+    |                   |
+    |                   |
+    |                   cannot `continue` outside of a loop
+    |                   help: consider removing the: `continue`
12 
13 error[E0268]: `break` outside of a loop

15    |
15    |
16 LL |     while |_: [_; break]| {} {}
-    |                   ^^^^^ cannot `break` outside of a loop
+    |                   |
+    |                   |
+    |                   cannot `break` outside of a loop
+    |                   help: consider removing the: `break`
19 error: aborting due to 3 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-array-break-length/closure-array-break-length.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args closures/closure-array-break-length.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/closures/closure-array-break-length.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-array-break-length" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/closures/closure-array-break-length/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0268]: `continue` outside of a loop
   |
   |
LL |     |_: [_; continue]| {}; //~ ERROR: `continue` outside of a loop
   |             |
   |             |
   |             cannot `continue` outside of a loop
   |             help: consider removing the: `continue`

error[E0268]: `continue` outside of a loop
   |
   |
LL |     while |_: [_; continue]| {} {} //~ ERROR: `continue` outside of a loop
   |                   |
   |                   |
   |                   cannot `continue` outside of a loop
   |                   help: consider removing the: `continue`

error[E0268]: `break` outside of a loop
   |
   |
LL |     while |_: [_; break]| {} {} //~ ERROR: `break` outside of a loop
   |                   |
   |                   |
   |                   cannot `break` outside of a loop
   |                   help: consider removing the: `break`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0268`.

---
To only update this specific test, also pass `--test-args error-codes/E0697.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0697.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0697" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0697/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0697]: closures cannot be static
  --> /checkout/src/test/ui/error-codes/E0697.rs:2:5
   |
LL |     static || {}; //~ ERROR E0697
   |     ^^^^^^^^^ help: consider removing the: `static`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0697`.

---

2   --> $DIR/issue-28105.rs:4:5
3    |
4 LL |     continue
-    |     ^^^^^^^^ cannot `continue` outside of a loop
+    |     |
+    |     |
+    |     cannot `continue` outside of a loop
+    |     help: consider removing the: `continue`
6 
7 error[E0268]: `break` outside of a loop

9    |
10 LL |     break
10 LL |     break
-    |     ^^^^^ cannot `break` outside of a loop
+    |     |
+    |     |
+    |     cannot `break` outside of a loop
+    |     help: consider removing the: `break`
13 error: aborting due to 2 previous errors
14 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28105/issue-28105.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-28105.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-28105.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28105" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-28105/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0268]: `continue` outside of a loop
   |
   |
LL |     continue //~ ERROR `continue` outside of a loop
   |     |
   |     |
   |     cannot `continue` outside of a loop
   |     help: consider removing the: `continue`

error[E0268]: `break` outside of a loop
   |
   |
LL |     break //~ ERROR `break` outside of a loop
   |     |
   |     |
   |     cannot `break` outside of a loop
   |     help: consider removing the: `break`
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0268`.

---
diff of stderr:

2   --> $DIR/issue-43162.rs:3:5
3    |
4 LL |     break true;
-    |     ^^^^^^^^^^ cannot `break` outside of a loop
+    |     |
+    |     |
+    |     cannot `break` outside of a loop
+    |     help: consider removing the: `break`
6 
7 error[E0268]: `break` outside of a loop

9    |
10 LL |     break {};
10 LL |     break {};
-    |     ^^^^^^^^ cannot `break` outside of a loop
+    |     |
+    |     |
+    |     cannot `break` outside of a loop
+    |     help: consider removing the: `break`
13 error[E0308]: mismatched types
14   --> $DIR/issue-43162.rs:1:13



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43162/issue-43162.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-43162.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-43162.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43162" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-43162/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0268]: `break` outside of a loop
   |
   |
LL |     break true; //~ ERROR E0268
   |     |
   |     |
   |     cannot `break` outside of a loop
   |     help: consider removing the: `break`

error[E0268]: `break` outside of a loop
   |
   |
LL |     break {}; //~ ERROR E0268
   |     |
   |     |
   |     cannot `break` outside of a loop
   |     help: consider removing the: `break`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/issues/issue-43162.rs:1:13
   |
LL | fn foo() -> bool {
LL | fn foo() -> bool {
   |    ---      ^^^^ expected `bool`, found `()`
   |    |
   |    implicitly returns `()` as its body has no tail or `return` expression
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0268, E0308.
For more information about an error, try `rustc --explain E0268`.
---
diff of stderr:

2   --> $DIR/issue-50581.rs:2:14
3    |
4 LL |     |_: [u8; break]| ();
-    |              ^^^^^ cannot `break` outside of a loop
+    |              |
+    |              |
+    |              cannot `break` outside of a loop
+    |              help: consider removing the: `break`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50581/issue-50581.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-50581.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50581.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50581" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50581/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0268]: `break` outside of a loop
   |
   |
LL |     |_: [u8; break]| (); //~ ERROR [E0268]
   |              |
   |              |
   |              cannot `break` outside of a loop
   |              help: consider removing the: `break`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0268`.

---
diff of stderr:

8   --> $DIR/issue-50576.rs:2:17
9    |
10 LL |     |bool: [u8; break 'L]| 0;
-    |                 ^^^^^^^^ cannot `break` outside of a loop
+    |                 |
+    |                 |
+    |                 cannot `break` outside of a loop
+    |                 help: consider removing the: `break`
12 
13 error[E0268]: `break` outside of a loop

15    |
15    |
16 LL |     Vec::<[u8; break]>::new();
-    |                ^^^^^ cannot `break` outside of a loop
+    |                |
+    |                |
+    |                cannot `break` outside of a loop
+    |                help: consider removing the: `break`
19 error: aborting due to 3 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50576/issue-50576.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-50576.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-50576.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50576" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-50576/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0426]: use of undeclared label `'L`
  --> /checkout/src/test/ui/issues/issue-50576.rs:2:23
   |
LL |     |bool: [u8; break 'L]| 0;
   |                       ^^ undeclared label `'L`

error[E0268]: `break` outside of a loop
   |
   |
LL |     |bool: [u8; break 'L]| 0;
   |                 |
   |                 |
   |                 cannot `break` outside of a loop
   |                 help: consider removing the: `break`

error[E0268]: `break` outside of a loop
   |
   |
LL |     Vec::<[u8; break]>::new(); //~ ERROR [E0268]
   |                |
   |                |
   |                cannot `break` outside of a loop
   |                help: consider removing the: `break`
error: aborting due to 3 previous errors

Some errors have detailed explanations: E0268, E0426.
For more information about an error, try `rustc --explain E0268`.
---

2   --> $DIR/issue-83048.rs:4:5
3    |
4 LL |     break;
-    |     ^^^^^ cannot `break` outside of a loop
+    |     |
+    |     |
+    |     cannot `break` outside of a loop
+    |     help: consider removing the: `break`
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-83048/issue-83048.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-83048.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-83048.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-83048" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "unpretty=thir-tree" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-83048/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0268]: `break` outside of a loop
   |
   |
LL |     break; //~ ERROR: `break` outside of a loop [E0268]
   |     |
   |     |
   |     cannot `break` outside of a loop
   |     help: consider removing the: `break`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0268`.

---
To only update this specific test, also pass `--test-args static/static-closures.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/static/static-closures.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Zemit-future-incompat-report" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-closures" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-closures/auxiliary"
------------------------------------------

------------------------------------------
stderr:
---
test result: FAILED. 12094 passed; 10 failed; 115 ignored; 0 measured; 0 filtered out; finished in 128.45s



command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--suite" "ui" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-10/bin/FileCheck" "--nodejs" "/usr/bin/node" "--host-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Cdebuginfo=0  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python3" "--lldb-python" "/usr/bin/python3" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "10.0.0" "--llvm-components" "aarch64 aarch64asmparser aarch64codegen aarch64desc aarch64disassembler aarch64info aarch64utils aggressiveinstcombine all all-targets amdgpu amdgpuasmparser amdgpucodegen amdgpudesc amdgpudisassembler amdgpuinfo amdgpuutils analysis arm armasmparser armcodegen armdesc armdisassembler arminfo armutils asmparser asmprinter avr avrasmparser avrcodegen avrdesc avrdisassembler avrinfo binaryformat bitreader bitstreamreader bitwriter bpf bpfasmparser bpfcodegen bpfdesc bpfdisassembler bpfinfo cfguard codegen core coroutines coverage debuginfocodeview debuginfodwarf debuginfogsym debuginfomsf debuginfopdb demangle dlltooldriver dwarflinker engine executionengine frontendopenmp fuzzmutate globalisel hexagon hexagonasmparser hexagoncodegen hexagondesc hexagondisassembler hexagoninfo instcombine instrumentation interpreter ipo irreader jitlink lanai lanaiasmparser lanaicodegen lanaidesc lanaidisassembler lanaiinfo libdriver lineeditor linker lto mc mca mcdisassembler mcjit mcparser mips mipsasmparser mipscodegen mipsdesc mipsdisassembler mipsinfo mirparser msp430 msp430asmparser msp430codegen msp430desc msp430disassembler msp430info native nativecodegen nvptx nvptxcodegen nvptxdesc nvptxinfo objcarcopts object objectyaml option orcerror orcjit passes perfjitevents powerpc powerpcasmparser powerpccodegen powerpcdesc powerpcdisassembler powerpcinfo profiledata remarks riscv riscvasmparser riscvcodegen riscvdesc riscvdisassembler riscvinfo riscvutils runtimedyld scalaropts selectiondag sparc sparcasmparser sparccodegen sparcdesc sparcdisassembler sparcinfo support symbolize systemz systemzasmparser systemzcodegen systemzdesc systemzdisassembler systemzinfo tablegen target textapi transformutils vectorize webassembly webassemblyasmparser webassemblycodegen webassemblydesc webassemblydisassembler webassemblyinfo windowsmanifest x86 x86asmparser x86codegen x86desc x86disassembler x86info x86utils xcore xcorecodegen xcoredesc xcoredisassembler xcoreinfo xray" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--channel" "nightly" "--color" "always"


Build completed unsuccessfully in 0:13:04
