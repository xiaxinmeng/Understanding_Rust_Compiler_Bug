plain
iii..................................................................................... 13288/13307
...................
failures:

---- [ui] src/test/ui/rfc-2011-nicer-assert-messages/all-expr-kinds.rs stdout ----
error: test compilation failed although it shouldn't!
status: exit status: 1
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/rfc-2011-nicer-assert-messages/all-expr-kinds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2011-nicer-assert-messages/all-expr-kinds/a" "-Crpath" "-O" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--edition=2021" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/rfc-2011-nicer-assert-messages/all-expr-kinds/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/ui/rfc-2011-nicer-assert-messages/all-expr-kinds.rs:8:1
   |
LL | extern crate common;
   | ^^^^^^^^^^^^^^^^^^^^ can't find crate
