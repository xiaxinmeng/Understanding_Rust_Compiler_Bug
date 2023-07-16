plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
...................................................................ii...............i... 10648/14493
..iii................................................................................... 10736/14493
........................................................................................ 10824/14493
........................................................................................ 10912/14493
..........................................F............................................. 11000/14493
.....................F...........F...................................................... 11088/14493
........................................................................................ 11264/14493
........................................................................................ 11352/14493
.............................................................iiiii...i....i.i........... 11440/14493
........................................................................................ 11528/14493
---

- warning: unnecessary lifetime parameter `'a`
-   --> $DIR/equal-hidden-lifetimes.rs:7:25
-    |
- LL | fn equal_regions_static<'a: 'static>(x: &'a i32) -> impl Sized {
-    |
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = help: you can use the `'static` lifetime directly, in place of `'a`
- warning: 1 warning emitted
- 
- 

---

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/impl-trait/equal-hidden-lifetimes.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/equal-hidden-lifetimes" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/equal-hidden-lifetimes/auxiliary"
stdout: none
stderr: none

---- [ui] tests/ui/issues/issue-30438-c.rs stdout ----
diff of stderr:


- warning: unnecessary lifetime parameter `'z`
-   --> $DIR/issue-30438-c.rs:7:74
-    |
- LL | fn silly<'y, 'z>(_s: &'y Test<'z>) -> &'y <Test<'z> as Trait>::Out where 'z: 'static {
-    |
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = help: you can use the `'static` lifetime directly, in place of `'z`
10 error[E0515]: cannot return reference to local variable `x`
11   --> $DIR/issue-30438-c.rs:10:5
12    |


13 LL |     &x
14    |     ^^ returns a reference to data owned by the current function
- error: aborting due to previous error; 1 warning emitted
+ error: aborting due to previous error
17 
18 For more information about this error, try `rustc --explain E0515`.
---
To only update this specific test, also pass `--test-args issues/issue-30438-c.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/issues/issue-30438-c.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-30438-c" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-30438-c/auxiliary"
stdout: none
--- stderr -------------------------------
  --> fake-test-src-base/issues/issue-30438-c.rs:10:5
   |
LL |     &x
LL |     &x
   |     ^^ returns a reference to data owned by the current function
error: aborting due to previous error

For more information about this error, try `rustc --explain E0515`.
------------------------------------------
---

- warning: unnecessary lifetime parameter `'a`
-   --> $DIR/regions-free-region-outlives-static-outlives-free-region.rs:12:11
-    |
- LL |     where 'a: 'static
-    |
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = help: you can use the `'static` lifetime directly, in place of `'a`
- warning: 1 warning emitted
- 
- 

---
To only update this specific test, also pass `--test-args regions/regions-free-region-outlives-static-outlives-free-region.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/regions/regions-free-region-outlives-static-outlives-free-region.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-outlives-static-outlives-free-region/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-free-region-outlives-static-outlives-free-region/auxiliary"
stdout: none
stderr: none

---- [ui] tests/ui/regions/regions-static-bound.rs stdout ----
diff of stderr:


- warning: unnecessary lifetime parameter `'a`
-   --> $DIR/regions-static-bound.rs:2:11
-    |
- LL |     where 'a: 'static { t }
-    |
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = help: you can use the `'static` lifetime directly, in place of `'a`
- warning: unnecessary lifetime parameter `'b`
-   --> $DIR/regions-static-bound.rs:6:19
-    |
-    |
- LL |     where 'a: 'b, 'b: 'static { t }
-    |
-    |
-    = help: you can use the `'static` lifetime directly, in place of `'b`
18 error: lifetime may not live long enough
19   --> $DIR/regions-static-bound.rs:10:5
20    |


50    |     `v` escapes the function body here
51    |     argument requires that `'2` must outlive `'static`
- error: aborting due to 3 previous errors; 2 warnings emitted
+ error: aborting due to 3 previous errors
54 
55 For more information about this error, try `rustc --explain E0521`.
---
To only update this specific test, also pass `--test-args regions/regions-static-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/regions/regions-static-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
  --> fake-test-src-base/regions/regions-static-bound.rs:10:5
   |
LL | fn static_id_wrong_way<'a>(t: &'a ()) -> &'static () where 'static: 'a {
   |                        -- lifetime `'a` defined here
LL |     t
   |     ^ returning this value requires that `'a` must outlive `'static`
error[E0521]: borrowed data escapes outside of function
  --> fake-test-src-base/regions/regions-static-bound.rs:15:5
   |
   |
LL | fn error(u: &(), v: &()) {
   |          -  - let's call the lifetime of this reference `'1`
   |          |
   |          `u` is a reference that is only valid in the function body
LL |     static_id(&u);
   |     |
   |     |
   |     `u` escapes the function body here
   |     argument requires that `'1` must outlive `'static`
error[E0521]: borrowed data escapes outside of function
  --> fake-test-src-base/regions/regions-static-bound.rs:17:5
   |
   |
LL | fn error(u: &(), v: &()) {
   |                  -  - let's call the lifetime of this reference `'2`
   |                  |
   |                  `v` is a reference that is only valid in the function body
...
LL |     static_id_indirect(&v);
   |     |
   |     |
   |     `v` escapes the function body here
   |     argument requires that `'2` must outlive `'static`
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0521`.
------------------------------------------
---

- warning: unnecessary lifetime parameter `'a`
-   --> $DIR/regions-static-bound-rpass.rs:4:11
-    |
- LL |     where 'a: 'static { t }
-    |
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = help: you can use the `'static` lifetime directly, in place of `'a`
- warning: unnecessary lifetime parameter `'a`
-   --> $DIR/regions-static-bound-rpass.rs:8:11
-    |
-    |
- LL |     where 'a: 'static { t }
-    |
-    |
-    = help: you can use the `'static` lifetime directly, in place of `'a`
- warning: unnecessary lifetime parameter `'b`
-   --> $DIR/regions-static-bound-rpass.rs:12:19
-    |
-    |
- LL |     where 'a: 'b, 'b: 'static { t }
-    |
-    |
-    = help: you can use the `'static` lifetime directly, in place of `'b`
- warning: 3 warnings emitted
- 
- 

---
To only update this specific test, also pass `--test-args regions/regions-static-bound-rpass.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/regions/regions-static-bound-rpass.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound-rpass/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/regions/regions-static-bound-rpass/auxiliary"
stdout: none
stderr: none

---- [ui] tests/ui/static/static-lifetime-bound.rs stdout ----
diff of stderr:


- warning: unnecessary lifetime parameter `'a`
-   --> $DIR/static-lifetime-bound.rs:1:6
-    |
- LL | fn f<'a: 'static>(_: &'a i32) {}
-    |
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = help: you can use the `'static` lifetime directly, in place of `'a`
- 
10 error[E0597]: `x` does not live long enough
12    |

20 LL | }
20 LL | }
21    | - `x` dropped here while still borrowed
- error: aborting due to previous error; 1 warning emitted
+ error: aborting due to previous error
24 
25 For more information about this error, try `rustc --explain E0597`.
---
To only update this specific test, also pass `--test-args static/static-lifetime-bound.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/static/static-lifetime-bound.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-lifetime-bound" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/static/static-lifetime-bound/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0597]: `x` does not live long enough
  --> fake-test-src-base/static/static-lifetime-bound.rs:5:7
LL |     let x = 0;
LL |     let x = 0;
   |         - binding `x` declared here
LL |     f(&x); //~ERROR does not live long enough
   |     --^^-
   |     | borrowed value does not live long enough
   |     | borrowed value does not live long enough
   |     argument requires that `x` is borrowed for `'static`
LL | }
   | - `x` dropped here while still borrowed
error: aborting due to previous error

For more information about this error, try `rustc --explain E0597`.
------------------------------------------
---

- warning: unnecessary lifetime parameter `'a`
-   --> $DIR/bounds-are-checked.rs:8:6
-    |
- LL | fn f<'a: 'static>(t: &'a str) -> X<'a> {
-    |
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = note: `#[warn(named_static_lifetimes)]` on by default
-    = help: you can use the `'static` lifetime directly, in place of `'a`
- 
10 error[E0792]: expected generic lifetime parameter, found `'static`
12    |

16 LL |     t
17    |     ^
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/bounds-are-checked.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-alias-impl-trait/bounds-are-checked.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/bounds-are-checked/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0792]: expected generic lifetime parameter, found `'static`
  --> fake-test-src-base/type-alias-impl-trait/bounds-are-checked.rs:10:5
   |
LL | type X<'a> = impl Into<&'static str> + From<&'a str>;
   |        -- cannot use static lifetime; use a bound lifetime instead or remove the lifetime parameter from the opaque type
LL |     t
   |     ^

error: aborting due to previous error
---

- warning: unnecessary lifetime parameter `'a`
-   --> $DIR/implied_lifetime_wf_check3.rs:6:22
-    |
- LL |     fn assert_static<'a: 'static>() {}
-    |
-    |
-    = help: you can use the `'static` lifetime directly, in place of `'a`
- warning: unnecessary lifetime parameter `'a`
-   --> $DIR/implied_lifetime_wf_check3.rs:15:22
-    |
-    |
- LL |     fn assert_static<'a: 'static>() {}
-    |
-    |
-    = help: you can use the `'static` lifetime directly, in place of `'a`
- warning: unnecessary lifetime parameter `'a`
-   --> $DIR/implied_lifetime_wf_check3.rs:22:22
-    |
-    |
- LL |     fn assert_static<'a: 'static>() {}
-    |
-    |
-    = help: you can use the `'static` lifetime directly, in place of `'a`
25 error: lifetime may not live long enough
26   --> $DIR/implied_lifetime_wf_check3.rs:8:43
27    |


53 LL |     fn test<A: 'static>() where Ty<A>: 'static { assert_static::<A>() }
55 
- error: aborting due to 4 previous errors; 3 warnings emitted
+ error: aborting due to 4 previous errors
57 
---
To only update this specific test, also pass `--test-args type-alias-impl-trait/implied_lifetime_wf_check3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/tests/ui/type-alias-impl-trait/implied_lifetime_wf_check3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--remap-path-prefix=/checkout/tests/ui=fake-test-src-base" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/implied_lifetime_wf_check3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/type-alias-impl-trait/implied_lifetime_wf_check3/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
  --> fake-test-src-base/type-alias-impl-trait/implied_lifetime_wf_check3.rs:8:43
   |
LL |     fn test<'a>() where Ty<'a>: 'static { assert_static::<'a>() }
   |             -- lifetime `'a` defined here ^^^^^^^^^^^^^^^^^^^ requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> fake-test-src-base/type-alias-impl-trait/implied_lifetime_wf_check3.rs:17:46
   |
   |
LL |     fn test<'a>() where for<'b> Ty<'b>: 'a { assert_static::<'a>() }
   |             -- lifetime `'a` defined here    ^^^^^^^^^^^^^^^^^^^ requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> fake-test-src-base/type-alias-impl-trait/implied_lifetime_wf_check3.rs:24:21
   |
   |
LL |     fn test<'a>() { assert_static::<'a>() }
   |             --      ^^^^^^^^^^^^^^^^^^^ requires that `'a` must outlive `'static`
   |             |
   |             lifetime `'a` defined here
error[E0310]: the parameter type `A` may not live long enough
  --> fake-test-src-base/type-alias-impl-trait/implied_lifetime_wf_check3.rs:32:41
   |
   |
LL |     fn test<A>() where Ty<A>: 'static { assert_static::<A>() }
   |                                         ^^^^^^^^^^^^^^^^^^ ...so that the type `A` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL |     fn test<A: 'static>() where Ty<A>: 'static { assert_static::<A>() }

error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0310`.
