plain

---- [ui] src/test/ui/associated-types/associated-types-subtyping-1.rs stdout ----
diff of stderr:

10    |            ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
11 ...
12 LL |     let _c: <T as Trait<'b>>::Type = a;
-    |                                      - because of assignment  here
+    |                                      - because of assignment here
14    |
15    = help: consider adding the following bound: `'b: 'a`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1/associated-types-subtyping-1.stderr
To only update this specific test, also pass `--test-args associated-types/associated-types-subtyping-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/associated-types/associated-types-subtyping-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn method2<'a,'b,T>(x: &'a T, y: &'b T)
   |            -- -- lifetime `'b` defined here
   |            |
   |            lifetime `'a` defined here
...
LL |     let a: <T as Trait<'a>>::Type = make_any();
   |            ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
...
LL |     let _c: <T as Trait<'b>>::Type = a;
   |                                      - because of assignment here
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/associated-types/associated-types-subtyping-1.rs:36:13
   |
   |
LL | fn method3<'a,'b,T>(x: &'a T, y: &'b T)
   |            -- -- lifetime `'b` defined here
   |            |
   |            lifetime `'a` defined here
...
LL |     let _c: <T as Trait<'a>>::Type = b;
   |             ^^^^^^^^^^^^^^^^^^^^^^ type annotation requires that `'b` must outlive `'a`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to 2 previous errors
------------------------------------------



---- [ui] src/test/ui/impl-trait/multiple-lifetimes/error-handling.rs stdout ----
diff of stderr:

7    |        lifetime `'a` defined here
9 LL |     let u = v;
-    |             - because of assignment  here
+    |             - because of assignment here
11 ...
11 ...
12 LL |         let _: &'b i32 = *u.0;
13    |                ^^^^^^^ type annotation requires that `'a` must outlive `'b`

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/error-handling/error-handling.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args impl-trait/multiple-lifetimes/error-handling.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/multiple-lifetimes/error-handling.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/error-handling" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/multiple-lifetimes/error-handling/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a, 'b, 'c>(x: &'static i32, mut y: &'a i32) -> E<'b, 'c> {
   |        --  -- lifetime `'b` defined here
   |        |
   |        lifetime `'a` defined here
LL |     let u = v;
   |             - because of assignment here
...
...
LL |         let _: &'b i32 = *u.0;
   |                ^^^^^^^ type annotation requires that `'a` must outlive `'b`
   |
   = help: consider adding the following bound: `'a: 'b`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/user-annotations/closure-substs.rs stdout ----
diff of stderr:

33    |         let's call the lifetime of this reference `'1`
35 LL |         b(x);
-    |         ---- because of argument  here
+    |         ---- because of argument here
37 
---
To only update this specific test, also pass `--test-args nll/user-annotations/closure-substs.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/closure-substs.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/closure-substs" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/closure-substs/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn foo<'a>() {
   |        -- lifetime `'a` defined here
...
LL |         return x; //~ ERROR lifetime may not live long enough
   |                ^ returning this value requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/closure-substs.rs:13:16
   |
   |
LL |     |x: &i32| -> &'static i32 {
   |         - let's call the lifetime of this reference `'1`
LL |         return x; //~ ERROR lifetime may not live long enough
   |                ^ returning this value requires that `'1` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/closure-substs.rs:19:18
   |
   |
LL | fn bar<'a>() {
   |        -- lifetime `'a` defined here
LL |     // Here `x` is free in the closure sig:
LL |     |x: &'a i32, b: fn(&'static i32)| {
   |                  ^ type annotation requires that `'a` must outlive `'static`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/closure-substs.rs:27:15
   |
   |
LL |     |x: &i32, b: fn(&'static i32)| {
   |         -     ^ type annotation requires that `'1` must outlive `'static`
   |         |
   |         let's call the lifetime of this reference `'1`
LL |         //~^ ERROR lifetime may not live long enough
LL |         b(x);
   |         ---- because of argument here
error: aborting due to 4 previous errors
------------------------------------------


