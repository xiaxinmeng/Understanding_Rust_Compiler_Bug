plain

---- [ui (nll)] src/test/ui/impl-trait/issues/issue-67830.rs stdout ----
diff of stderr:

+ error: higher kinded lifetime bounds on nested opaque types are not supported yet
+    |
+    |
+ LL | fn test() -> impl for<'a> MyFn<&'a A, Output=impl Iterator + 'a> {
+    |
+    = help: See https://github.com/rust-lang/rust/issues/96194 for further details
+ 
+ 
1 error: implementation of `FnOnce` is not general enough
-   --> $DIR/issue-67830.rs:23:5
3    |
3    |
4 LL |     Wrap(|a| Some(a).into_iter())
5    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough

8    = note: ...but it actually implements `FnOnce<(&'2 A,)>`, for some specific lifetime `'2`
9 
10 error: implementation of `FnOnce` is not general enough
-   --> $DIR/issue-67830.rs:23:5
12    |
12    |
13 LL |     Wrap(|a| Some(a).into_iter())
14    |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough

16    = note: closure with signature `fn(&'2 A) -> std::option::IntoIter<&A>` must implement `FnOnce<(&'1 A,)>`, for any lifetime `'1`...
17    = note: ...but it actually implements `FnOnce<(&'2 A,)>`, for some specific lifetime `'2`
- error: aborting due to 2 previous errors
+ error: aborting due to 3 previous errors
20 
21 
21 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-67830.nll/issue-67830.nll.stderr
To only update this specific test, also pass `--test-args impl-trait/issues/issue-67830.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-67830.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-67830.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-67830.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn test() -> impl for<'a> MyFn<&'a A, Output=impl Iterator + 'a> {
   |
   = help: See https://github.com/rust-lang/rust/issues/96194 for further details


error: implementation of `FnOnce` is not general enough
   |
   |
LL |     Wrap(|a| Some(a).into_iter())
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'2 A) -> std::option::IntoIter<&A>` must implement `FnOnce<(&'1 A,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 A,)>`, for some specific lifetime `'2`

error: implementation of `FnOnce` is not general enough
   |
   |
LL |     Wrap(|a| Some(a).into_iter())
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ implementation of `FnOnce` is not general enough
   |
   = note: closure with signature `fn(&'2 A) -> std::option::IntoIter<&A>` must implement `FnOnce<(&'1 A,)>`, for any lifetime `'1`...
   = note: ...but it actually implements `FnOnce<(&'2 A,)>`, for some specific lifetime `'2`
error: aborting due to 3 previous errors
------------------------------------------



---- [ui (nll)] src/test/ui/impl-trait/issues/issue-88236-2.rs stdout ----
diff of stderr:

+ error: higher kinded lifetime bounds on nested opaque types are not supported yet
+    |
+    |
+ LL | fn make_impl() -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {}
+    |
+    = help: See https://github.com/rust-lang/rust/issues/96194 for further details
+ 
+ 
+ error: higher kinded lifetime bounds on nested opaque types are not supported yet
+    |
+    |
+ LL | fn make_weird_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {
+    |
+    = help: See https://github.com/rust-lang/rust/issues/96194 for further details
+ 
+ 
+ error: higher kinded lifetime bounds on nested opaque types are not supported yet
+    |
+    |
+ LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {
+    |
+    = help: See https://github.com/rust-lang/rust/issues/96194 for further details
+ 
+ 
+ error: higher kinded lifetime bounds on nested opaque types are not supported yet
+    |
+    |
+ LL | fn make_bad_impl_2<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Zend<'a>> {
+    |
+    = help: See https://github.com/rust-lang/rust/issues/96194 for further details
+ 
+ 
1 error: implementation of `Hrtb` is not general enough
-   --> $DIR/issue-88236-2.rs:17:5
3    |
4 LL |     &()
4 LL |     &()
5    |     ^^^ implementation of `Hrtb` is not general enough

8    = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`
9 
10 error: implementation of `Hrtb` is not general enough
-   --> $DIR/issue-88236-2.rs:17:5
12    |
13 LL |     &()
13 LL |     &()
14    |     ^^^ implementation of `Hrtb` is not general enough

17    = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`
19 error: lifetime may not live long enough
-   --> $DIR/issue-88236-2.rs:20:5
+   --> $DIR/issue-88236-2.rs:22:5
21    |
21    |
22 LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {
23    |                  -- lifetime `'b` defined here
34    |                                                                                 ++++
35 
35 
36 error: implementation of `Hrtb` is not general enough
-   --> $DIR/issue-88236-2.rs:20:5
38    |
39 LL |     x
39 LL |     x
40    |     ^ implementation of `Hrtb` is not general enough

43    = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`
44 
45 error: implementation of `Hrtb` is not general enough
-   --> $DIR/issue-88236-2.rs:20:5
47    |
48 LL |     x
48 LL |     x
49    |     ^ implementation of `Hrtb` is not general enough

51    = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
52    = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`
- error: aborting due to 5 previous errors
+ error: lifetime may not live long enough
+   --> $DIR/issue-88236-2.rs:31:5
+    |
+    |
+ LL | fn make_bad_impl_2<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Zend<'a>> {
+    |                    -- lifetime `'b` defined here
+ LL |     x
+    |     ^ returning this value requires that `'b` must outlive `'static`
+    |
+ help: to declare that the `impl Trait` captures data from argument `x`, you can add an explicit `'b` lifetime bound
+    |
+ LL | fn make_bad_impl_2<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Zend<'a>> + 'b {
+    |                                                                                   ++++
+ help: to declare that the `impl Trait` captures data from argument `x`, you can add an explicit `'b` lifetime bound
+    |
+ LL | fn make_bad_impl_2<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Zend<'a> + 'b> {
+ 
+ 
+ error: implementation of `Hrtb` is not general enough
+    |
+ LL |     x
+ LL |     x
+    |     ^ implementation of `Hrtb` is not general enough
+    |
+    = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
+    = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`
+ 
+ error: implementation of `Hrtb` is not general enough
+    |
+ LL |     x
+ LL |     x
+    |     ^ implementation of `Hrtb` is not general enough
+    |
+    = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
+    = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`
+ error: aborting due to 12 previous errors
55 
56 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-88236-2.nll/issue-88236-2.nll.stderr
To only update this specific test, also pass `--test-args impl-trait/issues/issue-88236-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-88236-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-88236-2.nll" "-Zborrowck=mir" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-88236-2.nll/auxiliary"
stdout: none
--- stderr -------------------------------
error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn make_impl() -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {}
   |
   = help: See https://github.com/rust-lang/rust/issues/96194 for further details


error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn make_weird_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {
   |
   = help: See https://github.com/rust-lang/rust/issues/96194 for further details


error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {
   |
   = help: See https://github.com/rust-lang/rust/issues/96194 for further details


error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn make_bad_impl_2<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Zend<'a>> {
   |
   = help: See https://github.com/rust-lang/rust/issues/96194 for further details


error: implementation of `Hrtb` is not general enough
   |
   |
LL |     &() //~^ ERROR implementation of `Hrtb` is not general enough
   |     ^^^ implementation of `Hrtb` is not general enough
   |
   = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
   = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`

error: implementation of `Hrtb` is not general enough
   |
   |
LL |     &() //~^ ERROR implementation of `Hrtb` is not general enough
   |     ^^^ implementation of `Hrtb` is not general enough
   |
   = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
   = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/impl-trait/issues/issue-88236-2.rs:22:5
   |
   |
LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {
   |                  -- lifetime `'b` defined here
LL |     x //~^ ERROR implementation of `Hrtb` is not general enough
   |     ^ returning this value requires that `'b` must outlive `'static`
   |
help: to declare that the `impl Trait` captures data from argument `x`, you can add an explicit `'b` lifetime bound
   |
LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> + 'b {
   |                                                                                  ++++
help: to declare that the `impl Trait` captures data from argument `x`, you can add an explicit `'b` lifetime bound
   |
LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a + 'b> {


error: implementation of `Hrtb` is not general enough
   |
   |
LL |     x //~^ ERROR implementation of `Hrtb` is not general enough
   |     ^ implementation of `Hrtb` is not general enough
   |
   = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
   = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`

error: implementation of `Hrtb` is not general enough
   |
   |
LL |     x //~^ ERROR implementation of `Hrtb` is not general enough
   |     ^ implementation of `Hrtb` is not general enough
   |
   = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
   = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/impl-trait/issues/issue-88236-2.rs:31:5
   |
   |
LL | fn make_bad_impl_2<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Zend<'a>> {
   |                    -- lifetime `'b` defined here
LL |     x //~^ ERROR implementation of `Hrtb` is not general enough
   |     ^ returning this value requires that `'b` must outlive `'static`
   |
help: to declare that the `impl Trait` captures data from argument `x`, you can add an explicit `'b` lifetime bound
   |
LL | fn make_bad_impl_2<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Zend<'a>> + 'b {
   |                                                                                   ++++
help: to declare that the `impl Trait` captures data from argument `x`, you can add an explicit `'b` lifetime bound
Some tests failed in compiletest suite=ui compare_mode=Nll mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL | fn make_bad_impl_2<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Zend<'a> + 'b> {


error: implementation of `Hrtb` is not general enough
   |
   |
LL |     x //~^ ERROR implementation of `Hrtb` is not general enough
   |     ^ implementation of `Hrtb` is not general enough
   |
   = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
   = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`

error: implementation of `Hrtb` is not general enough
   |
   |
LL |     x //~^ ERROR implementation of `Hrtb` is not general enough
   |     ^ implementation of `Hrtb` is not general enough
   |
   = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
   = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`
error: aborting due to 12 previous errors
------------------------------------------


