plain
........................................................................................ 4664/13760
................i....................................................................... 4752/13760
........................................................................................ 4840/13760
........................................................................................ 4928/13760
........................F.....F......................................................... 5016/13760
..................................F..................................................... 5104/13760
........................................................................................ 5280/13760
..................................................................i..................... 5368/13760
................................................i....................................... 5456/13760
........................................................................................ 5544/13760
---

---- [ui] src/test/ui/impl-trait/impl-fn-hrtb-bounds.rs stdout ----
diff of stderr:

22 LL | fn a() -> impl Fn(&u8) -> (impl Debug + '_) {
24 
+ error: lifetime may not live long enough
+   --> $DIR/impl-fn-hrtb-bounds.rs:6:9
+    |
+    |
+ LL |     |x| x
+    |      -- ^ returning this value requires that `'1` must outlive `'2`
+    |      ||
+    |      |return type of closure is impl Debug + '2
+    |      has type `&'1 u8`
+ 
25 error: higher kinded lifetime bounds on nested opaque types are not supported yet
27    |


34 LL | fn b() -> impl for<'a> Fn(&'a u8) -> (impl Debug + 'a) {
36 
+ error: lifetime may not live long enough
+   --> $DIR/impl-fn-hrtb-bounds.rs:11:9
+    |
+    |
+ LL |     |x| x
+    |      -- ^ returning this value requires that `'1` must outlive `'2`
+    |      ||
+    |      |return type of closure is impl Debug + '2
+    |      has type `&'1 u8`
+ 
37 error: higher kinded lifetime bounds on nested opaque types are not supported yet
39    |


46 LL | fn c() -> impl for<'a> Fn(&'a u8) -> (impl Debug + '_) {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
48 
- error: aborting due to 4 previous errors
+ error: lifetime may not live long enough
+   --> $DIR/impl-fn-hrtb-bounds.rs:16:9
+   --> $DIR/impl-fn-hrtb-bounds.rs:16:9
+    |
+ LL |     |x| x
+    |      -- ^ returning this value requires that `'1` must outlive `'2`
+    |      ||
+    |      |return type of closure is impl Debug + '2
+    |      has type `&'1 u8`
+ error: aborting due to 7 previous errors
50 
51 For more information about this error, try `rustc --explain E0106`.
52 
---
To only update this specific test, also pass `--test-args impl-trait/impl-fn-hrtb-bounds.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl-fn-hrtb-bounds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-fn-hrtb-bounds" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-fn-hrtb-bounds/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0106]: missing lifetime specifier
   |
   |
LL | fn d() -> impl Fn() -> (impl Debug + '_) {
   |                                      ^^ expected named lifetime parameter
   |
   = help: this function's return type contains a borrowed value, but there is no value for it to be borrowed from
help: consider using the `'static` lifetime
   |
LL | fn d() -> impl Fn() -> (impl Debug + 'static) {


error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn a() -> impl Fn(&u8) -> (impl Debug + '_) {
   |
note: lifetime declared here
  --> /checkout/src/test/ui/impl-trait/impl-fn-hrtb-bounds.rs:4:19
   |
   |
LL | fn a() -> impl Fn(&u8) -> (impl Debug + '_) {

error: lifetime may not live long enough
  --> /checkout/src/test/ui/impl-trait/impl-fn-hrtb-bounds.rs:6:9
   |
   |
LL |     |x| x
   |      -- ^ returning this value requires that `'1` must outlive `'2`
   |      ||
   |      |return type of closure is impl Debug + '2
   |      has type `&'1 u8`

error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn b() -> impl for<'a> Fn(&'a u8) -> (impl Debug + 'a) {
   |
note: lifetime declared here
  --> /checkout/src/test/ui/impl-trait/impl-fn-hrtb-bounds.rs:9:20
   |
   |
LL | fn b() -> impl for<'a> Fn(&'a u8) -> (impl Debug + 'a) {

error: lifetime may not live long enough
  --> /checkout/src/test/ui/impl-trait/impl-fn-hrtb-bounds.rs:11:9
   |
   |
LL |     |x| x
   |      -- ^ returning this value requires that `'1` must outlive `'2`
   |      ||
   |      |return type of closure is impl Debug + '2
   |      has type `&'1 u8`

error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn c() -> impl for<'a> Fn(&'a u8) -> (impl Debug + '_) {
   |
note: lifetime declared here
  --> /checkout/src/test/ui/impl-trait/impl-fn-hrtb-bounds.rs:14:20
   |
   |
LL | fn c() -> impl for<'a> Fn(&'a u8) -> (impl Debug + '_) {

error: lifetime may not live long enough
  --> /checkout/src/test/ui/impl-trait/impl-fn-hrtb-bounds.rs:16:9
   |
   |
LL |     |x| x
   |      -- ^ returning this value requires that `'1` must outlive `'2`
   |      ||
   |      |return type of closure is impl Debug + '2
   |      has type `&'1 u8`
error: aborting due to 7 previous errors

For more information about this error, try `rustc --explain E0106`.
------------------------------------------
------------------------------------------


---- [ui] src/test/ui/impl-trait/impl-fn-parsing-ambiguities.rs stdout ----
diff of stderr:

22 LL | fn a() -> impl Fn(&u8) -> impl Debug + '_ {
24 
- error: aborting due to 3 previous errors
+ error: lifetime may not live long enough
+   --> $DIR/impl-fn-parsing-ambiguities.rs:7:9
+   --> $DIR/impl-fn-parsing-ambiguities.rs:7:9
+    |
+ LL |     |x| x
+    |      -- ^ returning this value requires that `'1` must outlive `'2`
+    |      ||
+    |      |return type of closure is impl Debug + '2
+    |      has type `&'1 u8`
+ error: aborting due to 4 previous errors
26 
27 

---
To only update this specific test, also pass `--test-args impl-trait/impl-fn-parsing-ambiguities.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/impl-fn-parsing-ambiguities.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-fn-parsing-ambiguities" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/impl-fn-parsing-ambiguities/auxiliary"
stdout: none
--- stderr -------------------------------
error: ambiguous `+` in a type
   |
   |
LL | fn a() -> impl Fn(&u8) -> impl Debug + '_ {
   |                           ^^^^^^^^^^^^^^^ help: use parentheses to disambiguate: `(impl Debug + '_)`

error: ambiguous `+` in a type
   |
   |
LL | fn b() -> impl Fn() -> impl Debug + Send {
   |                        ^^^^^^^^^^^^^^^^^ help: use parentheses to disambiguate: `(impl Debug + Send)`

error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn a() -> impl Fn(&u8) -> impl Debug + '_ {
   |
note: lifetime declared here
  --> /checkout/src/test/ui/impl-trait/impl-fn-parsing-ambiguities.rs:4:19
   |
   |
LL | fn a() -> impl Fn(&u8) -> impl Debug + '_ {

error: lifetime may not live long enough
  --> /checkout/src/test/ui/impl-trait/impl-fn-parsing-ambiguities.rs:7:9
   |
   |
LL |     |x| x
   |      -- ^ returning this value requires that `'1` must outlive `'2`
   |      ||
   |      |return type of closure is impl Debug + '2
   |      has type `&'1 u8`
error: aborting due to 4 previous errors
------------------------------------------



---- [ui] src/test/ui/impl-trait/issues/issue-88236-2.rs stdout ----
diff of stderr:

61 LL |     x
62    |     ^ returning this value requires that `'b` must outlive `'static`
63    |
- help: to declare that the `impl Trait` captures data from argument `x`, you can add an explicit `'b` lifetime bound
+ help: to declare that `impl for<'a> Hrtb<'a, Assoc = impl Send + 'static>` captures data from argument `x`, you can add an explicit `'b` lifetime bound
65    |
66 LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> + 'b {


- help: to declare that the `impl Trait` captures data from argument `x`, you can add an explicit `'b` lifetime bound
+ help: to declare that `impl Send + 'a` captures data from argument `x`, you can add an explicit `'b` lifetime bound
69    |
70 LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a + 'b> {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-88236-2/issue-88236-2.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-88236-2/issue-88236-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/issues/issue-88236-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/issues/issue-88236-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-88236-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/issues/issue-88236-2/auxiliary"
stdout: none
--- stderr -------------------------------
error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn make_impl() -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {}
   |
note: lifetime declared here
  --> /checkout/src/test/ui/impl-trait/issues/issue-88236-2.rs:15:28
   |
   |
LL | fn make_impl() -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {}


error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn make_weird_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {
   |
note: lifetime declared here
  --> /checkout/src/test/ui/impl-trait/issues/issue-88236-2.rs:18:47
   |
   |
LL | fn make_weird_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {


error: implementation of `Hrtb` is not general enough
   |
LL |     &()
LL |     &()
   |     ^^^ implementation of `Hrtb` is not general enough
   |
   = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
   = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`

error: implementation of `Hrtb` is not general enough
   |
LL |     &()
LL |     &()
   |     ^^^ implementation of `Hrtb` is not general enough
   |
   = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
   = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`

error: higher kinded lifetime bounds on nested opaque types are not supported yet
   |
   |
LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {
   |
note: lifetime declared here
  --> /checkout/src/test/ui/impl-trait/issues/issue-88236-2.rs:25:45
   |
   |
LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {

error: lifetime may not live long enough
  --> /checkout/src/test/ui/impl-trait/issues/issue-88236-2.rs:27:5
   |
   |
LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> {
   |                  -- lifetime `'b` defined here
LL |     //~^ ERROR higher kinded lifetime bounds on nested opaque types are not supported yet
LL |     x
   |     ^ returning this value requires that `'b` must outlive `'static`
   |
help: to declare that `impl for<'a> Hrtb<'a, Assoc = impl Send + 'static>` captures data from argument `x`, you can add an explicit `'b` lifetime bound
   |
LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a> + 'b {
   |                                                                                  ++++
help: to declare that `impl Send + 'a` captures data from argument `x`, you can add an explicit `'b` lifetime bound
   |
LL | fn make_bad_impl<'b>(x: &'b ()) -> impl for<'a> Hrtb<'a, Assoc = impl Send + 'a + 'b> {


error: implementation of `Hrtb` is not general enough
   |
LL |     x
LL |     x
   |     ^ implementation of `Hrtb` is not general enough
   |
   = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
   = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`

error: implementation of `Hrtb` is not general enough
   |
LL |     x
LL |     x
   |     ^ implementation of `Hrtb` is not general enough
   |
   = note: `Hrtb<'0>` would have to be implemented for the type `&()`, for any lifetime `'0`...
   = note: ...but `Hrtb<'1>` is actually implemented for the type `&'1 ()`, for some specific lifetime `'1`
error: aborting due to 8 previous errors
------------------------------------------


