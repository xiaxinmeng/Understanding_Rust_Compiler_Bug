plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              master     -> FETCH_HEAD
Searching for toolstate changes between f206533fd40378da6e2a07567e8d7592edd13ee4 and 73ce9747f4de3616720e13e18f81b07966626ecf
Tool subtrees were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /usr/bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
test test::parser::crate_parsing_errors_on_unclosed_delims ... ok
test test::parser::parser_errors_in_submods_are_surfaced ... ok
test test::configuration_snippet::configuration_snippet_tests ... ok

Mismatch at src/closures.rs:252:
         ast::ClosureBinder::NotPresent => "".to_owned(),
 
 
-    let const_ = if matches!(constness, ast::Const::Yes(_)) { "const " } else { "" };
+    let const_ = if matches!(constness, ast::Const::Yes(_)) {
+    } else {
+        ""
+    };
 
 
     let immovable = if movability == ast::Movability::Static {
         "static "
Mismatch at src/closures.rs:306:
Mismatch at src/closures.rs:306:
         .tactic(tactic)
         .preserve_newline(true);
     let list_str = write_list(&item_vec, &fmt)?;
-    let mut prefix = format!("{}{}{}{}{}|{}|", binder, const_, immovable, is_async, mover, list_str);
+    let mut prefix = format!(
+        "{}{}{}{}{}|{}|",
+        binder, const_, immovable, is_async, mover, list_str
 
 
     if !ret_str.is_empty() {
         if prefix.contains('\n') {
test test::system_tests ... ok
test test::idempotence_tests ... ok

failures:
failures:

---- test::self_tests stdout ----
Ran 5 self tests.
thread 'test::self_tests' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 self tests failed', src/tools/rustfmt/src/test/mod.rs:400:5


failures:
    test::self_tests
