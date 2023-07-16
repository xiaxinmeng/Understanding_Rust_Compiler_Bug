plain
..........................iii....................................................................... 12600/12631
...............................
failures:

---- [ui] ui/generic-associated-types/bugs/issue-87803.rs stdout ----


1 error[E0195]: lifetime parameters or bounds on method `scan` do not match the trait declaration
-   --> $DIR/issue-87803.rs:20:12
3    |
3    |
4 LL |     fn scan<'a>(&mut self, i : Self::Input<'a>) -> Self::Token<'a>;
5    |            ---- lifetimes in impl do not match this method in trait

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-87803/issue-87803.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args generic-associated-types/bugs/issue-87803.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/generic-associated-types/bugs/issue-87803.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-87803" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/generic-associated-types/bugs/issue-87803/auxiliary"
------------------------------------------

------------------------------------------
stderr:
stderr:
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
------------------------------------------
error[E0195]: lifetime parameters or bounds on method `scan` do not match the trait declaration
  --> /checkout/src/test/ui/generic-associated-types/bugs/issue-87803.rs:21:12
   |
LL |     fn scan<'a>(&mut self, i : Self::Input<'a>) -> Self::Token<'a>;
   |            ---- lifetimes in impl do not match this method in trait
...
LL |     fn scan<'a>(&mut self, s : &'a str) -> &'a str { //~ lifetime parameters
   |            ^^^^ lifetimes do not match method in trait
error: aborting due to previous error

For more information about this error, try `rustc --explain E0195`.

