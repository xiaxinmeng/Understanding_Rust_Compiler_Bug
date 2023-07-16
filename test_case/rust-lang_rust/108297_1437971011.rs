plain
Prepare all required actions
Getting action download info
Download action repository 'actions/checkout@v3' (SHA:ac593985615ec2ede58e132d2e21d2b1cbd6127c)
Download action repository 'rust-lang/simpleinfra@master' (SHA:75573f9759179a720f4c3af6c9fb518ac0061dca)
Complete job name: PR (x86_64-gnu-llvm-14, false, ubuntu-20.04-xl)
git config --global core.autocrlf false
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-llvm-14
---
---- [ui] tests/rustdoc-ui/doc-comment-multi-line-cfg-attr.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doc-comment-multi-line-cfg-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-comment-multi-line-cfg-attr" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-comment-multi-line-cfg-attr/auxiliary" "--test"
running 1 test
test /checkout/tests/rustdoc-ui/doc-comment-multi-line-cfg-attr.rs - Bar (line 6) ... FAILED

failures:
failures:

---- /checkout/tests/rustdoc-ui/doc-comment-multi-line-cfg-attr.rs - Bar (line 6) stdout ----
error: mismatched closing delimiter: `)`
  --> /checkout/tests/rustdoc-ui/doc-comment-multi-line-cfg-attr.rs:7:123
   |
LL | #![cfg_attr(not(dox), deny(missing_abi,
   |                           - closing delimiter possibly meant for this
LL | fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_tests_rustdoc_ui_doc_comment_multi_line_cfg_attr_rs_6_0() {
LL | non_ascii_idents))]
   |                 ^ mismatched closing delimiter

error: unexpected closing delimiter: `}`
error: unexpected closing delimiter: `}`
  --> /checkout/tests/rustdoc-ui/doc-comment-multi-line-cfg-attr.rs:11:1
   |
LL | non_ascii_idents))]
   |                 - missing open `(` for this delimiter
...
LL | } _doctest_main__checkout_tests_rustdoc_ui_doc_comment_multi_line_cfg_attr_rs_6_0() }

error: aborting due to 2 previous errors

Couldn't compile the test.
---
---- [ui] tests/rustdoc-ui/doc-comment-multi-line-attr.rs stdout ----

error: test compilation failed although it shouldn't!
status: exit status: 101
command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustdoc" "/checkout/tests/rustdoc-ui/doc-comment-multi-line-attr.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "--json" "future-incompat" "-Ccodegen-units=1" "-Zui-testing" "-Zsimulate-remapped-rust-src-base=/rustc/FAKE_PREFIX" "-Ztranslate-remapped-path-to-local-path=no" "-Zdeduplicate-diagnostics=no" "-Cstrip=debuginfo" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-comment-multi-line-attr" "-Cdebuginfo=0" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/rustdoc-ui/doc-comment-multi-line-attr/auxiliary" "--test"
running 1 test
test /checkout/tests/rustdoc-ui/doc-comment-multi-line-attr.rs - (line 7) ... FAILED

failures:
failures:

---- /checkout/tests/rustdoc-ui/doc-comment-multi-line-attr.rs - (line 7) stdout ----
error: mismatched closing delimiter: `)`
  --> /checkout/tests/rustdoc-ui/doc-comment-multi-line-attr.rs:8:119
   |
LL | #![deny(
   |        - closing delimiter possibly meant for this
LL | fn main() { #[allow(non_snake_case)] fn _doctest_main__checkout_tests_rustdoc_ui_doc_comment_multi_line_attr_rs_7_0() {
LL | unused_parens,
LL | )]
   | ^ mismatched closing delimiter


error: unexpected closing delimiter: `}`
  --> /checkout/tests/rustdoc-ui/doc-comment-multi-line-attr.rs:11:1
   |
LL | )]
   | - missing open `(` for this delimiter
LL | } _doctest_main__checkout_tests_rustdoc_ui_doc_comment_multi_line_attr_rs_7_0() }

error: aborting due to 2 previous errors

Couldn't compile the test.
