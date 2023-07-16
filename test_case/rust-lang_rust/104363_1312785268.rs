plain
failures:

---- [ui] src/test/ui/macros/rfc-2011-nicer-assert-messages/all-expr-kinds.rs stdout ----
normalized stderr:
warning: unnecessary allocation, use `&` instead
   |
   |
LL |     [ box elem == box 3 ] => "Assertion failed: box elem == box 3"
   |
   = note: `#[warn(unused_allocation)]` on by default


warning: unnecessary allocation, use `&` instead
   |
   |
LL |     [ box elem == box 3 ] => "Assertion failed: box elem == box 3"

warning: 2 warnings emitted


---
To only update this specific test, also pass `--test-args macros/rfc-2011-nicer-assert-messages/all-expr-kinds.rs`

error: 1 errors occurred comparing output.
status: exit status: 0
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/macros/rfc-2011-nicer-assert-messages/all-expr-kinds.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "-O" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-2011-nicer-assert-messages/all-expr-kinds/a" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/macros/rfc-2011-nicer-assert-messages/all-expr-kinds/auxiliary" "--edition=2021"
stdout: none
--- stderr -------------------------------
warning: unnecessary allocation, use `&` instead
   |
   |
LL |     [ box elem == box 3 ] => "Assertion failed: box elem == box 3"
   |
   = note: `#[warn(unused_allocation)]` on by default


warning: unnecessary allocation, use `&` instead
   |
   |
LL |     [ box elem == box 3 ] => "Assertion failed: box elem == box 3"

warning: 2 warnings emitted
------------------------------------------

