plain

---- [ui] ui/error-codes/E0081.rs stdout ----
diff of stderr:

7 LL |     X = 3,
8    |         ^ enum already has `3`
9 
- error[E0081]: discriminant value `1` already exists
+ error[E0081]: discriminant value `255` already exists
11   --> $DIR/E0081.rs:14:9
13 LL |     P = 257,

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-    |         --- first use of `1` (overflowed from `257`)
+    |         --- first use of `255` (overflowed from `257`)
16 LL |     X = 513,
16 LL |     X = 513,
-    |         ^^^ enum already has `1` (overflowed from `513`)
+    |         ^^^ enum already has `255` (overflowed from `513`)
19 error: aborting due to 2 previous errors
20 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0081/E0081.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args error-codes/E0081.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0081.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0081" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0081/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0081]: discriminant value `3` already exists
   |
LL |     P = 3,
   |         - first use of `3`
   |         - first use of `3`
LL |     //~^ NOTE first use of `3`
LL |     X = 3,
   |         ^ enum already has `3`

error[E0081]: discriminant value `255` already exists
   |
LL |     P = 257,
LL |     P = 257,
   |         --- first use of `255` (overflowed from `257`)
LL |     //~^ NOTE first use of `1` (overflowed from `257`)
LL |     X = 513,
   |         ^^^ enum already has `255` (overflowed from `513`)
error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0081`.
------------------------------------------
