plain
1 error: lifetime may not live long enough
-   --> $DIR/issue-55796.rs:18:9
+   --> $DIR/issue-55796.rs:16:9
3    |
4 LL | pub trait Graph<'a> {
5    |                 -- lifetime `'a` defined here

8    |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
10 error: lifetime may not live long enough
-   --> $DIR/issue-55796.rs:23:9
+   --> $DIR/issue-55796.rs:21:9
12    |
12    |
13 LL | pub trait Graph<'a> {
14    |                 -- lifetime `'a` defined here

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55796.nll/issue-55796.nll.stderr
To only update this specific test, also pass `--test-args issues/issue-55796.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-55796.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55796.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-55796.nll/auxiliary"
stdout:
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

------------------------------------------
stderr:
------------------------------------------
------------------------------------------
error: lifetime may not live long enough
  --> /checkout/src/test/ui/issues/issue-55796.rs:16:9
   |
LL | pub trait Graph<'a> {
   |                 -- lifetime `'a` defined here
...
LL |         Box::new(self.out_edges(u).map(|e| e.target()))
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/issues/issue-55796.rs:21:9
   |
   |
LL | pub trait Graph<'a> {
   |                 -- lifetime `'a` defined here
...
LL |         Box::new(self.in_edges(u).map(|e| e.target()))
   |         ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ returning this value requires that `'a` must outlive `'static`
error: aborting due to 2 previous errors


------------------------------------------
