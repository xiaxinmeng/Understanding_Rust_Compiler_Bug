plain

6    |
7 help: try giving this closure an explicit return type
8    |
- LL |     unbound_drop(|| -> [_; 0] { [] });
-    |                     +++++++++++    +
+ LL |     unbound_drop(|| -> [_; 0] { []});
+    |                     +++++++++++   +
12 error: aborting due to previous error
13 


---

error: 1 errors occurred comparing output.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/suggest-closure-return-type-3.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-closure-return-type-3" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/suggest-closure-return-type-3/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0282]: type annotations needed for `[_; 0]`
   |
   |
LL |     unbound_drop(|| []);
   |                  ^^ -- type must be known at this point
help: try giving this closure an explicit return type
   |
   |
LL |     unbound_drop(|| -> [_; 0] { []});
   |                     +++++++++++   +
error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
------------------------------------------
