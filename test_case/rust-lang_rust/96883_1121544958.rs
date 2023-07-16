plain
diff of stderr:

12    | ^^^^^^^^^^^^^^^^^^^^^^^^^^^
13 
14 error[E0391]: cycle detected when computing type of `Foo::X`
-   --> $DIR/cycle-trait-default-type-trait.rs:4:23
16    |
16    |
17 LL | trait Foo<X = Box<dyn Foo>> {
+    |           ^^^^^^^^^^^^^^^^
19    |
19    |
20    = note: ...which immediately requires computing type of `Foo::X` again
21 note: cycle used when collecting item types in top-level module

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/cycle-trait-default-type-trait.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
To only update this specific test, also pass `--test-args cycle-trait/cycle-trait-default-type-trait.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/cycle-trait/cycle-trait-default-type-trait.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/cycle-trait/cycle-trait-default-type-trait/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `Foo::X`
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {
   |
   |
   = note: ...which immediately requires computing type of `Foo::X` again
note: cycle used when collecting item types in top-level module
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {


error[E0391]: cycle detected when computing type of `Foo::X`
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {
   |
   |
   = note: ...which immediately requires computing type of `Foo::X` again
note: cycle used when collecting item types in top-level module
   |
   |
LL | trait Foo<X = Box<dyn Foo>> {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0391`.
---
diff of stderr:

5    |                              ^^^^^^^^^^
6    |
7 note: ...which requires expanding type alias `DefaultFoo`...
-   --> $DIR/issue-34373.rs:8:19
9    |
9    |
10 LL | type DefaultFoo = Foo;
+    | ^^^^^^^^^^^^^^^^^^^^^^
+    | ^^^^^^^^^^^^^^^^^^^^^^
12    = note: ...which again requires computing type of `Foo::T`, completing the cycle
13 note: cycle used when collecting item types in top-level module


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/issue-34373.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/issue-34373.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-34373.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-34373.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-34373/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `Foo::T`
   |
   |
LL | pub struct Foo<T = Box<Trait<DefaultFoo>>>;  //~ ERROR cycle detected
   |
   |
note: ...which requires expanding type alias `DefaultFoo`...
   |
   |
LL | type DefaultFoo = Foo;
   | ^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires computing type of `Foo::T`, completing the cycle
note: cycle used when collecting item types in top-level module
   |
LL | / #![allow(warnings)]
LL | |
LL | |
LL | | trait Trait<T> {
LL | |     fn foo(_: T) {}
LL | | fn main() {
LL | | }
   | |_^

