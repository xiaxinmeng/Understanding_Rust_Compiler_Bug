plain
---- [ui] src/test/ui/proc-macro/invalid-punct-ident-1.rs stdout ----
diff of stderr:

3    |
4 LL | invalid_punct!();
+    |
+    |
+    = help: message: unsupported character `'`'`
7 error: aborting due to previous error
8 


---
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/proc-macro/invalid-punct-ident-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/proc-macro/invalid-punct-ident-1/auxiliary"
stdout: none
--- stderr -------------------------------
error: proc macro panicked
   |
   |
LL | invalid_punct!(); //~ ERROR proc macro panicked
   |
   |
   = help: message: unsupported character `'`'`
error: aborting due to previous error
------------------------------------------


