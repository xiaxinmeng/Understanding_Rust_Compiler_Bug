plain
To only update this specific test, also pass `--test-args deny-missing-docs-crate.rs`

error: 1 errors occurred comparing output.
status: exit status: 1
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/deny-missing-docs-crate/auxiliary"
stdout: none
--- stderr -------------------------------
  --> /checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs:1:1
   |
   |
LL | / #![deny(missing_docs)] //~ ERROR
LL | |
LL | | pub struct Foo; //~ ERROR
   |
note: the lint level is defined here
  --> /checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs:1:9
   |
   |
LL | #![deny(missing_docs)] //~ ERROR

error: missing documentation for a struct
  --> /checkout/src/test/rustdoc-ui/deny-missing-docs-crate.rs:3:1
   |
   |
LL | pub struct Foo; //~ ERROR

error: aborting due to 2 previous errors
------------------------------------------

