plain

---- [ui] src/test/ui/chalkify/trait-objects.rs stdout ----
diff of stderr:

22    |     ^^^^ expected an `Fn<(i32,)>` closure, found `dyn Fn(i32) -> i32`
23    |
24    = help: the trait `Fn<(i32,)>` is not implemented for `dyn Fn(i32) -> i32`
+ help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
+    |
+ LL | fn main() where dyn Fn(i32) -> i32: Fn<(i32,)> {
25 
26 error: aborting due to 3 previous errors
27 

---
To only update this specific test, also pass `--test-args chalkify/trait-objects.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/chalkify/trait-objects.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/trait-objects" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/chalkify/trait-objects/auxiliary" "-Z" "chalk"
stdout: none
--- stderr -------------------------------
error: the type `&dyn Fn(i32) -> _` is not well-formed (chalk)
   |
   |
LL |     let f: &dyn Fn(i32) -> _ = &|x| x + x;


error[E0277]: `(i32,)` is not a tuple
   |
LL |     f(2);
LL |     f(2);
   |     ^^^^ the trait `Tuple` is not implemented for `(i32,)`
   |
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn main() where (i32,): Tuple {


error[E0277]: expected a `Fn<(i32,)>` closure, found `dyn Fn(i32) -> i32`
   |
LL |     f(2);
LL |     f(2);
   |     ^^^^ expected an `Fn<(i32,)>` closure, found `dyn Fn(i32) -> i32`
   |
   = help: the trait `Fn<(i32,)>` is not implemented for `dyn Fn(i32) -> i32`
help: consider introducing a `where` clause, but there might be an alternative better way to express this requirement
   |
LL | fn main() where dyn Fn(i32) -> i32: Fn<(i32,)> {

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0277`.
