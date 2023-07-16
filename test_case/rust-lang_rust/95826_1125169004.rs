plain
diff of stderr:

8   --> $DIR/ptr_arith.rs:15:13
9    |
10 LL |     let x = &0 as *const _ as usize;
-    |             ^^^^^^^^^^^^^^^^^^^^^^^ unable to turn pointer into raw bytes
+    |             ^^^^^^^^^^^^^^^^^^^^^^^ "exposing pointers" needs an rfc before being allowed inside constants
13 error[E0080]: could not evaluate static initializer
14   --> $DIR/ptr_arith.rs:23:14



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/ptr_arith/ptr_arith.stderr
To only update this specific test, also pass `--test-args consts/miri_unleashed/ptr_arith.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/ptr_arith" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Zunleash-the-miri-inside-of-you" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/miri_unleashed/ptr_arith/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: could not evaluate static initializer
   |
   |
LL |     let _v = x == x;
   |              ^^^^^^ "pointer arithmetic or comparison" needs an rfc before being allowed inside constants
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:15:13
   |
   |
LL |     let x = &0 as *const _ as usize;
   |             ^^^^^^^^^^^^^^^^^^^^^^^ "exposing pointers" needs an rfc before being allowed inside constants
error[E0080]: could not evaluate static initializer
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:23:14
   |
   |
LL |     let _v = x + 0;
   |              ^^^^^ unable to turn pointer into raw bytes
warning: skipping const checks
   |
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:9:14
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:9:14
   |
LL |     let _v = x == x;
help: skipping check that does not even have a feature gate
  --> /checkout/src/test/ui/consts/miri_unleashed/ptr_arith.rs:15:13
   |
   |
LL |     let x = &0 as *const _ as usize;

error: aborting due to 3 previous errors; 1 warning emitted

For more information about this error, try `rustc --explain E0080`.
