plain
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
10 LL |     let a = 42._;
11    |                ^
+    |
+ help: If the number is meant to be a floating point number, consider adding a `0` after the period
+ LL |     let a = 42.0_;
+    |                +
12 
13 error: aborting due to 2 previous errors
---
To only update this specific test, also pass `--test-args parser/underscore-suffix-for-float.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/underscore-suffix-for-float.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/underscore-suffix-for-float" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/underscore-suffix-for-float/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found reserved identifier `_`
   |
   |
LL |     let a = 42._; //~ ERROR expected identifier, found reserved identifier `_`


error[E0610]: `{integer}` is a primitive type and therefore doesn't have fields
   |
   |
LL |     let a = 42._; //~ ERROR expected identifier, found reserved identifier `_`
   |
   |
help: If the number is meant to be a floating point number, consider adding a `0` after the period
   |
LL |     let a = 42.0_; //~ ERROR expected identifier, found reserved identifier `_`

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0610`.
