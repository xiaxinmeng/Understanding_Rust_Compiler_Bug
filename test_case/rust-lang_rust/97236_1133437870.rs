plain

---- [ui] src/test/ui/lifetimes/issue-97193.rs stdout ----
diff of stderr:

1 error: incorrect function inside `extern` block
-   --> $DIR/issue-97183.rs:2:8
3    |
4 LL |   extern "C" {
4 LL |   extern "C" {
5    |   ---------- `extern` blocks define existing foreign functions and functions inside of them cannot have a body

17    = note: for more information, visit https://doc.rust-lang.org/std/keyword.extern.html
19 error: `self` parameter is only allowed in associated functions
-   --> $DIR/issue-97183.rs:2:10
+   --> $DIR/issue-97193.rs:2:10
21    |
---
To only update this specific test, also pass `--test-args lifetimes/issue-97193.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lifetimes/issue-97193.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-97193" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lifetimes/issue-97193/auxiliary"
stdout: none
--- stderr -------------------------------
error: incorrect function inside `extern` block
   |
LL |   extern "C" {
LL |   extern "C" {
   |   ---------- `extern` blocks define existing foreign functions and functions inside of them cannot have a body
LL |       fn a(&mut self) {
   |  ________^____________-
   | |        cannot have a body
   | |        cannot have a body
LL | |         //~^ ERROR incorrect function inside `extern` block
LL | |         //~| ERROR `self` parameter is only allowed in associated functions
LL | |         fn b(buf: &Self) {}
LL | |     }
   | |_____- help: remove the invalid body: `;`
   |
   = help: you might have meant to write a function accessible through FFI, which can be done by writing `extern fn` outside of the `extern` block
   = note: for more information, visit https://doc.rust-lang.org/std/keyword.extern.html
error: `self` parameter is only allowed in associated functions
  --> /checkout/src/test/ui/lifetimes/issue-97193.rs:2:10
   |
LL |     fn a(&mut self) {
LL |     fn a(&mut self) {
   |          ^^^^^^^^^ not semantically valid as function parameter
   |
   = note: associated functions are those in `impl` or `trait` definitions
error: aborting due to 2 previous errors
------------------------------------------


