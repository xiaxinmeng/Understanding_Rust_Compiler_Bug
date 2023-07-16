plain
1 error[E0382]: use of moved value: `x`
-   --> $DIR/issue-104870.rs:16:7
+   --> $DIR/missing-clone-for-suggestion.rs:17:7
3    |
4 LL | fn f(x: *mut u8) {
5    |      - move occurs because `x` has type `*mut u8`, which does not implement the `Copy` trait
9    |       ^ value used here after move
10    |
10    |
11 note: consider changing this parameter type in function `g` to borrow instead if owning the value isn't necessary
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-   --> $DIR/issue-104870.rs:12:12
13    |
13    |
14 LL | fn g<T>(x: T) {}
15    |    -       ^ this parameter takes ownership of the value

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/missing-clone-for-suggestion/missing-clone-for-suggestion.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args lang-items/missing-clone-for-suggestion.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/lang-items/missing-clone-for-suggestion.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/missing-clone-for-suggestion" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/lang-items/missing-clone-for-suggestion/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0382]: use of moved value: `x`
   |
   |
LL | fn f(x: *mut u8) {
   |      - move occurs because `x` has type `*mut u8`, which does not implement the `Copy` trait
LL |     g(x);
   |       - value moved here
LL |     g(x); //~ ERROR use of moved value: `x`
   |       ^ value used here after move
   |
note: consider changing this parameter type in function `g` to borrow instead if owning the value isn't necessary
   |
   |
LL | fn g<T>(x: T) {}
   |    -       ^ this parameter takes ownership of the value
   |    in this function

error: aborting due to previous error

