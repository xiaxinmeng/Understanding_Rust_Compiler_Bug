plain
........................................................................................ 8448/13852
........................................................................................ 8536/13852
........................................................................................ 8624/13852
........................................................................................ 8712/13852
......F...............................F..................i..ii.......................... 8800/13852
...............................................iiii..................................... 8976/13852
........................................................................................ 9064/13852
i........................................i.............................................. 9152/13852
......................i................................................................. 9240/13852
---
---- [ui] src/test/ui/nll/user-annotations/ascribed-type-wf.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/ascribed-type-wf.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/ascribed-type-wf" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/ascribed-type-wf/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn extend<'a>() {
   |           -- lifetime `'a` defined here
LL |     None::<<&'a () as Trait>::Ty>;
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ requires that `'a` must outlive `'static`
error: aborting due to previous error
------------------------------------------



---- [ui] src/test/ui/nll/user-annotations/normalization-infer.rs stdout ----
diff of stderr:

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
79   --> $DIR/normalization-infer.rs:32:28
80    |
81 LL |     let _: Alias<_, _> = (&temp(), 0u8);
-    |            -----------     ^^^^^^ creates a temporary which is freed while still in use
+    |            -----------     ^^^^^^ creates a temporary value which is freed while still in use
83    |            |
84    |            type annotation requires that borrow lasts for `'static`


92 LL |     Some::<Alias<_, _>>((&temp(), 0u8));
93    |                         --^^^^^^------ - temporary value is freed at the end of this statement
-    |                         | creates a temporary which is freed while still in use
+    |                         | creates a temporary value which is freed while still in use
+    |                         | creates a temporary value which is freed while still in use
96    |                         this usage requires that borrow lasts for `'static`
98 error: aborting due to 9 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/normalization-infer/normalization-infer.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args nll/user-annotations/normalization-infer.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/nll/user-annotations/normalization-infer.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/normalization-infer" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/nll/user-annotations/normalization-infer/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0310]: the parameter type `A` may not live long enough
   |
   |
LL |     let _: <(_,) as Tr>::Ty = a; //~ ERROR type `A`
   |            ^^^^^^^^^^^^^^^^ ...so that the type `A` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn test1<A: 'static, B, C, D>(a: A, b: B, c: C) {

error[E0310]: the parameter type `B` may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/normalization-infer.rs:12:5
   |
   |
LL |     Some::<<(_,) as Tr>::Ty>(b); //~ ERROR type `B`
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `B` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn test1<A, B: 'static, C, D>(a: A, b: B, c: C) {

error[E0310]: the parameter type `C` may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/normalization-infer.rs:13:11
   |
   |
LL |     || -> <(_,) as Tr>::Ty { c }; //~ ERROR type `C`
   |           ^^^^^^^^^^^^^^^^ ...so that the type `C` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn test1<A, B, C: 'static, D>(a: A, b: B, c: C) {

error[E0310]: the parameter type `D` may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/normalization-infer.rs:14:6
   |
   |
LL |     |d: <(_,) as Tr>::Ty| -> D { d }; //~ ERROR type `D`
   |      ^ ...so that the type `D` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn test1<A, B, C, D: 'static>(a: A, b: B, c: C) {

error[E0310]: the parameter type `A` may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/normalization-infer.rs:28:12
   |
   |
LL |     let _: Alias<_, _> = (a, 0u8); //~ ERROR type `A`
   |            ^^^^^^^^^^^ ...so that the type `A` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn test2<A: 'static, B, C>(a: A, b: B, c: C) {

error[E0310]: the parameter type `B` may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/normalization-infer.rs:29:5
   |
   |
LL |     Some::<Alias<_, _>>((b, 0u8)); //~ ERROR type `B`
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ ...so that the type `B` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn test2<A, B: 'static, C>(a: A, b: B, c: C) {

error[E0310]: the parameter type `C` may not live long enough
  --> /checkout/src/test/ui/nll/user-annotations/normalization-infer.rs:30:11
   |
   |
LL |     || -> Alias<_, _> { (c, 0u8) }; //~ ERROR type `C`
   |           ^^^^^^^^^^^ ...so that the type `C` will meet its required lifetime bounds
help: consider adding an explicit lifetime bound...
   |
   |
LL | fn test2<A, B, C: 'static>(a: A, b: B, c: C) {

error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/nll/user-annotations/normalization-infer.rs:32:28
   |
   |
LL |     let _: Alias<_, _> = (&temp(), 0u8); //~ ERROR temporary value
   |            -----------     ^^^^^^ creates a temporary value which is freed while still in use
   |            |
   |            type annotation requires that borrow lasts for `'static`
LL | }
   | - temporary value is freed at the end of this statement

error[E0716]: temporary value dropped while borrowed
error[E0716]: temporary value dropped while borrowed
  --> /checkout/src/test/ui/nll/user-annotations/normalization-infer.rs:33:27
   |
LL |     Some::<Alias<_, _>>((&temp(), 0u8)); //~ ERROR temporary value
   |                         --^^^^^^------ - temporary value is freed at the end of this statement
   |                         | creates a temporary value which is freed while still in use
   |                         | creates a temporary value which is freed while still in use
   |                         this usage requires that borrow lasts for `'static`
error: aborting due to 9 previous errors

Some errors have detailed explanations: E0310, E0716.
For more information about an error, try `rustc --explain E0310`.
