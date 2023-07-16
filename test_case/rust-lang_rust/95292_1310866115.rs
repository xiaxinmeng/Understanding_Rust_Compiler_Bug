plain

---- [ui] src/test/ui/rfc-2632-const-trait-impl/specializing-constness.rs stdout ----
diff of stderr:

4 LL | impl<T: Default + Sup> A for T {
6 
6 
- error: missing `~const` qualifier
+ error: missing `~const` qualifier for specialization
9    |
9    |
10 LL | impl<T: Default + Sup> A for T {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/specializing-constness/specializing-constness.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args rfc-2632-const-trait-impl/specializing-constness.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2632-const-trait-impl/specializing-constness.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/specializing-constness" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2632-const-trait-impl/specializing-constness/auxiliary"
stdout: none
--- stderr -------------------------------
error: cannot specialize on const impl with non-const impl
   |
   |
LL | impl<T: Default + Sup> A for T {


error: missing `~const` qualifier for specialization
   |
   |
LL | impl<T: Default + Sup> A for T {

error: aborting due to 2 previous errors
------------------------------------------

