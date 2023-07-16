plain

+ error[E0308]: mismatched types
+   --> $DIR/issue-3044.rs:3:35
+    |
+ LL |       needlesArr.iter().fold(|x, y| {
+ LL | |     });
+    | |_____^ expected closure, found `()`
+    |
+    = note: expected closure `[closure@$DIR/issue-3044.rs:3:28: 4:6]`
+    = note: expected closure `[closure@$DIR/issue-3044.rs:3:28: 4:6]`
+             found unit type `()`
+ help: consider returning local binding `x`
+    |
+ LL ~     needlesArr.iter().fold(|x, y| {
+ LL +         x
+ LL +     
+ 
1 error[E0061]: this function takes 2 arguments but 1 argument was supplied
2   --> $DIR/issue-3044.rs:3:23
3    |
3    |

4 LL |       needlesArr.iter().fold(|x, y| {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
5    |  _______________________^^^^-
- LL | |         x
7 LL | |     });
8    | |______- an argument is missing

15 help: provide the argument
16    |
16    |
17 LL ~     needlesArr.iter().fold(|x, y| {
- LL +         x
19 LL ~     }, /* value */);
21 

- error: aborting due to previous error
+ error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args issues/issue-3044.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-3044.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3044" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-3044/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-3044.rs:3:35
   |
   |
LL |       needlesArr.iter().fold(|x, y| {
LL | |     });
   | |_____^ expected closure, found `()`
   |
   |
   = note: expected closure `[closure@/checkout/src/test/ui/issues/issue-3044.rs:3:28: 4:6]`
help: consider returning local binding `x`
   |
   |
LL ~     needlesArr.iter().fold(|x, y| {
LL +         x
LL +     

error[E0061]: this function takes 2 arguments but 1 argument was supplied
  --> /checkout/src/test/ui/issues/issue-3044.rs:3:23
   |
   |
LL |       needlesArr.iter().fold(|x, y| {
   |  _______________________^^^^-
LL | |     });
   | |______- an argument is missing
note: associated function defined here
  --> /checkout/library/core/src/iter/traits/iterator.rs:2407:8
   |
   |
LL |     fn fold<B, F>(mut self, init: B, mut f: F) -> B
help: provide the argument
   |
   |
LL ~     needlesArr.iter().fold(|x, y| {
LL ~     }, /* value */);

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0061, E0308.
