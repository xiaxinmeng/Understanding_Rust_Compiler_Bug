plain
diff of stderr:

2   --> $DIR/issue-6458-3.rs:4:5
3    |
4 LL |     mem::transmute(0);
-    |     ^^^^^^^^^^^^^^ cannot infer type of the type parameter `U` declared on the function `transmute`
+    |     ^^^^^^^^^^^^^^ cannot infer type of the type parameter `Dst` declared on the function `transmute`
7 help: consider specifying the generic arguments
8    |


- LL |     mem::transmute::<i32, U>(0);
-    |                   ++++++++++
+ LL |     mem::transmute::<i32, Dst>(0);
11 
12 error: aborting due to previous error
13 

---
To only update this specific test, also pass `--test-args issues/issue-6458-3.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-6458-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6458-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-6458-3/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/issues/issue-6458-3.rs:4:5
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL |     mem::transmute(0);
LL |     mem::transmute(0);
   |     ^^^^^^^^^^^^^^ cannot infer type of the type parameter `Dst` declared on the function `transmute`
help: consider specifying the generic arguments
   |
   |
LL |     mem::transmute::<i32, Dst>(0);

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
