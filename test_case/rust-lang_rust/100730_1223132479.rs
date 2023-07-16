plain
........................................................................................ 4840/13422
........................................................................................ 4928/13422
........................................................................................ 5016/13422
........................................................................................ 5104/13422
......................................................................i..F.............. 5192/13422
........................................................................................ 5368/13422
........................................................................................ 5456/13422
........................................................................................ 5544/13422
........................................................................................ 5632/13422
---
......................................i................................................. 6424/13422
........................................................................................ 6512/13422
..................................i..................................................... 6600/13422
........................................................................................ 6688/13422
...........i......F.............F..................................ii.ii........i....i.. 6776/13422
........................................................................................ 6952/13422
.................................i....i.........................................i....... 7040/13422
...........i.............i.........................................................i.... 7128/13422
........................................................................................ 7216/13422
---

---- [ui] src/test/ui/infinite/infinite-instantiation.rs stdout ----
diff of stderr:

- error: reached the recursion limit while instantiating `function::<Option<Option<Option<...>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
+ error: "reached the recursion limit while instantiating `function::<Option<Option<Option<...>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
3    |
3    |
4 LL |         function(counter - 1, t.to_option());

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-instantiation/infinite-instantiation.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args infinite/infinite-instantiation.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/infinite/infinite-instantiation.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-instantiation" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-instantiation/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error: "reached the recursion limit while instantiating `function::<Option<Option<Option<...>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL |         function(counter - 1, t.to_option());
   |
note: `function` defined here
  --> /checkout/src/test/ui/infinite/infinite-instantiation.rs:20:1
   |
   |
LL | fn function<T:ToOpt + Clone>(counter: usize, t: T) {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/infinite/infinite-instantiation/infinite-instantiation.long-type.txt'
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-22638.rs stdout ----
diff of stderr:

- error: reached the recursion limit while instantiating `A::matches::$CLOSURE`
+ error: "reached the recursion limit while instantiating `A::matches::$CLOSURE`
3    |
3    |
4 LL |         a.matches(f)

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638/issue-22638.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-22638.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-22638.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638/auxiliary"
stdout: none
--- stderr -------------------------------
error: "reached the recursion limit while instantiating `A::matches::<[closure@/checkout/...es/issue-22638.rs:44:23: 44:25]>`
   |
   |
LL |         a.matches(f)
   |
   |
note: `A::matches` defined here
   |
   |
LL |     pub fn matches<F: Fn()>(&self, f: &F) {
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-22638/issue-22638.long-type.txt'
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-37311-type-length-limit/issue-37311.rs stdout ----
diff of stderr:

- error: reached the recursion limit while instantiating `<(&(&(&(&(&(&(&(&(&(&(&(&(&(&(&(.....), ...), ...) as Foo>::recurse`
+ error: "reached the recursion limit while instantiating `<(&(&(&(&(&(&(&(&(&(&(&(&(&(&(&(.....), ...), ...) as Foo>::recurse`
3    |
3    |
4 LL |         (self, self).recurse();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37311-type-length-limit/issue-37311/issue-37311.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-37311-type-length-limit/issue-37311.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-37311-type-length-limit/issue-37311.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37311-type-length-limit/issue-37311" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37311-type-length-limit/issue-37311/auxiliary"
stdout: none
--- stderr -------------------------------
error: "reached the recursion limit while instantiating `<(&(&(&(&(&(&(&(&(&(&(&(&(&(&(&(.....), ...), ...) as Foo>::recurse`
   |
   |
LL |         (self, self).recurse(); //~ ERROR reached the recursion limit
   |
   |
note: `<T as Foo>::recurse` defined here
   |
LL |     fn recurse(&self) {
   |     ^^^^^^^^^^^^^^^^^
   |     ^^^^^^^^^^^^^^^^^
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-37311-type-length-limit/issue-37311/issue-37311.long-type.txt'
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-67552.rs stdout ----
diff of stderr:

- error: reached the recursion limit while instantiating `rec::<&mut &mut &mut &mut &mut &... &mut &mut &mut &mut &mut Empty>`
+ error: "reached the recursion limit while instantiating `rec::<&mut &mut &mut &mut &mut &... &mut &mut &mut &mut &mut Empty>`
3    |
3    |
4 LL |         rec(identity(&mut it))

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67552/issue-67552.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-67552.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-67552.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67552" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Copt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67552/auxiliary"
stdout: none
--- stderr -------------------------------
error: "reached the recursion limit while instantiating `rec::<&mut &mut &mut &mut &mut &... &mut &mut &mut &mut &mut Empty>`
   |
   |
LL |         rec(identity(&mut it))
   |
note: `rec` defined here
  --> /checkout/src/test/ui/issues/issue-67552.rs:22:1
   |
   |
LL | / fn rec<T>(mut it: T)
LL | | where
LL | |     T: Iterator,
   | |________________^
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-67552/issue-67552.long-type.txt'
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/issues/issue-8727.rs stdout ----
diff of stderr:

9    = note: `#[warn(unconditional_recursion)]` on by default
10    = help: a `loop` may express intention better if this is on purpose
11 
- error: reached the recursion limit while instantiating `generic::<Option<Option<Option<O...>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
+ error: "reached the recursion limit while instantiating `generic::<Option<Option<Option<O...>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
14    |
14    |
15 LL |     generic::<Option<T>>();

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-8727/issue-8727.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-8727.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-8727.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-8727" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-8727/auxiliary"
stdout: none
--- stderr -------------------------------
warning: function cannot return without recursing
   |
   |
LL | fn generic<T>() { //~ WARN function cannot return without recursing
   | ^^^^^^^^^^^^^^^ cannot return without recursing
LL |     generic::<Option<T>>();
   |     ---------------------- recursive call site
   = note: `#[warn(unconditional_recursion)]` on by default
   = note: `#[warn(unconditional_recursion)]` on by default
   = help: a `loop` may express intention better if this is on purpose

error: "reached the recursion limit while instantiating `generic::<Option<Option<Option<O...>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL |     generic::<Option<T>>();
   |
note: `generic` defined here
  --> /checkout/src/test/ui/issues/issue-8727.rs:7:1
   |
   |
LL | fn generic<T>() { //~ WARN function cannot return without recursing
   | ^^^^^^^^^^^^^^^
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-8727/issue-8727.long-type.txt'
error: aborting due to previous error; 1 warning emitted
------------------------------------------



---- [ui] src/test/ui/recursion/recursion.rs stdout ----
diff of stderr:

- error: reached the recursion limit while instantiating `test::<Cons<Cons<Cons<Cons<Cons<...>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
+ error: "reached the recursion limit while instantiating `test::<Cons<Cons<Cons<Cons<Cons<...>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
3    |
3    |
4 LL |     _ => {test (n-1, i+1, Cons {head:2*i+1, tail:first}, Cons{head:i*i, tail:second})}

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursion/recursion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args recursion/recursion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/recursion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-C" "overflow-checks=off" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursion/auxiliary"
stdout: none
--- stderr -------------------------------
error: "reached the recursion limit while instantiating `test::<Cons<Cons<Cons<Cons<Cons<...>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>>`
   |
   |
LL |     _ => {test (n-1, i+1, Cons {head:2*i+1, tail:first}, Cons{head:i*i, tail:second})}
   |
note: `test` defined here
  --> /checkout/src/test/ui/recursion/recursion.rs:16:1
   |
   |
LL | fn test<T:Dot> (n:isize, i:isize, first:T, second:T) ->isize {
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursion/recursion.long-type.txt'
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/type_length_limit.rs stdout ----
diff of stderr:

4 LL | pub fn drop<T>(_x: T) {}
6    |
6    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
8    = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
+    = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
9 
10 error: reached the type-length limit while instantiating `<[closure@std::rt::lang_start<()...e<()>>::call_once - shim(vtable)`
11   --> $SRC_DIR/core/src/ops/function.rs:LL:COL

13 LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
15    |
15    |
-    = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
17    = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
+    = note: the full type name has been written to '$TEST_BUILD_DIR/type_length_limit/type_length_limit.long-type.txt'
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args type_length_limit.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/type_length_limit.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Copt-level=0" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/auxiliary"
stdout: none
--- stderr -------------------------------
error: reached the type-length limit while instantiating `std::mem::drop::<Option<((((...,....., ...), ..., ...), ..., ...)>>`
   |
   |
LL | pub fn drop<T>(_x: T) {}
   |
   |
   = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.long-type.txt'

error: reached the type-length limit while instantiating `<[closure@std::rt::lang_start<()...e<()>>::call_once - shim(vtable)`
   |
   |
LL |     extern "rust-call" fn call_once(self, args: Args) -> Self::Output;
   |
   |
   = help: consider adding a `#![type_length_limit="8"]` attribute to your crate
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type_length_limit/type_length_limit.long-type.txt'
error: aborting due to 2 previous errors
------------------------------------------


