plain

---- [ui] src/test/ui/issues/issue-57362-2.rs stdout ----
diff of stderr:

1 error[E0599]: no function or associated item named `make_g` found for fn pointer `for<'r> fn(&'r ())` in the current scope
3    |
3    |
- LL |     let x = <fn (&())>::make_g(); 
+ LL |     let x = <fn (&())>::make_g();
5    |                         ^^^^^^ function or associated item not found in `for<'r> fn(&'r ())`
7    = help: items from traits can only be used if the trait is implemented and in scope


The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57362-2/issue-57362-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-57362-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-57362-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57362-2" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-57362-2/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0599]: no function or associated item named `make_g` found for fn pointer `for<'r> fn(&'r ())` in the current scope
   |
   |
LL |     let x = <fn (&())>::make_g();
   |                         ^^^^^^ function or associated item not found in `for<'r> fn(&'r ())`
   = help: items from traits can only be used if the trait is implemented and in scope
   = help: items from traits can only be used if the trait is implemented and in scope
note: `X` defines an item `make_g`, perhaps you need to implement it
   |
   |
LL | trait X {

error: aborting due to previous error

For more information about this error, try `rustc --explain E0599`.
