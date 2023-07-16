plain
diff of stderr:

2   --> $DIR/do-catch-suggests-try.rs:4:25
3    |
4 LL |     let _: Option<()> = do catch {};
-    |                         ^^^^^^^^ help: replace with the new syntax: `try`
+    |                         ^^^^^^^^ help: replace with the new syntax
6    |
7    = note: following RFC #2388, the new non-placeholder syntax is `try`


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/do-catch-suggests-try/do-catch-suggests-try.stderr
To only update this specific test, also pass `--test-args parser/do-catch-suggests-try.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/do-catch-suggests-try.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/do-catch-suggests-try" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/do-catch-suggests-try/auxiliary"
stdout: none
--- stderr -------------------------------
error: found removed `do catch` syntax
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     let _: Option<()> = do catch {};
   |                         ^^^^^^^^ help: replace with the new syntax
   |
   = note: following RFC #2388, the new non-placeholder syntax is `try`
error[E0308]: mismatched types
  --> /checkout/src/test/ui/parser/do-catch-suggests-try.rs:9:33
   |
   |
LL |     let _recovery_witness: () = 1; //~ ERROR mismatched types
   |                            |
   |                            expected due to this

error: aborting due to 2 previous errors
