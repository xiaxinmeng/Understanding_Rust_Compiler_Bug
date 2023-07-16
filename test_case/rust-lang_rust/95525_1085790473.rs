plain

---- [ui] ui/impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle-2.rs stdout ----
diff of stderr:

5    |     ^^^ no implementation for `Bar == (Bar, i32)`
6    |
7    = help: the trait `PartialEq<(Bar, i32)>` is not implemented for `Bar`
- help: consider annotating `Bar` with `#[derive(PartialEq)]`
-    |
- LL | #[derive(PartialEq)]
12 
13 error: aborting due to previous error
14 

---
To only update this specific test, also pass `--test-args impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/impl-trait/recursive-type-alias-impl-trait-declaration-too-subtle-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0277]: can't compare `Bar` with `(Bar, i32)`
   |
   |
LL |     Bar //~ ERROR can't compare `Bar` with `(Bar, i32)`
   |     ^^^ no implementation for `Bar == (Bar, i32)`
   |
   = help: the trait `PartialEq<(Bar, i32)>` is not implemented for `Bar`
error: aborting due to previous error

For more information about this error, try `rustc --explain E0277`.
------------------------------------------
