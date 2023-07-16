plain

---- [ui] src/test/ui/binop/issue-77910-1.rs stdout ----
diff of stderr:

12 error[E0277]: `for<'r> fn(&'r i32) -> &'r i32 {foo}` doesn't implement `Debug`
14    |
14    |
+ LL | fn foo(s: &i32) -> &i32 {
+ ...
+ ...
15 LL |     assert_eq!(foo, y);
16    |     ^^^^^^^^^^^^^^^^^^ `for<'r> fn(&'r i32) -> &'r i32 {foo}` cannot be formatted using `{:?}` because it doesn't implement `Debug`


18    = help: the trait `Debug` is not implemented for `for<'r> fn(&'r i32) -> &'r i32 {foo}`
-    = help: the following other types implement trait `Debug`:
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
-              extern "C" fn() -> Ret
-              extern "C" fn(A) -> Ret
-              extern "C" fn(A, ...) -> Ret
-              extern "C" fn(A, B) -> Ret
-              extern "C" fn(A, B, ...) -> Ret
-              extern "C" fn(A, B, C) -> Ret
-              extern "C" fn(A, B, C, ...) -> Ret
-              extern "C" fn(A, B, C, D) -> Ret
-            and 68 others
+    = help: use parentheses to call the function: `foo(s)`
29    = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
31 error: aborting due to 2 previous errors


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1/issue-77910-1.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args binop/issue-77910-1.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/binop/issue-77910-1.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/binop/issue-77910-1/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0369]: binary operation `==` cannot be applied to type `for<'r> fn(&'r i32) -> &'r i32 {foo}`
   |
   |
LL |     assert_eq!(foo, y);
   |     |
   |     |
   |     for<'r> fn(&'r i32) -> &'r i32 {foo}
   |
   |
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)

error[E0277]: `for<'r> fn(&'r i32) -> &'r i32 {foo}` doesn't implement `Debug`
   |
   |
LL | fn foo(s: &i32) -> &i32 {
...
...
LL |     assert_eq!(foo, y);
   |     ^^^^^^^^^^^^^^^^^^ `for<'r> fn(&'r i32) -> &'r i32 {foo}` cannot be formatted using `{:?}` because it doesn't implement `Debug`
   |
   = help: the trait `Debug` is not implemented for `for<'r> fn(&'r i32) -> &'r i32 {foo}`
   = help: use parentheses to call the function: `foo(s)`
   = note: this error originates in the macro `assert_eq` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to 2 previous errors

Some errors have detailed explanations: E0277, E0369.
For more information about an error, try `rustc --explain E0277`.
