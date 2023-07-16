plain
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/ctfe-labelled-loop.rs:6:5
+   --> $DIR/ctfe-labelled-loop.rs:7:12
3    |
- LL | /     'mylabel: loop {
- LL | |         if i > n {
- LL | |             break 'mylabel
- LL | |         }
- LL | |         i += 1;
- LL | |     }
-    | |_____^ exceeded interpreter step limit (see `#[const_eval_limit]`)
+ LL |         if i > n {
+    |            ^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
11    |
12 note: inside `labelled_loop`
-   --> $DIR/ctfe-labelled-loop.rs:6:5
+   --> $DIR/ctfe-labelled-loop.rs:7:12
14    |
- LL | /     'mylabel: loop {
- LL | |         if i > n {
- LL | |             break 'mylabel
- LL | |         }
- LL | |         i += 1;
- LL | |     }
-    | |_____^
+ LL |         if i > n {
22 note: inside `X`
23   --> $DIR/ctfe-labelled-loop.rs:15:16
24    |

---
To only update this specific test, also pass `--test-args consts/const-eval/stable-metric/ctfe-labelled-loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/stable-metric/ctfe-labelled-loop.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/stable-metric/ctfe-labelled-loop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/stable-metric/ctfe-labelled-loop/auxiliary" "-Z" "tiny-const-eval-limit"
stdout: none
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/stable-metric/ctfe-labelled-loop.rs:7:12
   |
   |
LL |         if i > n {
   |            ^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
   |
note: inside `labelled_loop`
  --> fake-test-src-base/consts/const-eval/stable-metric/ctfe-labelled-loop.rs:7:12
   |
LL |         if i > n {
note: inside `X`
  --> fake-test-src-base/consts/const-eval/stable-metric/ctfe-labelled-loop.rs:15:16
   |
LL | const X: u32 = labelled_loop(19);
---
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/ctfe-simple-loop.rs:5:5
+   --> $DIR/ctfe-simple-loop.rs:5:11
3    |
- LL | /     while index < n {
- LL | |         index = index + 1;
- LL | |     }
-    | |_____^ exceeded interpreter step limit (see `#[const_eval_limit]`)
+ LL |     while index < n {
+    |           ^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
9 note: inside `simple_loop`
-   --> $DIR/ctfe-simple-loop.rs:5:5
+   --> $DIR/ctfe-simple-loop.rs:5:11
11    |
11    |
- LL | /     while index < n {
- LL | |         index = index + 1;
- LL | |     }
-    | |_____^
+ LL |     while index < n {
16 note: inside `X`
17   --> $DIR/ctfe-simple-loop.rs:11:16
18    |

---
To only update this specific test, also pass `--test-args consts/const-eval/stable-metric/ctfe-simple-loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/stable-metric/ctfe-simple-loop.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/stable-metric/ctfe-simple-loop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/stable-metric/ctfe-simple-loop/auxiliary" "-Z" "tiny-const-eval-limit"
stdout: none
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/stable-metric/ctfe-simple-loop.rs:5:11
   |
   |
LL |     while index < n { //~ ERROR evaluation of constant value failed [E0080]
   |           ^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
note: inside `simple_loop`
  --> fake-test-src-base/consts/const-eval/stable-metric/ctfe-simple-loop.rs:5:11
   |
   |
LL |     while index < n { //~ ERROR evaluation of constant value failed [E0080]
note: inside `X`
  --> fake-test-src-base/consts/const-eval/stable-metric/ctfe-simple-loop.rs:11:16
   |
LL | const X: u32 = simple_loop(19);
---
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/const_eval_limit_reached.rs:6:5
+   --> $DIR/const_eval_limit_reached.rs:6:11
3    |
- LL | /     while x != 1000 {
- LL | |
- LL | |         x += 1;
- LL | |     }
-    | |_____^ exceeded interpreter step limit (see `#[const_eval_limit]`)
+ LL |     while x != 1000 {
+    |           ^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached/const_eval_limit_reached.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const_limit/const_eval_limit_reached.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const_limit/const_eval_limit_reached.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const_limit/const_eval_limit_reached/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const_limit/const_eval_limit_reached.rs:6:11
   |
LL |     while x != 1000 {
LL |     while x != 1000 {
   |           ^^^^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/infinite_loop.rs:6:9
+   --> $DIR/infinite_loop.rs:6:15
3    |
- LL | /         while n != 0 {
- LL | |
- LL | |             n = if n % 2 == 0 { n/2 } else { 3*n + 1 };
- LL | |         }
-    | |_________^ exceeded interpreter step limit (see `#[const_eval_limit]`)
+ LL |         while n != 0 {
+    |               ^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/infinite_loop/infinite_loop.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/infinite_loop.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/infinite_loop.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/infinite_loop" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/infinite_loop/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/infinite_loop.rs:6:15
   |
LL |         while n != 0 {
LL |         while n != 0 {
   |               ^^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
---
1 error[E0080]: evaluation of constant value failed
-   --> $DIR/issue-52475.rs:5:9
+   --> $DIR/issue-52475.rs:5:15
3    |
- LL | /         while n < 5 {
- LL | |             n = (n + 1) % 5;
- LL | |             x = &0; // Materialize a new AllocId
- LL | |         }
-    | |_________^ exceeded interpreter step limit (see `#[const_eval_limit]`)
+ LL |         while n < 5 {
+    |               ^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
10 error: aborting due to previous error
11 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-52475/issue-52475.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args consts/const-eval/issue-52475.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/consts/const-eval/issue-52475.rs" "-Zthreads=1" "--sysroot" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-52475" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/const-eval/issue-52475/auxiliary"
stdout: none
error[E0080]: evaluation of constant value failed
  --> fake-test-src-base/consts/const-eval/issue-52475.rs:5:15
   |
   |
LL |         while n < 5 { //~ ERROR evaluation of constant value failed [E0080]
   |               ^^^^^ exceeded interpreter step limit (see `#[const_eval_limit]`)
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
