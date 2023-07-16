plain
    Checking rustdoc v0.0.0 (/checkout/src/librustdoc)
error: expected identifier, found `(`
  --> src/librustdoc/passes/check_doc_test_visibility.rs:28:10
   |
28 | crate fn (krate: Crate, cx: &mut DocContext<'_>) -> Crate {
   |          ^ expected identifier
error[E0432]: unresolved import `self::check_doc_test_visibility::CHECK_DOC_TEST_VISIBILITY`
  --> src/librustdoc/passes/mod.rs:37:11
   |
   |
37 | crate use self::check_doc_test_visibility::CHECK_DOC_TEST_VISIBILITY;
   |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ no `CHECK_DOC_TEST_VISIBILITY` in `passes::check_doc_test_visibility`

error[E0432]: unresolved imports `crate::passes::check_doc_test_visibility::should_have_doc_example`, `crate::passes::check_doc_test_visibility::Tests`
 --> src/librustdoc/passes/calculate_doc_coverage.rs:5:48
  |
5 | use crate::passes::check_doc_test_visibility::{should_have_doc_example, Tests};
  |                                                ^^^^^^^^^^^^^^^^^^^^^^^  ^^^^^ no `Tests` in `passes::check_doc_test_visibility`
  |                                                |
  |                                                no `should_have_doc_example` in `passes::check_doc_test_visibility`
For more information about this error, try `rustc --explain E0432`.
error: could not compile `rustdoc` due to 3 previous errors
Build completed unsuccessfully in 0:02:52
