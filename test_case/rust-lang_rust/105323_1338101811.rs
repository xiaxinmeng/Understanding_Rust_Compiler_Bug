plain

---- [ui] src/test/ui/consts/invalid-union.rs stdout ----
diff of 64bit.stderr:

21 LL |     let _: &'static _ = &C;
23 
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
- note: erroneous constant used
-   --> $DIR/invalid-union.rs:43:25
-   --> $DIR/invalid-union.rs:43:25
-    |
- LL |     let _: &'static _ = &C;
- 
30 error: aborting due to previous error
31 
32 For more information about this error, try `rustc --explain E0080`.
32 For more information about this error, try `rustc --explain E0080`.


The actual 64bit.stderr differed from the expected 64bit.stderr.
Actual 64bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/invalid-union.64bit.stderr
To only update this specific test, also pass `--test-args consts/invalid-union.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/invalid-union.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | fn main() { //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^ constructing invalid value at .<deref>.y.<enum-variant(B)>.0: encountered `UnsafeCell` in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 8, align: 8) {
           }

note: erroneous constant used
  --> /checkout/src/test/ui/consts/invalid-union.rs:43:25
  --> /checkout/src/test/ui/consts/invalid-union.rs:43:25
   |
LL |     let _: &'static _ = &C;

note: erroneous constant used
  --> /checkout/src/test/ui/consts/invalid-union.rs:43:25
   |
   |
LL |     let _: &'static _ = &C;

error: aborting due to previous error

For more information about this error, try `rustc --explain E0080`.
