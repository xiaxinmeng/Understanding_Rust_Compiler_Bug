plain
- 4 | |     }
-   | |_____^
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   |
-   = help: consider creating a new `struct` definition instead of nesting
+    |
+ LL |   struct S1 {
+    |  ___________^
+ LL | |     struct S2 {
+ LL | |     struct S2 {
+    | |__________^
+    |
+    = help: consider creating a new `struct` definition instead of nesting
+ error: aborting due to previous error
+ 
10 

---
To only update this specific test, also pass `--test-args parser/issues/issue-101540.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/issues/issue-101540.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-101540" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/issues/issue-101540/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/parser/issues/issue-101540.rs:1:11
   |
LL |   struct S1 {
   |  ___________^
   |  ___________^
LL | |     struct S2 {
   | |__________^
   |
   = help: consider creating a new `struct` definition instead of nesting
error: aborting due to previous error
------------------------------------------


