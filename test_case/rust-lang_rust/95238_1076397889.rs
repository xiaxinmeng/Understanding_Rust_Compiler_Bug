plain
........................................................iii......................................... 12700/12761
.............................................................
failures:

---- [ui] ui/structs-enums/struct-enum-ignoring-field-with-underscore.rs stdout ----

1 error: expected identifier, found reserved identifier `_`
-   --> $DIR/issue-83263-struct-enum-ignoring-field-with-underscore.rs:9:27
+   --> $DIR/struct-enum-ignoring-field-with-underscore.rs:9:27
+   --> $DIR/struct-enum-ignoring-field-with-underscore.rs:9:27
3    |
4 LL |     if let Some(Foo::Bar {_}) = foo {}
5    |                           ^ expected identifier, found reserved identifier
6 
7 error[E0027]: pattern does not mention field `bar`
-   --> $DIR/issue-83263-struct-enum-ignoring-field-with-underscore.rs:9:17
+   --> $DIR/struct-enum-ignoring-field-with-underscore.rs:9:17
+   --> $DIR/struct-enum-ignoring-field-with-underscore.rs:9:17
9    |
10 LL |     if let Some(Foo::Bar {_}) = foo {}
11    |                 ^^^^^^^^^^^^ missing field `bar`
Some tests failed in compiletest suite=ui mode=ui host=x86_64-unknown-linux-gnu target=x86_64-unknown-linux-gnu

The actual stderr differed from the expected stderr.
Actual stderr saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/struct-enum-ignoring-field-with-underscore/struct-enum-ignoring-field-with-underscore.stderr
To update references, rerun the tests and pass the `--bless` flag
To update references, rerun the tests and pass the `--bless` flag
To only update this specific test, also pass `--test-args structs-enums/struct-enum-ignoring-field-with-underscore.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs-enums/struct-enum-ignoring-field-with-underscore.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/struct-enum-ignoring-field-with-underscore" "-A" "unused" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs-enums/struct-enum-ignoring-field-with-underscore/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found reserved identifier `_`
  --> /checkout/src/test/ui/structs-enums/struct-enum-ignoring-field-with-underscore.rs:9:27
   |
LL |     if let Some(Foo::Bar {_}) = foo {}
   |                           ^ expected identifier, found reserved identifier
error[E0027]: pattern does not mention field `bar`
  --> /checkout/src/test/ui/structs-enums/struct-enum-ignoring-field-with-underscore.rs:9:17
   |
   |
LL |     if let Some(Foo::Bar {_}) = foo {}
   |                 ^^^^^^^^^^^^ missing field `bar`
help: include the missing field in the pattern
   |
   |
LL |     if let Some(Foo::Bar {_, bar }) = foo {}
   |                            ~~~~~~~
help: if you don't care about this missing field, you can explicitly ignore it
   |
LL |     if let Some(Foo::Bar {_, .. }) = foo {}

error: aborting due to 2 previous errors

For more information about this error, try `rustc --explain E0027`.
