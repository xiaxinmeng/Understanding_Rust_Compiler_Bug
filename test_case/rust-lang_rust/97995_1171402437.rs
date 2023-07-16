plain
1 error[E0080]: it is undefined behavior to use this value
-   --> $DIR/invalid-union.rs:40:1
+   --> $DIR/invalid-union.rs:41:1
3    |
4 LL | fn main() {
5    | ^^^^^^^^^ constructing invalid value at .<deref>.y.<enum-variant(B)>.0: encountered `UnsafeCell` in a `const`
10            }
11 
12 error: erroneous constant used
-   --> $DIR/invalid-union.rs:41:25
-   --> $DIR/invalid-union.rs:41:25
+   --> $DIR/invalid-union.rs:42:25
14    |
15 LL |     let _: &'static _ = &C;
16    |                         ^^ referenced constant has errors
24 For more information about this error, try `rustc --explain E0080`.
24 For more information about this error, try `rustc --explain E0080`.
25 Future incompatibility report: Future breakage diagnostic:
26 error: erroneous constant used
-   --> $DIR/invalid-union.rs:41:25
28    |
28    |
29 LL |     let _: &'static _ = &C;
30    |                         ^^ referenced constant has errors

The actual 32bit.stderr differed from the expected 32bit.stderr.
The actual 32bit.stderr differed from the expected 32bit.stderr.
Actual 32bit.stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/invalid-union.32bit.stderr
To only update this specific test, also pass `--test-args consts/invalid-union.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/consts/invalid-union.rs" "-Zthreads=1" "--target=i686-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/i686-unknown-linux-gnu/native/rust-test-helpers" "-Clinker=x86_64-linux-gnu-gcc" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/consts/invalid-union/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0080]: it is undefined behavior to use this value
   |
   |
LL | fn main() { //~ ERROR it is undefined behavior to use this value
   | ^^^^^^^^^ constructing invalid value at .<deref>.y.<enum-variant(B)>.0: encountered `UnsafeCell` in a `const`
   |
   = note: The rules on what exactly is undefined behavior aren't clear, so this check might be overzealous. Please open an issue on the rustc repository if you believe it should not be considered undefined behavior.
   = note: the raw bytes of the constant (size: 4, align: 4) {
           }

error: erroneous constant used
  --> /checkout/src/test/ui/consts/invalid-union.rs:42:25
  --> /checkout/src/test/ui/consts/invalid-union.rs:42:25
   |
LL |     let _: &'static _ = &C; //~ ERROR erroneous constant used
   |                         ^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>

error: aborting due to 2 previous errors


For more information about this error, try `rustc --explain E0080`.
Future incompatibility report: Future breakage diagnostic:
error: erroneous constant used
   |
   |
LL |     let _: &'static _ = &C; //~ ERROR erroneous constant used
   |                         ^^ referenced constant has errors
   |
   = note: `#[deny(const_err)]` on by default
   = note: for more information, see issue #71800 <https://github.com/rust-lang/rust/issues/71800>
------------------------------------------


