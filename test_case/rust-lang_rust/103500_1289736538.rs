plain

6 LL |     x
7    |     ^
8    |
- help: to declare that `impl Swap` captures `'b`, you can add an explicit `'b` lifetime bound
+ help: to declare that `impl Swap + 'a` captures `'b`, you can add an explicit `'b` lifetime bound
10    |
11 LL | fn hide_ref<'a, 'b, T: 'static>(x: &'a mut &'b T) -> impl Swap + 'a + 'b {

19 LL |     x
20    |     ^
21    |
21    |
- help: to declare that `impl Swap` captures `'b`, you can add an explicit `'b` lifetime bound
+ help: to declare that `impl Swap + 'a` captures `'b`, you can add an explicit `'b` lifetime bound
23    |
24 LL | fn hide_rc_refcell<'a, 'b: 'a, T: 'static>(x: Rc<RefCell<&'b T>>) -> impl Swap + 'a + 'b {


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/hidden-lifetimes/hidden-lifetimes.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/hidden-lifetimes/hidden-lifetimes.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/hidden-lifetimes.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/hidden-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/hidden-lifetimes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/hidden-lifetimes/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Swap + 'a` captures lifetime that does not appear in bounds
   |
   |
LL | fn hide_ref<'a, 'b, T: 'static>(x: &'a mut &'b T) -> impl Swap + 'a {
   |                 -- hidden type `&'a mut &'b T` captures the lifetime `'b` as defined here
   |     ^
   |
   |
help: to declare that `impl Swap + 'a` captures `'b`, you can add an explicit `'b` lifetime bound
   |
LL | fn hide_ref<'a, 'b, T: 'static>(x: &'a mut &'b T) -> impl Swap + 'a + 'b {


error[E0700]: hidden type for `impl Swap + 'a` captures lifetime that does not appear in bounds
   |
   |
LL | fn hide_rc_refcell<'a, 'b: 'a, T: 'static>(x: Rc<RefCell<&'b T>>) -> impl Swap + 'a {
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |                        -- hidden type `Rc<RefCell<&'b T>>` captures the lifetime `'b` as defined here
   |     ^
   |
   |
help: to declare that `impl Swap + 'a` captures `'b`, you can add an explicit `'b` lifetime bound
   |
LL | fn hide_rc_refcell<'a, 'b: 'a, T: 'static>(x: Rc<RefCell<&'b T>>) -> impl Swap + 'a + 'b {

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0700`.
For more information about this error, try `rustc --explain E0700`.
------------------------------------------


---- [ui] src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs stdout ----
diff of stderr:

32    |               |
33    |               let's call the lifetime of this reference `'1`
34    |
- help: consider changing `impl Copy`'s explicit `'static` bound to the lifetime of argument `x`
+ help: consider changing `impl Copy + 'static`'s explicit `'static` bound to the lifetime of argument `x`
36    |
37 LL | fn elided2(x: &i32) -> impl Copy + '_ { x }


47 LL | fn explicit2<'a>(x: &'a i32) -> impl Copy + 'static { x }
48    |              -- lifetime `'a` defined here            ^ returning this value requires that `'a` must outlive `'static`
49    |
- help: consider changing `impl Copy`'s explicit `'static` bound to the lifetime of argument `x`
+ help: consider changing `impl Copy + 'static`'s explicit `'static` bound to the lifetime of argument `x`
51    |
52 LL | fn explicit2<'a>(x: &'a i32) -> impl Copy + 'a { x }


87 LL | fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }
88    |               -- lifetime `'a` defined here                         ^ returning this value requires that `'a` must outlive `'static`
89    |
- help: consider changing `impl LifetimeTrait<'a>`'s explicit `'static` bound to the lifetime of argument `x`
+ help: consider changing `impl LifetimeTrait<'a> + 'static`'s explicit `'static` bound to the lifetime of argument `x`
91    |
92 LL | fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'a { x }


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound/must_outlive_least_region_or_bound.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound/must_outlive_least_region_or_bound.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/must_outlive_least_region_or_bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/must_outlive_least_region_or_bound/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0700]: hidden type for `impl Copy` captures lifetime that does not appear in bounds
   |
   |
LL | fn elided(x: &i32) -> impl Copy { x }
   |              |
   |              |
   |              hidden type `&i32` captures the anonymous lifetime defined here
   |
help: to declare that `impl Copy` captures `'_`, you can add an explicit `'_` lifetime bound
   |
LL | fn elided(x: &i32) -> impl Copy + '_ { x }


error[E0700]: hidden type for `impl Copy` captures lifetime that does not appear in bounds
   |
   |
LL | fn explicit<'a>(x: &'a i32) -> impl Copy { x }
   |             |
   |             |
   |             hidden type `&'a i32` captures the lifetime `'a` as defined here
   |
help: to declare that `impl Copy` captures `'a`, you can add an explicit `'a` lifetime bound
   |
LL | fn explicit<'a>(x: &'a i32) -> impl Copy + 'a { x }

error: lifetime may not live long enough
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:9:46
   |
   |
LL | fn elided2(x: &i32) -> impl Copy + 'static { x }
   |               -                              ^ returning this value requires that `'1` must outlive `'static`
   |               |
   |               let's call the lifetime of this reference `'1`
   |
help: consider changing `impl Copy + 'static`'s explicit `'static` bound to the lifetime of argument `x`
   |
LL | fn elided2(x: &i32) -> impl Copy + '_ { x }
   |                                    ~~
help: alternatively, add an explicit `'static` bound to this reference
   |
LL | fn elided2(x: &'static i32) -> impl Copy + 'static { x }

error: lifetime may not live long enough
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:12:55
   |
   |
LL | fn explicit2<'a>(x: &'a i32) -> impl Copy + 'static { x }
   |              -- lifetime `'a` defined here            ^ returning this value requires that `'a` must outlive `'static`
   |
help: consider changing `impl Copy + 'static`'s explicit `'static` bound to the lifetime of argument `x`
   |
LL | fn explicit2<'a>(x: &'a i32) -> impl Copy + 'a { x }
   |                                             ~~
help: alternatively, add an explicit `'static` bound to this reference
   |
LL | fn explicit2<'a>(x: &'static i32) -> impl Copy + 'static { x }

error[E0621]: explicit lifetime required in the type of `x`
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:15:41
   |
   |
LL | fn foo<'a>(x: &i32) -> impl Copy + 'a { x }
   |               ----                      ^ lifetime `'a` required
   |               |
   |               help: add explicit lifetime `'a` to the type of `x`: `&'a i32`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:26:55
   |
   |
LL | fn elided5(x: &i32) -> (Box<dyn Debug>, impl Debug) { (Box::new(x), x) }
   |               -                                       ^^^^^^^^^^^^^^^^ returning this value requires that `'1` must outlive `'static`
   |               |
   |               let's call the lifetime of this reference `'1`
   |
help: to declare that the trait object captures data from argument `x`, you can add an explicit `'_` lifetime bound
   |
LL | fn elided5(x: &i32) -> (Box<dyn Debug + '_>, impl Debug) { (Box::new(x), x) }
   |                                       ++++
help: to declare that `impl Debug` captures data from argument `x`, you can add an explicit `'_` lifetime bound
   |
LL | fn elided5(x: &i32) -> (Box<dyn Debug>, impl Debug + '_) { (Box::new(x), x) }

error: lifetime may not live long enough
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:32:69
   |
   |
LL | fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'static { x }
   |               -- lifetime `'a` defined here                         ^ returning this value requires that `'a` must outlive `'static`
   |
help: consider changing `impl LifetimeTrait<'a> + 'static`'s explicit `'static` bound to the lifetime of argument `x`
   |
LL | fn with_bound<'a>(x: &'a i32) -> impl LifetimeTrait<'a> + 'a { x }
   |                                                           ~~
help: alternatively, add an explicit `'static` bound to this reference
   |
LL | fn with_bound<'a>(x: &'static i32) -> impl LifetimeTrait<'a> + 'static { x }


error[E0700]: hidden type for `impl Fn(&'a u32)` captures lifetime that does not appear in bounds
   |
   |
LL | fn move_lifetime_into_fn<'a, 'b>(x: &'a u32, y: &'b u32) -> impl Fn(&'a u32) {
   |                              -- hidden type `[closure@/checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:38:5: 38:13]` captures the lifetime `'b` as defined here
LL |     move |_| println!("{}", y)
   |
   |
help: to declare that `impl Fn(&'a u32)` captures `'b`, you can add an explicit `'b` lifetime bound
   |
LL | fn move_lifetime_into_fn<'a, 'b>(x: &'a u32, y: &'b u32) -> impl Fn(&'a u32) + 'b {

error[E0310]: the parameter type `T` may not live long enough
  --> /checkout/src/test/ui/impl-trait/must_outlive_least_region_or_bound.rs:43:5
   |
   |
LL |     x
   |     ^ ...so that the type `T` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn ty_param_wont_outlive_static<T:Debug + 'static>(x: T) -> impl Debug + 'static {

error: aborting due to 9 previous errors

Some errors have detailed explanations: E0310, E0621, E0700.
