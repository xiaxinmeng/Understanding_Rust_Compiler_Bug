plain

---- [ui] src/test/ui/derives/issue-97343.rs stdout ----
diff of stderr:

6 LL | pub struct Irrelevant<Irrelevant> {
7    |                       ^^^^^^^^^^ type argument not allowed
8    |
+ note: type parameter `Irrelevant` defined here
+    |
+    |
+ LL | pub struct Irrelevant<Irrelevant> {
9    = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)
10 
11 error: aborting due to previous error

---
To only update this specific test, also pass `--test-args derives/issue-97343.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/derives/issue-97343.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-97343" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/derives/issue-97343/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0109]: type arguments are not allowed for this type
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL | #[derive(Debug)]
   |          ----- in this derive macro expansion
   |          ----- in this derive macro expansion
LL | pub struct Irrelevant<Irrelevant> { //~ ERROR type arguments are not allowed for this type
   |                       ^^^^^^^^^^ type argument not allowed
   |
note: type parameter `Irrelevant` defined here
   |
   |
LL | pub struct Irrelevant<Irrelevant> { //~ ERROR type arguments are not allowed for this type
   = note: this error originates in the derive macro `Debug` (in Nightly builds, run with -Z macro-backtrace for more info)

error: aborting due to previous error

