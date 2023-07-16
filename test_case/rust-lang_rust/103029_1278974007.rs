plain

12 LL | | );
13    | |_- in this macro invocation
14    |
-    = help: consider increasing the expansion grow limit by adding a `#![expansion_growth_limit = "3000"]` attribute to your crate (`issue_95698`)
+    = help: consider increasing the expansion grow limit by adding a `#![expansion_growth_limit = "12000"]` attribute to your crate (`issue_95698`)
16    = note: this error originates in the macro `from_cow_impls` (in Nightly builds, run with -Z macro-backtrace for more info)
18 error: aborting due to previous error


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-95698/issue-95698.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args limits/issue-95698.rs`

Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/limits/issue-95698.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-95698" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/limits/issue-95698/auxiliary"
stdout: none
--- stderr -------------------------------
error: expansion grow limit reached while expanding `from_cow_impls!`
   |
   |
LL | /         from_cow_impls!(
LL | |             $( $from, Cow::from ),+
LL | |         );
...
...
LL | / from_cow_impls!(
LL | |     &'a [u8], /*callback,*/
LL | |     Vec<u8>, /*callback,*/
LL | | );
   |
   |
   = help: consider increasing the expansion grow limit by adding a `#![expansion_growth_limit = "12000"]` attribute to your crate (`issue_95698`)
   = note: this error originates in the macro `from_cow_impls` (in Nightly builds, run with -Z macro-backtrace for more info)
error: aborting due to previous error
------------------------------------------


