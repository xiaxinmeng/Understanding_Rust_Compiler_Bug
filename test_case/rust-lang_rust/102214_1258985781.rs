plain

---- [ui] src/test/ui/macros/issue-83344.rs stdout ----
diff of stderr:

1 error: 1 positional argument in format string, but no arguments were given
3    |
- LL |     println!("{}\
-    |               ^^
+ LL |       println!("{}\
+ LL |       println!("{}\
+    |  _______________^
+ LL | | ");
6 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
7 error: aborting due to previous error
8 
---
To only update this specific test, also pass `--test-args macros/issue-83344.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/issue-83344.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-83344" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/issue-83344/auxiliary"
stdout: none
--- stderr -------------------------------
error: 1 positional argument in format string, but no arguments were given
   |
LL |       println!("{}\
   |  _______________^
   |  _______________^
LL | | "); //~^ ERROR: 1 positional argument in format string, but no arguments were given

error: aborting due to previous error
------------------------------------------

