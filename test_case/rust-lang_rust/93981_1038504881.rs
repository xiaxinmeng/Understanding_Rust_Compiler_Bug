plain

---- [ui] ui/typeck/issue-91328.rs stdout ----
diff of stderr:

25 LL |         [a, b] => a + b,
26    |         ^^^^^^ pattern cannot match with input type `Vec<i32>`
27 
- error[E0529]: expected an array or slice, found `Box<[i32; 1]>`
+ error[E0529]: expected an array or slice, found `Box<[i32; 2]>`
30    |
31 LL |     match a {


32    |           - help: consider using `as_deref` here: `a.as_deref()`
33 LL |
34 LL |         Some([a, b]) => a + b,
-    |              ^^^^^^ pattern cannot match with input type `Box<[i32; 1]>`
+    |              ^^^^^^ pattern cannot match with input type `Box<[i32; 2]>`
37 error: aborting due to 4 previous errors
38 

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---
To only update this specific test, also pass `--test-args typeck/issue-91328.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-91328.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-91328" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-91328/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0529]: expected an array or slice, found `Vec<i32>`
   |
LL |     match r {
LL |     match r {
   |           - help: consider using `as_deref` here: `r.as_deref()`
LL |     //~^ HELP: consider using `as_deref` here
LL |         Ok([a, b]) => a + b,
   |            ^^^^^^ pattern cannot match with input type `Vec<i32>`

error[E0529]: expected an array or slice, found `Vec<i32>`
   |
LL |     match o {
LL |     match o {
   |           - help: consider using `as_deref` here: `o.as_deref()`
LL |     //~^ HELP: consider using `as_deref` here
LL |         Some([a, b]) => a + b,
   |              ^^^^^^ pattern cannot match with input type `Vec<i32>`

error[E0529]: expected an array or slice, found `Vec<i32>`
   |
LL |     match v {
LL |     match v {
   |           - help: consider slicing here: `v[..]`
LL |     //~^ HELP: consider slicing here
LL |         [a, b] => a + b,
   |         ^^^^^^ pattern cannot match with input type `Vec<i32>`

error[E0529]: expected an array or slice, found `Box<[i32; 2]>`
   |
LL |     match a {
LL |     match a {
   |           - help: consider using `as_deref` here: `a.as_deref()`
LL |     //~^ HELP: consider using `as_deref` here
LL |         Some([a, b]) => a + b,
   |              ^^^^^^ pattern cannot match with input type `Box<[i32; 2]>`
error: aborting due to 4 previous errors

For more information about this error, try `rustc --explain E0529`.

