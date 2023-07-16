plain
---- [ui] src/test/rustdoc-ui/unused-braces-lint.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/unused-braces-lint.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused-braces-lint" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/unused-braces-lint/auxiliary"
stdout: none
--- stderr -------------------------------
error: unnecessary braces around block return value
   |
LL | /     {
LL | |         {
   | |________^
