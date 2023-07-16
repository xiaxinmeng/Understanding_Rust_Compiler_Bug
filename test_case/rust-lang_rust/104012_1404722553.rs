plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:8c19d39c6d9d7e831f6e393b2a871216393a5761)
Complete job name: PR (x86_64-gnu-llvm-13, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-13
---
........................................................................................ 9240/14249
....................................i........................................i.......... 9328/14249
..........................................................i............................. 9416/14249
........................................................................................ 9504/14249
...................F.F.................................................................. 9592/14249
........................................................................................ 9768/14249
.............i.......................................................................... 9856/14249
........................................................................................ 9944/14249
........................................................................................ 10032/14249
---
..........................................................iii........................... 14168/14249
.................................................................................
failures:

---- [ui] tests/ui/parser/issue-68987-unmatch-issue-2.rs stdout ----

1 error: unexpected closing delimiter: `}`
-   --> $DIR/issue-68987-unmatch-issue-2.rs:12:1
+   --> $DIR/issue-68987-unmatch-issue-2.rs:14:1
---
12 error: mismatched closing delimiter: `)`
-   --> $DIR/issue-68987-unmatch-issue-2.rs:1:32
+   --> $DIR/issue-68987-unmatch-issue-2.rs:3:32
14    |
15 LL | async fn obstest() -> Result<> {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-68987-unmatch-issue-2/issue-68987-unmatch-issue-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-68987-unmatch-issue-2/issue-68987-unmatch-issue-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-68987-unmatch-issue-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/issue-68987-unmatch-issue-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-68987-unmatch-issue-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-68987-unmatch-issue-2/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/parser/issue-68987-unmatch-issue-2.rs:14:1
   |
LL |     } else {
   |            - this opening brace...
   |            - this opening brace...
LL |
LL |     }
   |     - ...matches this closing brace
LL | } //~ ERROR unexpected closing delimiter

error: mismatched closing delimiter: `)`
  --> fake-test-src-base/parser/issue-68987-unmatch-issue-2.rs:3:32
   |
   |
LL | async fn obstest() -> Result<> {
   |                                ^ unclosed delimiter
LL |     let obs_connect = || -> Result<(), MyError) { //~ ERROR mismatched closing delimiter

error: aborting due to 2 previous errors
------------------------------------------



Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---- [ui] tests/ui/parser/issue-68987-unmatch-issue-3.rs stdout ----

1 error: unexpected closing delimiter: `}`
-   --> $DIR/issue-68987-unmatch-issue-3.rs:7:1
+   --> $DIR/issue-68987-unmatch-issue-3.rs:9:1
+   --> $DIR/issue-68987-unmatch-issue-3.rs:9:1
3    |
4 LL | fn f(i: u32, j: u32) {
5    |                      - this delimiter might not be properly closed...
10    | ^ unexpected closing delimiter
11 
12 error: mismatched closing delimiter: `)`
-   --> $DIR/issue-68987-unmatch-issue-3.rs:4:19
-   --> $DIR/issue-68987-unmatch-issue-3.rs:4:19
+   --> $DIR/issue-68987-unmatch-issue-3.rs:6:19
14    |
15 LL |     while cnt < j {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-68987-unmatch-issue-3/issue-68987-unmatch-issue-3.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-68987-unmatch-issue-3/issue-68987-unmatch-issue-3.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args parser/issue-68987-unmatch-issue-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/parser/issue-68987-unmatch-issue-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-68987-unmatch-issue-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issue-68987-unmatch-issue-3/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/parser/issue-68987-unmatch-issue-3.rs:9:1
   |
   |
LL | fn f(i: u32, j: u32) {
   |                      - this delimiter might not be properly closed...
LL |     }
LL |     }
   |     - ...as it matches this but it has different indentation
LL | } //~ ERROR unexpected closing delimiter

error: mismatched closing delimiter: `)`
  --> fake-test-src-base/parser/issue-68987-unmatch-issue-3.rs:6:19
   |
   |
LL |     while cnt < j {
   |                   ^ unclosed delimiter
LL |         write!&mut res, " "); //~ ERROR mismatched closing delimiter

error: aborting due to 2 previous errors
------------------------------------------

