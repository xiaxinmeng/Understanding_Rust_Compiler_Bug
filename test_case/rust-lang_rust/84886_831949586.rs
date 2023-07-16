plain
test comment::test::format_doc_comments ... ok
test config::test::test_dump_default_config ... ok
test config::test::test_print_docs_exclude_unstable ... ok
test config::test::test_print_docs_include_unstable ... ok
test config::test::use_small_heuristics::test_chain_width_config_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_attr_fn_like_width_config_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_fn_call_width_config_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_off_sets_correct_widths ... ok
test config::test::use_small_heuristics::test_override_attr_fn_like_width_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_override_array_width_exceeds_max_width ... ok
test config::test::test_empty_string_license_template_path ... ok
test config::test::use_small_heuristics::test_override_chain_width_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_override_single_line_if_else_max_width_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_override_struct_lit_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_max_sets_correct_widths ... ok
test config::test::use_small_heuristics::test_override_struct_variant_width_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_default_sets_correct_widths ... ok
test config::test::use_small_heuristics::test_override_with_off ... ok
test config::test::use_small_heuristics::test_override_with_off ... ok
test config::test::use_small_heuristics::test_override_fn_call_width_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_override_with_max ... ok
test config::test::use_small_heuristics::test_override_works_with_default ... ok
test config::test::use_small_heuristics::test_single_line_if_else_max_width_config_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_struct_lit_config_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_struct_variant_width_config_exceeds_max_width ... ok
test config::test::use_small_heuristics::test_array_width_config_exceeds_max_width ... ok
test emitter::checkstyle::tests::emits_single_xml_tree_containing_all_files ... ok
test config::test::test_override_existing_license_with_no_license ... ok
test emitter::checkstyle::xml::tests::other_characters_are_not_escaped ... ok
test emitter::checkstyle::xml::tests::special_characters_are_escaped ... ok
---
  |
4 |             #![cfg(unix)]
  |             ^^^^^^^^^^^^^
  |
  = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted in this context
 --> tests/target/issue-3592.rs:8:13
  |
  |
8 |             #![cfg(not(unix))]
  |
  |
  = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
error: an inner attribute is not permitted in this context
   --> tests/source/match.rs:413:9
    |
    |
413 |         #![allow(simple_match)]
    |
    |
    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
test test::self_tests ... ok
error: an inner attribute is not permitted in this context
   --> tests/target/match.rs:444:9
    |
    |
444 |         #![allow(simple_match)]
    |
    |
    = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files. Outer attributes, like `#[test]`, annotate the item following them.
test test::system_tests ... FAILED
test test::idempotence_tests ... FAILED

failures:
failures:

---- test::system_tests stdout ----
Warning: the `merge_imports` option is deprecated. Use `imports_granularity=Crate` instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `1`,
 right: `0`: 1 system tests failed', src/tools/rustfmt/src/test/mod.rs:186:9
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'test::system_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:73:10
---- test::idempotence_tests stdout ----
---- test::idempotence_tests stdout ----
Warning: the `merge_imports` option is deprecated. Use `imports_granularity=Crate` instead
thread '<unnamed>' panicked at 'assertion failed: `(left == right)`
  left: `2`,
  left: `2`,
 right: `0`: 2 idempotent tests failed', src/tools/rustfmt/src/test/mod.rs:323:9
thread 'test::idempotence_tests' panicked at 'Failed to join a test thread: Any { .. }', src/tools/rustfmt/src/test/mod.rs:73:10

failures:
    test::idempotence_tests
    test::system_tests
---
This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
Verifying status of rustfmt...
This PR updated 'src/tools/rustfmt', verifying if status is 'test-pass'...

We detected that this PR updated 'rustfmt', but its tests failed.

If you do intend to update 'rustfmt', please check the error messages above and
commit another update.

If you do NOT intend to update 'rustfmt', please ensure you did not accidentally
change the submodule at 'src/tools/rustfmt'. You may ask your reviewer for the
proper steps.
{"book":"test-pass","rustfmt":"test-fail","rls":"test-pass","cargo-miri":"test-fail","nomicon":"test-pass","rustbook":"test-fail","reference":"test-pass","embedded-book":"test-pass","edition-guide":"test-pass","rust-by-example":"test-pass","miri":"build-fail"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
