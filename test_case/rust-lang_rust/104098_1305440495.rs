plain

1 error: lifetime may not live long enough
-   --> $DIR/wf-static-method.rs:17:9
-    |
- LL | impl<'a, 'b> Foo<'a, 'b, Evil<'a, 'b>> for () {
-    |      --  -- lifetime `'b` defined here
-    |      |
-    |      lifetime `'a` defined here
- LL |         u
- LL |         u
-    |         ^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
-    |
-    = help: consider adding the following bound: `'b: 'a`
- error: lifetime may not live long enough
-   --> $DIR/wf-static-method.rs:27:18
-    |
-    |
- LL | impl<'a, 'b> Foo<'a, 'b, ()> for IndirectEvil<'a, 'b> {
-    |      --  -- lifetime `'b` defined here
-    |      |
-    |      lifetime `'a` defined here
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- ...
- LL |         let me = Self::make_me();
-    |                  ^^^^^^^^^^^^^ requires that `'b` must outlive `'a`
-    |
-    = help: consider adding the following bound: `'b: 'a`
- error: lifetime may not live long enough
-   --> $DIR/wf-static-method.rs:35:9
-    |
-    |
- LL | impl<'a, 'b> Evil<'a, 'b> {
-    |      --  -- lifetime `'b` defined here
-    |      |
-    |      lifetime `'a` defined here
- LL |     fn inherent_evil(u: &'b u32) -> &'a u32 {
- LL |         u
-    |         ^ associated function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
-    |
-    = help: consider adding the following bound: `'b: 'a`
- error: lifetime may not live long enough
41   --> $DIR/wf-static-method.rs:44:5
42    |
42    |
43 LL | fn evil<'a, 'b>(b: &'b u32) -> &'a u32 {
73    |
73    |
74    = help: consider adding the following bound: `'b: 'a`
- error: aborting due to 6 previous errors
+ error: aborting due to 3 previous errors
77 
78 
78 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method/wf-static-method.stderr
To only update this specific test, also pass `--test-args wf/wf-static-method.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/wf/wf-static-method.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/wf/wf-static-method/auxiliary"
stdout: none
--- stderr -------------------------------
error: lifetime may not live long enough
   |
   |
LL | fn evil<'a, 'b>(b: &'b u32) -> &'a u32 {
   |         --  -- lifetime `'b` defined here
   |         |
   |         lifetime `'a` defined here
LL |     <()>::static_evil(b)
   |     ^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/wf/wf-static-method.rs:49:5
   |
   |
LL | fn indirect_evil<'a, 'b>(b: &'b u32) -> &'a u32 {
   |                  --  -- lifetime `'b` defined here
   |                  |
   |                  lifetime `'a` defined here
LL |     <IndirectEvil>::static_evil(b)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: lifetime may not live long enough
  --> /checkout/src/test/ui/wf/wf-static-method.rs:54:5
   |
   |
LL | fn inherent_evil<'a, 'b>(b: &'b u32) -> &'a u32 {
   |                  --  -- lifetime `'b` defined here
   |                  |
   |                  lifetime `'a` defined here
LL |     <Evil>::inherent_evil(b)
   |     ^^^^^^^^^^^^^^^^^^^^^^^^ function was supposed to return data with lifetime `'a` but it is returning data with lifetime `'b`
   |
   = help: consider adding the following bound: `'b: 'a`
error: aborting due to 3 previous errors
------------------------------------------


