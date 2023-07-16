plain
---- [ui] src/test/ui/suggestions/const-in-struct-pat.rs stdout ----
diff of stderr:

9    |                 |
10    |                 expected struct `String`, found struct `foo`
11    |                 `foo` is interpreted as a unit struct, not a new binding
- help: bind the struct field to a different name instead
-    |
-    |
- LL |     let Thing { foo: other_foo } = t;
-    |                    +++++++++++
+    |                 help: introduce a new binding instead: `other_foo`
18 error: aborting due to previous error
19 


---
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/suggestions/const-in-struct-pat.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/const-in-struct-pat" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/suggestions/const-in-struct-pat/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/suggestions/const-in-struct-pat.rs:8:17
   |
LL | struct foo;
   | ---------- unit struct defined here
   | ---------- unit struct defined here
...
LL |     let Thing { foo } = t; //~ ERROR mismatched types
   |                 ^^^     - this expression has type `Thing`
   |                 expected struct `String`, found struct `foo`
   |                 expected struct `String`, found struct `foo`
   |                 `foo` is interpreted as a unit struct, not a new binding
   |                 help: introduce a new binding instead: `other_foo`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
