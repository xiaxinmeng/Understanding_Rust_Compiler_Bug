plain
4 LL |         this.x
-    |         ^^^^
-    |         |
-    |         not found in this scope
-    |         help: you might have meant to use `self` here instead
+    |         ^^^^ help: you might have meant to use `self` here instead
+    |
+ help: the binding `this` is available in a different scope in the same function
+    |
+ LL |         let this = a;
+    |             ^^^^
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
---
To only update this specific test, also pass `--test-args self/suggest-self.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/self/suggest-self.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/suggest-self" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/self/suggest-self/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0425]: cannot find value `this` in this scope
   |
LL |         this.x
LL |         this.x
   |         ^^^^ help: you might have meant to use `self` here instead
   |
help: the binding `this` is available in a different scope in the same function
   |
LL |         let this = a;
   |             ^^^^


error[E0425]: cannot find value `this` in this scope
  --> /checkout/src/test/ui/self/suggest-self.rs:26:9
   |
LL |         this.foo()
   |         ^^^^
   |         |
   |         not found in this scope
   |         help: you might have meant to use `self` here instead

error[E0425]: cannot find value `my` in this scope
   |
   |
LL |         my.bar()
   |         |
   |         not found in this scope
   |         not found in this scope
   |         help: you might have meant to use `self` here instead
error: aborting due to 3 previous errors

For more information about this error, try `rustc --explain E0425`.
------------------------------------------
