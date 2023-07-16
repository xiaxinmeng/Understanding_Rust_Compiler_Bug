plain
..................iii................................................................... 13200/13237
.....................................
failures:

---- [ui] src/test/ui/parser/public-instead-of-pub.rs stdout ----


1 error: expected one of `!` or `::`, found keyword `struct`
-   --> $DIR/public-instead-of-pub.rs:3:8
+   --> $DIR/public-instead-of-pub.rs:4:8
3    |
4 LL | public struct MyStruct;
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu
5    |        ^^^^^^ expected one of `!` or `::`

The actual stderr differed from the expected stderr.
The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/public-instead-of-pub/public-instead-of-pub.stderr
normalized fixed:
// Checks what happens when `public` is used instead of the correct, `pub`
// edition:2018
// run-rustfix
pub struct MyStruct;
//~^ ERROR 3:8: 3:14: expected one of `!` or `::`, found keyword `struct`
//~^^ HELP write `pub` instead of `public` to make the item public


The actual fixed differed from the expected fixed.
The actual fixed differed from the expected fixed.
Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/public-instead-of-pub/public-instead-of-pub.fixed
To only update this specific test, also pass `--test-args parser/public-instead-of-pub.rs`

error: 2 errors occurred comparing output.
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/public-instead-of-pub.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/public-instead-of-pub" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2018" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/public-instead-of-pub/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected one of `!` or `::`, found keyword `struct`
  --> /checkout/src/test/ui/parser/public-instead-of-pub.rs:4:8
   |
LL | public struct MyStruct;
   |        ^^^^^^ expected one of `!` or `::`
   |
help: write `pub` instead of `public` to make the item public
   |
LL | pub struct MyStruct;
   | ~~~
error: aborting due to previous error
------------------------------------------


