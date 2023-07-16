plain

1 error: missing code example in this documentation
2   --> $DIR/lint-missing-doc-code-example.rs:19:1
3    |
- LL | / pub mod module1 {
- LL | | }
-    | |_^
+ LL | pub mod module1 {
7    |
8 note: the lint level is defined here
9   --> $DIR/lint-missing-doc-code-example.rs:2:9

---
To only update this specific test, also pass `--test-args lint-missing-doc-code-example.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-missing-doc-code-example" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/lint-missing-doc-code-example/auxiliary"
stdout: none
--- stderr -------------------------------
error: missing code example in this documentation
   |
   |
LL | pub mod module1 { //~ ERROR
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs:2:9
   |
   |
LL | #![deny(rustdoc::missing_doc_code_examples)]

error: missing code example in this documentation
  --> /checkout/src/test/rustdoc-ui/lint-missing-doc-code-example.rs:37:3
   |
