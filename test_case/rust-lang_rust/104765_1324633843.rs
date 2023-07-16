plain

---- [ui] src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs stdout ----
diff of stderr:

30 LL | async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b {
32 
- error: aborting due to 2 previous errors
- error: aborting due to 2 previous errors
+ error[E0700]: hidden type for `impl Trait<'a>` captures lifetime that does not appear in bounds
+    |
+    |
+ LL |   async fn async_ret_impl_trait2<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a>
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
+    |                                      -- hidden type `(&'a u8, &'b u8)` captures the lifetime `'b` as defined here
+ LL | / {
+ LL | |     (a, b)
+ LL | | }
+    | |_^
+    | |_^
+    |
+ help: to declare that `impl Trait<'a>` captures `'b`, you can add an explicit `'b` lifetime bound
+    |
+ LL | async fn async_ret_impl_trait2<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b
+ 
+ error: aborting due to 3 previous errors
34 
35 For more information about this error, try `rustc --explain E0700`.
---
To only update this specific test, also pass `--test-args async-await/multiple-lifetimes/ret-impl-trait-one.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/multiple-lifetimes/ret-impl-trait-one/auxiliary" "--edition=2018"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL |   async fn async_ret_impl_trait3<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b {
   |  ________________________________--__--_______________________________________________^
   | |                                |   |
   | |                                |   lifetime `'b` defined here
   | |                                lifetime `'a` defined here
LL | |     //~^ ERROR lifetime may not live long enough
LL | |     (a, b)
LL | | }
   | |_^ function was supposed to return data with lifetime `'b` but it is returning data with lifetime `'a`
   |
   = help: consider adding the following bound: `'a: 'b`

error[E0700]: hidden type for `impl Trait<'a>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> {
   |  ____________________________________--__________________________________________^
   | |                                    |
   | |                                    hidden type `(&'a u8, &'b u8)` captures the lifetime `'b` as defined here
LL | |     //~^ ERROR captures lifetime that does not appear in bounds
LL | |     (a, b)
LL | | }
   |
   |
help: to declare that `impl Trait<'a>` captures `'b`, you can add an explicit `'b` lifetime bound
   |
LL | async fn async_ret_impl_trait1<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b {


error[E0700]: hidden type for `impl Trait<'a>` captures lifetime that does not appear in bounds
   |
   |
LL |   async fn async_ret_impl_trait2<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a>
   |                                      -- hidden type `(&'a u8, &'b u8)` captures the lifetime `'b` as defined here
LL | / {
LL | |     (a, b)
LL | | }
   | |_^
   | |_^
   |
help: to declare that `impl Trait<'a>` captures `'b`, you can add an explicit `'b` lifetime bound
   |
LL | async fn async_ret_impl_trait2<'a, 'b>(a: &'a u8, b: &'b u8) -> impl Trait<'a> + 'b

error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0700`.
