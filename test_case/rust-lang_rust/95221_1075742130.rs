plain
diff of stderr:

2   --> $DIR/E0396-fixed.rs:5:28
3    |
4 LL | const VALUE: u8 = unsafe { *REG_ADDR };
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |                            ^^^^^^^^^ 0x5f3759df is not a valid pointer
+    |                            ^^^^^^^^^ dereferencing pointer failed: 0x5f3759df is not a valid pointer
7 error: aborting due to previous error
8 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0396-fixed/E0396-fixed.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0396-fixed.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0396-fixed.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0396-fixed" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0396-fixed/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: evaluation of constant value failed
   |
   |
LL | const VALUE: u8 = unsafe { *REG_ADDR };
   |                            ^^^^^^^^^ dereferencing pointer failed: 0x5f3759df is not a valid pointer
error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
------------------------------------------
