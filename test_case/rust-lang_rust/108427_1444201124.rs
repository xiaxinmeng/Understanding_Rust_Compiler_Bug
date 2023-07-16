plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:697bea7ddceb6696743da8f159f268aef8bfb3c6)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---

---- [ui] tests/ui/let-else/let-else-brace-before-else.rs stdout ----
diff of stderr:

9 LL |     let Some(1) = ({ Some(1) }) else {
11 
11 
- error: right curly brace `}` before `else` in a `let...else` statement not allowed
-   --> $DIR/let-else-brace-before-else.rs:10:40
+ error: `loop...else` loops are not supported
14    |
14    |
- LL |     let Some(1) = loop { break Some(1) } else {
-    |                                        ^
+ LL |       let Some(1) = loop { break Some(1) } else {
+    |  ___________________----___________________^
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    | |                   `else` is attached to this loop
+ LL | |
+ LL | |         return;
+ LL | |         return;
+ LL | |     };
+    | |_____^
17    |
- help: wrap the expression in parentheses
-    |
- LL |     let Some(1) = (loop { break Some(1) }) else {
-    |                   +                      +
+    = note: consider moving this `else` clause to a separate `if` statement and use a `bool` variable to control if it should run
22 
23 error: right curly brace `}` before `else` in a `let...else` statement not allowed


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/let-else/let-else-brace-before-else/let-else-brace-before-else.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/let-else/let-else-brace-before-else/let-else-brace-before-else.stderr
diff of fixed:

7         //~^ ERROR right curly brace `}` before `else` in a `let...else` statement not allowed
9     };
9     };
-     let Some(1) = (loop { break Some(1) }) else {
+     let Some(1) = loop { break Some(1) } else {
11         //~^ ERROR right curly brace `}` before `else` in a `let...else` statement not allowed
13     };


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/let-else/let-else-brace-before-else/let-else-brace-before-else.fixed
To only update this specific test, also pass `--test-args let-else/let-else-brace-before-else.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/let-else/let-else-brace-before-else.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/let-else/let-else-brace-before-else" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/let-else/let-else-brace-before-else/auxiliary"
stdout: none
--- stderr -------------------------------
error: right curly brace `}` before `else` in a `let...else` statement not allowed
  --> fake-test-src-base/let-else/let-else-brace-before-else.rs:6:29
   |
LL |     let Some(1) = { Some(1) } else {
   |
help: wrap the expression in parentheses
   |
   |
LL |     let Some(1) = ({ Some(1) }) else {


error: `loop...else` loops are not supported
  --> fake-test-src-base/let-else/let-else-brace-before-else.rs:10:42
   |
LL |       let Some(1) = loop { break Some(1) } else {
   |  ___________________----___________________^
   | |                   `else` is attached to this loop
   | |                   `else` is attached to this loop
LL | |         //~^ ERROR right curly brace `}` before `else` in a `let...else` statement not allowed
LL | |         return;
LL | |     };
   |
   |
   = note: consider moving this `else` clause to a separate `if` statement and use a `bool` variable to control if it should run

error: right curly brace `}` before `else` in a `let...else` statement not allowed
  --> fake-test-src-base/let-else/let-else-brace-before-else.rs:14:34
   |
LL |     let 2 = 1 + match 1 { n => n } else {
   |
help: wrap the expression in parentheses
   |
   |
LL |     let 2 = 1 + (match 1 { n => n }) else {


error: right curly brace `}` before `else` in a `let...else` statement not allowed
  --> fake-test-src-base/let-else/let-else-brace-before-else.rs:18:40
   |
LL |     let Some(1) = unsafe { unsafe_fn() } else {
   |
help: wrap the expression in parentheses
   |
   |
LL |     let Some(1) = (unsafe { unsafe_fn() }) else {

error: aborting due to 4 previous errors
------------------------------------------

