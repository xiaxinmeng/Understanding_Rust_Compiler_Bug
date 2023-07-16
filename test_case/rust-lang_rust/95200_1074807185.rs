plain
..................................................iii............................................... 12700/12755
.......................................................
failures:

---- [ui] ui/const-generics/ice-const-generic-function-return-ty.rs stdout ----


1 error: expected one of `(`, `::`, `<`, or `>`, found `;`
-   --> $DIR/const-generic-function-return-ty.rs:2:46
+   --> $DIR/ice-const-generic-function-return-ty.rs:2:46
3    |
4 LL | fn return_ty() -> impl Into<<() as Reexported;
5    |                                              ^ expected one of `(`, `::`, `<`, or `>`
6 
7 error: aborting due to previous error
+ 
8 
8 


The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/ice-const-generic-function-return-ty/ice-const-generic-function-return-ty.stderr
To only update this specific test, also pass `--test-args const-generics/ice-const-generic-function-return-ty.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/const-generics/ice-const-generic-function-return-ty.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/ice-const-generic-function-return-ty" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/const-generics/ice-const-generic-function-return-ty/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected one of `(`, `::`, `<`, or `>`, found `;`
  --> /checkout/src/test/ui/const-generics/ice-const-generic-function-return-ty.rs:2:46
   |
LL | fn return_ty() -> impl Into<<() as Reexported;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |                                              ^ expected one of `(`, `::`, `<`, or `>`
error: aborting due to previous error
------------------------------------------


