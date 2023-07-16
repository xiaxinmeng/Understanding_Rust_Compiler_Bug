plain
...........................iii.......................................................... 13552/13598
..............................................
failures:

---- [ui] src/test/ui/structs/incomplete-fn-in-struct-definition.rs stdout ----

1 error: expected identifier, found keyword `fn`
2   --> $DIR/incomplete-fn-in-struct-definition.rs:4:5
3    |
---
To only update this specific test, also pass `--test-args structs/incomplete-fn-in-struct-definition.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/structs/incomplete-fn-in-struct-definition.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/incomplete-fn-in-struct-definition" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/structs/incomplete-fn-in-struct-definition/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found keyword `fn`
  --> /checkout/src/test/ui/structs/incomplete-fn-in-struct-definition.rs:4:5
LL | struct S {
   |        - while parsing this struct
   |        - while parsing this struct
LL |     fn: u8 //~ ERROR expected identifier, found keyword `fn`
   |
   |
help: escape `fn` to use it as an identifier
   |
LL |     r#fn: u8 //~ ERROR expected identifier, found keyword `fn`

error: aborting due to previous error
------------------------------------------

