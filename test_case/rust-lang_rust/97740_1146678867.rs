plain
diff of stderr:

5    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7 note: ...which requires const-evaluating + checking `FOO`...
-   --> $DIR/recursive-static-definition.rs:1:1
9    |
10 LL | pub static FOO: u32 = FOO;
-    | ^^^^^^^^^^^^^^^^^^^^^^^^^^
+    |                       ^^^
+    |                       ^^^
12    = note: ...which again requires const-evaluating + checking `FOO`, completing the cycle
13    = note: cycle used when running analysis passes on this crate


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-static-definition/recursive-static-definition.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-static-definition/recursive-static-definition.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args recursion/recursive-static-definition.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/recursion/recursive-static-definition.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-static-definition" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/recursion/recursive-static-definition/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0391]: cycle detected when const-evaluating + checking `FOO`
   |
LL | pub static FOO: u32 = FOO;
   | ^^^^^^^^^^^^^^^^^^^^^^^^^^
   |
   |
note: ...which requires const-evaluating + checking `FOO`...
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
LL | pub static FOO: u32 = FOO;
   |                       ^^^
   |                       ^^^
   = note: ...which again requires const-evaluating + checking `FOO`, completing the cycle
   = note: cycle used when running analysis passes on this crate
error: aborting due to previous error

For more information about this error, try `rustc --explain E0391`.
------------------------------------------
