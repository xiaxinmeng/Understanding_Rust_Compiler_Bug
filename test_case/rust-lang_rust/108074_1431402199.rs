plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:7ea263296c702710d7fac3c6d5bccdb2895b4e79)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
......iii............................................................................... 14432/14462
..............................
failures:

---- [ui] tests/ui/fn/fn-ptr-eq-op.rs stdout ----

121    |      +++++++++++     +++++++++++
122 
122 
123 error[E0369]: binary operation `==` cannot be applied to type `for<'a> fn(&'a ()) {foo}`
-   --> $DIR/fn-ptr-eq-op.rs:32:9
+   --> $DIR/fn-ptr-eq-op.rs:31:9
126 LL |     foo == x;
126 LL |     foo == x;
127    |     --- ^^ - fn(&())
134    |        +++++++++++     +++++++++++
135 
136 error[E0308]: mismatched types
-   --> $DIR/fn-ptr-eq-op.rs:32:12
-   --> $DIR/fn-ptr-eq-op.rs:32:12
+   --> $DIR/fn-ptr-eq-op.rs:31:12
138    |
139 LL |     foo == x;
140    |            ^ expected fn item, found fn pointer


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-ptr-eq-op/fn-ptr-eq-op.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args fn/fn-ptr-eq-op.rs`
error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/fn/fn-ptr-eq-op.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-ptr-eq-op" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/fn/fn-ptr-eq-op/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `for<'a> fn(&'a ())`
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:6:7
   |
LL |     x == y; //~ ERROR: `==` cannot be applied
   |     - ^^ - fn(&())
   |     |
   |     for<'a> fn(&'a ())
help: use parentheses to call these
   |
   |
LL |     x(/* &() */) == y(/* &() */); //~ ERROR: `==` cannot be applied


error[E0369]: binary operation `==` cannot be applied to type `for<'a> fn(&'a ())`
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:10:7
   |
LL |     x == y; //~ ERROR: `==` cannot be applied
   |     - ^^ - fn(&())
   |     |
   |     for<'a> fn(&'a ())
help: use parentheses to call these
   |
   |
LL |     x(/* &() */) == y(/* &() */); //~ ERROR: `==` cannot be applied


error[E0369]: binary operation `==` cannot be applied to type `for<'a> fn(&'a ())`
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:14:7
   |
LL |     x == y; //~ ERROR: `==` cannot be applied
   |     - ^^ - fn(&())
   |     |
   |     for<'a> fn(&'a ())
help: use parentheses to call these
   |
   |
LL |     x(/* &() */) == y(/* &() */); //~ ERROR: `==` cannot be applied
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu


error[E0369]: binary operation `==` cannot be applied to type `for<'a> fn(&'a ())`
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:18:7
   |
LL |     x == foo; //~ ERROR: `==` cannot be applied
   |     - ^^ --- fn(&())
   |     |
   |     for<'a> fn(&'a ())
help: use parentheses to call these
   |
   |
LL |     x(/* &() */) == foo(/* &() */); //~ ERROR: `==` cannot be applied


error[E0369]: binary operation `==` cannot be applied to type `for<'a> fn(&'a ())`
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:19:7
   |
LL |     y == foo; //~ ERROR: `==` cannot be applied
   |     - ^^ --- fn(&())
   |     |
   |     for<'a> fn(&'a ())
help: use parentheses to call these
   |
   |
LL |     y(/* &() */) == foo(/* &() */); //~ ERROR: `==` cannot be applied


error[E0369]: binary operation `==` cannot be applied to type `for<'a> fn(&'a ()) {foo}`
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:20:9
   |
LL |     foo == x; //~ ERROR: `==` cannot be applied
   |     --- ^^ - for<'a> fn(&'a ())
   |     |
   |     for<'a> fn(&'a ()) {foo}
help: use parentheses to call these
   |
   |
LL |     foo(/* &() */) == x(/* &() */); //~ ERROR: `==` cannot be applied

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:20:12
   |
LL |     foo == x; //~ ERROR: `==` cannot be applied
   |            ^ expected fn item, found fn pointer
   |
   = note: expected fn item `for<'a> fn(&'a ()) {foo}`
           found fn pointer `for<'a> fn(&'a ())`

error[E0369]: binary operation `==` cannot be applied to type `for<'a> fn(&'a ()) {foo}`
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:22:9
   |
LL |     foo == y; //~ ERROR: `==` cannot be applied
   |     --- ^^ - for<'a> fn(&'a ())
   |     |
   |     for<'a> fn(&'a ()) {foo}
help: use parentheses to call these
   |
   |
LL |     foo(/* &() */) == y(/* &() */); //~ ERROR: `==` cannot be applied

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:22:12
   |
LL |     foo == y; //~ ERROR: `==` cannot be applied
   |            ^ expected fn item, found fn pointer
   |
   = note: expected fn item `for<'a> fn(&'a ()) {foo}`
           found fn pointer `for<'a> fn(&'a ())`

error[E0369]: binary operation `==` cannot be applied to type `for<'a> fn(&'a ())`
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:27:7
   |
LL |     x == y; //~ ERROR: `==` cannot be applied
   |     - ^^ - fn(&())
   |     |
   |     for<'a> fn(&'a ())
help: use parentheses to call these
   |
   |
LL |     x(/* &() */) == y(/* &() */); //~ ERROR: `==` cannot be applied


error[E0369]: binary operation `==` cannot be applied to type `for<'a> fn(&'a ()) {foo}`
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:31:9
   |
LL |     foo == x; //~ ERROR: `==` cannot be applied
   |     --- ^^ - fn(&())
   |     |
   |     for<'a> fn(&'a ()) {foo}
help: use parentheses to call these
   |
   |
LL |     foo(/* &() */) == x(/* &() */); //~ ERROR: `==` cannot be applied

error[E0308]: mismatched types
error[E0308]: mismatched types
  --> fake-test-src-base/fn/fn-ptr-eq-op.rs:31:12
   |
LL |     foo == x; //~ ERROR: `==` cannot be applied
   |            ^ expected fn item, found fn pointer
   |
   = note: expected fn item `for<'a> fn(&'a ()) {foo}`
           found fn pointer `fn(&())`
error: aborting due to 12 previous errors

Some errors have detailed explanations: E0308, E0369.
For more information about an error, try `rustc --explain E0308`.
