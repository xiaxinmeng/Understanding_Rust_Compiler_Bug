plain

---- [ui] src/test/ui/inference/issue-83606.rs stdout ----
diff of stderr:

1 error[E0282]: type annotations needed for `[usize; _]`
3    |
3    |
- LL |     let _ = foo("foo"); //<- Do not suggest `foo::<N>("foo");`!
+ LL |     let _ = foo("foo");
5    |         -   ^^^ cannot infer the value of const parameter `N` declared on the function `foo`
6    |         |
7    |         consider giving this pattern the explicit type `[_; N]`, where the const parameter `N` is specified

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-83606/issue-83606.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args inference/issue-83606.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/inference/issue-83606.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-83606" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/inference/issue-83606/auxiliary"
stdout: none
--- stderr -------------------------------
error[E0282]: type annotations needed for `[usize; _]`
   |
LL |     let _ = foo("foo");
LL |     let _ = foo("foo");
   |         -   ^^^ cannot infer the value of const parameter `N` declared on the function `foo`
   |         |
   |         consider giving this pattern the explicit type `[_; N]`, where the const parameter `N` is specified
help: consider specifying the const argument
   |
   |
LL |     let _ = foo::<N>("foo");

error: aborting due to previous error

For more information about this error, try `rustc --explain E0282`.
