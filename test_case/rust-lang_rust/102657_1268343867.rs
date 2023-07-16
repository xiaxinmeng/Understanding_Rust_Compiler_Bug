plain
Successfully built d8a0e8d5b64c
Successfully tagged rust-ci:latest
Built container sha256:d8a0e8d5b64cc876dfed5cc34203e0b14866c62e9a0f511fc1bd73209e2b3bc0
Uploading finished image to https://ci-caches.rust-lang.org/docker/3ba9d538a45014cf9c069a7f7b39b17975213bb4ad3cb92953bcf42e2feac7246274891423ce6ec10459580375996bc5323b9b2e1eaa3f9fe83d7d3a2f2335cf
upload failed: - to s3://rust-lang-ci-sccache2/docker/3ba9d538a45014cf9c069a7f7b39b17975213bb4ad3cb92953bcf42e2feac7246274891423ce6ec10459580375996bc5323b9b2e1eaa3f9fe83d7d3a2f2335cf Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-llvm-13]
---
........................................................................................ 13640/13644
....
failures:

---- [ui] src/test/ui/impl-trait/transmute/in-defining-scope.rs stdout ----


1 error[E0391]: cycle detected when computing type of `foo::{opaque#0}`
-   --> $DIR/in-defining-scope.rs:5:13
+   --> $DIR/in-defining-scope.rs:3:13
4 LL | fn foo() -> impl Sized {
5    |             ^^^^^^^^^^

6    |
6    |
7 note: ...which requires borrow-checking `foo`...
-   --> $DIR/in-defining-scope.rs:5:1
+   --> $DIR/in-defining-scope.rs:3:1
10 LL | fn foo() -> impl Sized {
11    | ^^^^^^^^^^^^^^^^^^^^^^


12 note: ...which requires processing `foo`...
-   --> $DIR/in-defining-scope.rs:5:1
+   --> $DIR/in-defining-scope.rs:3:1
15 LL | fn foo() -> impl Sized {
16    | ^^^^^^^^^^^^^^^^^^^^^^


17 note: ...which requires processing MIR for `foo`...
-   --> $DIR/in-defining-scope.rs:5:1
+   --> $DIR/in-defining-scope.rs:3:1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
20 LL | fn foo() -> impl Sized {
21    | ^^^^^^^^^^^^^^^^^^^^^^


22 note: ...which requires unsafety-checking `foo`...
-   --> $DIR/in-defining-scope.rs:5:1
+   --> $DIR/in-defining-scope.rs:3:1
25 LL | fn foo() -> impl Sized {
26    | ^^^^^^^^^^^^^^^^^^^^^^


27 note: ...which requires building MIR for `foo`...
-   --> $DIR/in-defining-scope.rs:5:1
+   --> $DIR/in-defining-scope.rs:3:1
30 LL | fn foo() -> impl Sized {
31    | ^^^^^^^^^^^^^^^^^^^^^^


32 note: ...which requires building THIR for `foo`...
-   --> $DIR/in-defining-scope.rs:5:1
+   --> $DIR/in-defining-scope.rs:3:1
35 LL | fn foo() -> impl Sized {
36    | ^^^^^^^^^^^^^^^^^^^^^^


37 note: ...which requires type-checking `foo`...
-   --> $DIR/in-defining-scope.rs:5:1
+   --> $DIR/in-defining-scope.rs:3:1
40 LL | fn foo() -> impl Sized {
41    | ^^^^^^^^^^^^^^^^^^^^^^


42    = note: ...which again requires computing type of `foo::{opaque#0}`, completing the cycle
43 note: cycle used when checking item types in top-level module
-   --> $DIR/in-defining-scope.rs:4:1
+   --> $DIR/in-defining-scope.rs:2:1
46 LL | / use std::mem::transmute;
47 LL | | fn foo() -> impl Sized {

53    | |____________^
53    | |____________^
54 
55 error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
-   --> $DIR/in-defining-scope.rs:8:9
+   --> $DIR/in-defining-scope.rs:6:9
57    |
58 LL |         transmute::<_, u8>(foo());


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/transmute/in-defining-scope/in-defining-scope.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/transmute/in-defining-scope/in-defining-scope.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/transmute/in-defining-scope.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/transmute/in-defining-scope.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/transmute/in-defining-scope" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/transmute/in-defining-scope/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when computing type of `foo::{opaque#0}`
  --> /checkout/src/test/ui/impl-trait/transmute/in-defining-scope.rs:3:13
LL | fn foo() -> impl Sized {
   |             ^^^^^^^^^^
   |
   |
note: ...which requires borrow-checking `foo`...
  --> /checkout/src/test/ui/impl-trait/transmute/in-defining-scope.rs:3:1
LL | fn foo() -> impl Sized {
   | ^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing `foo`...
  --> /checkout/src/test/ui/impl-trait/transmute/in-defining-scope.rs:3:1
LL | fn foo() -> impl Sized {
   | ^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires processing MIR for `foo`...
  --> /checkout/src/test/ui/impl-trait/transmute/in-defining-scope.rs:3:1
LL | fn foo() -> impl Sized {
   | ^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires unsafety-checking `foo`...
  --> /checkout/src/test/ui/impl-trait/transmute/in-defining-scope.rs:3:1
LL | fn foo() -> impl Sized {
   | ^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building MIR for `foo`...
  --> /checkout/src/test/ui/impl-trait/transmute/in-defining-scope.rs:3:1
LL | fn foo() -> impl Sized {
   | ^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires building THIR for `foo`...
  --> /checkout/src/test/ui/impl-trait/transmute/in-defining-scope.rs:3:1
LL | fn foo() -> impl Sized {
   | ^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^
note: ...which requires type-checking `foo`...
  --> /checkout/src/test/ui/impl-trait/transmute/in-defining-scope.rs:3:1
LL | fn foo() -> impl Sized {
   | ^^^^^^^^^^^^^^^^^^^^^^
   | ^^^^^^^^^^^^^^^^^^^^^^
   = note: ...which again requires computing type of `foo::{opaque#0}`, completing the cycle
note: cycle used when checking item types in top-level module
  --> /checkout/src/test/ui/impl-trait/transmute/in-defining-scope.rs:2:1
LL | / use std::mem::transmute;
LL | | fn foo() -> impl Sized {
LL | | fn foo() -> impl Sized {
LL | |     //~^ ERROR cycle detected when computing type
LL | |     unsafe {
LL | |
LL | | fn main() {}
   | |____________^


error[E0512]: cannot transmute between types of different sizes, or dependently-sized types
  --> /checkout/src/test/ui/impl-trait/transmute/in-defining-scope.rs:6:9
   |
LL |         transmute::<_, u8>(foo());
   |
   |
   = note: source type: `impl Sized` (size can vary because of [type error])
   = note: target type: `u8` (8 bits)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0391, E0512.
For more information about an error, try `rustc --explain E0391`.
