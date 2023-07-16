plain
1 error[E0308]: mismatched types
-   --> $DIR/issue-103783-array-length.rs:23:34
+   --> $DIR/issue-103783-array-length.rs:21:34
3    |
4 LL |     type NaughtyLenArray = [u32; 3.14159];
5    |                                  ^^^^^^^ expected `usize`, found floating-point number

The actual stderr differed from the expected stderr.
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/arrays/issue-103783-array-length/issue-103783-array-length.stderr
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/arrays/issue-103783-array-length/issue-103783-array-length.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args transmutability/arrays/issue-103783-array-length.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/transmutability/arrays/issue-103783-array-length.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/arrays/issue-103783-array-length" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/transmutability/arrays/issue-103783-array-length/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/transmutability/arrays/issue-103783-array-length.rs:21:34
   |
   |
LL |     type NaughtyLenArray = [u32; 3.14159]; //~ ERROR mismatched types
   |                                  ^^^^^^^ expected `usize`, found floating-point number
error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
------------------------------------------
