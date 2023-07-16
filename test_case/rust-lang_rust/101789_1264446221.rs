plain
To only update this specific test, also pass `--test-args parser/removed-syntax-field-let.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/removed-syntax-field-let.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "--emit" "metadata" "-C" "prefer-dynamic" "--out-dir" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/removed-syntax-field-let" "-A" "unused" "-Crpath" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/removed-syntax-field-let/auxiliary"
stdout: none
--- stderr -------------------------------
error: expected identifier, found keyword `let`
   |
LL |     let foo: (),
   |     ^^^ expected identifier, found keyword
   |
   |
   = help: see <https://doc.rust-lang.org/book/ch05-01-defining-structs.html> for more information
help: remove the let, the `let` keyword is not allowed in struct field definitions
LL -     let foo: (),
LL +      foo: (),
   |

