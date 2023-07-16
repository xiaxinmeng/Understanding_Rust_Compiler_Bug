plain
diff of stderr:

5    |                 ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
6    |
7    = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_23122_2`)
- note: required for `GetNext<<<<<<... as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next>` to implement `Next`
+ note: required for `GetNext<<<<<<<... as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next>` to implement `Next`
10    |
10    |
11 LL | impl<T: Next> Next for GetNext<T> {

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-2/issue-23122-2.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args issues/issue-23122-2.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/issues/issue-23122-2.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-2" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-2/auxiliary"
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
stdout: none
--- stderr -------------------------------
error[E0275]: overflow evaluating the requirement `<<<<<<<... as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next: Sized`
   |
   |
LL |     type Next = <GetNext<T::Next> as Next>::Next;
   |
   |
   = help: consider increasing the recursion limit by adding a `#![recursion_limit = "256"]` attribute to your crate (`issue_23122_2`)
note: required for `GetNext<<<<<<<... as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next as Next>::Next>` to implement `Next`
   |
   |
LL | impl<T: Next> Next for GetNext<T> {
   |               ^^^^     ^^^^^^^^^^
   = note: the full type name has been written to '/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/issues/issue-23122-2/issue-23122-2.long-type-15191332486497938525.txt'
error: aborting due to previous error

For more information about this error, try `rustc --explain E0275`.
------------------------------------------
