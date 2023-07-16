plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:e58f560dd5e72b61cf9aeebf432d862b23ac76a8)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
........................................................................................ 14520/14538
..................
failures:

---- [ui] tests/ui/argument-suggestions/issue-108242-semicolon-recovery.rs stdout ----


- error: expected one of `)`, `,`, `.`, `?`, or an operator, found `foo`
-   --> $DIR/issue-108242-semicolon-recovery.rs:4:5
- LL |     foo(;
-    |          -
-    |          |
-    |          |
-    |          expected one of `)`, `,`, `.`, `?`, or an operator
-    |          help: missing `,`
- LL |     foo(;
- 
12 error: mismatched closing delimiter: `}`
13   --> $DIR/issue-108242-semicolon-recovery.rs:4:8
14    |
---
To only update this specific test, also pass `--test-args argument-suggestions/issue-108242-semicolon-recovery.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/argument-suggestions/issue-108242-semicolon-recovery.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/argument-suggestions/issue-108242-semicolon-recovery" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/argument-suggestions/issue-108242-semicolon-recovery/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/argument-suggestions/issue-108242-semicolon-recovery.rs:4:8
   |
LL | fn main() {
   |           - closing delimiter possibly meant for this
   |           - closing delimiter possibly meant for this
LL |     foo(; //~ ERROR this function takes 0 arguments but 2 arguments were supplied
LL |     foo(; //~ ERROR this function takes 0 arguments but 1 argument was supplied
   |        ^ unclosed delimiter
LL |     //~^ ERROR expected one of
LL | } //~ ERROR mismatched closing delimiter
   | ^ mismatched closing delimiter
error: aborting due to previous error
------------------------------------------


