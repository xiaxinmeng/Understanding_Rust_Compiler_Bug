plain
doc tests for: /checkout/src/doc/unstable-book/src/language-features/lang-items.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/language-features/lint-reasons.md" "--test-args" ""

stdout ----

running 4 tests
running 4 tests
test /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 23) - compile fail ... ok
test /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 53) ... FAILED
test /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Expecting_lint_emissions (line 76) ... ok
test /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Expecting_lint_emissions (line 87) ... ok
failures:


---- /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 53) stdout ----
error[E0658]: lint reasons are experimental
  |
  |
4 |     #[allow(unused_mut, reason = "the path will be modified on windows, but not on other platforms")]
  |
  = note: see issue #54503 <https://github.com/rust-lang/rust/issues/54503> for more information
  = help: add `#![feature(lint_reasons)]` to the crate attributes to enable

---
For more information about an error, try `rustc --explain E0412`.
Couldn't compile the test.

failures:
    /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 53)
test result: FAILED. 3 passed; 1 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.19s


stderr ----
