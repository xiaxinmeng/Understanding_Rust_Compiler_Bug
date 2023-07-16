plain

6    |
7    = note:   expected enum `Result<(), i32>`
8            found unit type `()`
- help: try wrapping the expression in `Ok`
+ help: try adding an expression at the end of the block
10    |
- LL |     Ok(a().await)
-    |     +++         +
+ LL ~     a().await;
+ LL ~     Ok(())
13 
14 error: aborting due to previous error
15 



The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/proper-span-for-type-error/proper-span-for-type-error.stderr
diff of fixed:

5 async fn a() {}
6 
7 async fn foo() -> Result<(), i32> {
-     Ok(a().await) //~ ERROR mismatched types
+     a().await;
+     Ok(()) //~ ERROR mismatched types
10 
11 fn main() {}



The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/proper-span-for-type-error/proper-span-for-type-error.fixed
To only update this specific test, also pass `--test-args async-await/proper-span-for-type-error.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/async-await/proper-span-for-type-error.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/proper-span-for-type-error" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/async-await/proper-span-for-type-error/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
------------------------------------------
error[E0308]: mismatched types
  --> /checkout/src/test/ui/async-await/proper-span-for-type-error.rs:8:5
   |
LL |     a().await //~ ERROR mismatched types
   |     ^^^^^^^^^ expected enum `Result`, found `()`
   = note:   expected enum `Result<(), i32>`
           found unit type `()`
help: try adding an expression at the end of the block
   |
   |
LL ~     a().await;
LL ~     Ok(()) //~ ERROR mismatched types

error: aborting due to previous error

For more information about this error, try `rustc --explain E0308`.
