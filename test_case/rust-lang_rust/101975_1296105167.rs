plain

---- [ui] src/test/ui/resolve/issue-101749.rs stdout ----
diff of stderr:

4 LL |     let _ = rect::area();
5    |             ^^^^ use of undeclared crate or module `rect`
6 
- error: `rect` is not a crate or module
-   --> $DIR/issue-101749.rs:18:13
-    |
- LL |     let _ = rect::area();
-    |
-    |
- warning: ident `rect` is defined at here
-   --> $DIR/issue-101749.rs:16:9
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |
- LL |     let rect = Rectangle::new(3, 4);
- help: maybe you meant to call instance method
-    |
-    |
- LL |     let _ = rect.area();
- 
- error: aborting due to 2 previous errors
+ error: aborting due to previous error
24 
---
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-101749/issue-101749.stderr
diff of fixed:

15 fn main() {
16     let rect = Rectangle::new(3, 4);
17     //~^ WARNING ident `rect` is defined at here
-     let _ = rect.area();
+     let _ = rect::area();
19     //~^ ERROR failed to resolve: use of undeclared crate or module `rect`
20     //~| ERROR `rect` is not a crate or module


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-101749/issue-101749.fixed
To only update this specific test, also pass `--test-args resolve/issue-101749.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-101749.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-101749" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-101749/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0433]: failed to resolve: use of undeclared crate or module `rect`
   |
   |
LL |     let _ = rect::area();
   |             ^^^^ use of undeclared crate or module `rect`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0433`.
------------------------------------------
