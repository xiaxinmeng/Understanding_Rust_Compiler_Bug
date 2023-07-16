plain

---- [ui] src/test/ui/typeck/issue-104513-ice.rs stdout ----
diff of stderr:

6 LL |     let _: S<impl Oops> = S;
8 
8 
- error[E0562]: `impl Trait` only allowed in function and inherent method return types, not in variable binding
+ error[E0562]: `impl Trait` not allowed within variable binding
11    |
11    |
12 LL |     let _: S<impl Oops> = S;

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-104513-ice/issue-104513-ice.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args typeck/issue-104513-ice.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/typeck/issue-104513-ice.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-104513-ice" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/typeck/issue-104513-ice/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0405]: cannot find trait `Oops` in this scope
   |
LL | fn f() {
LL | fn f() {
   |     - help: you might be missing a type parameter: `<Oops>`
LL |     let _: S<impl Oops> = S; //~ ERROR cannot find trait `Oops` in this scope


error[E0562]: `impl Trait` not allowed within variable binding
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
   |
   |
LL |     let _: S<impl Oops> = S; //~ ERROR cannot find trait `Oops` in this scope

error: aborting due to 2 previous errors

Some errors have detailed explanations: E0405, E0562.
