plain
doc tests for: /checkout/src/doc/unstable-book/src/language-features/lang-items.md
doc tests for: /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md


command did not execute successfully: "/checkout/obj/build/bootstrap/debug/rustdoc" "-Wrustdoc::invalid_codeblock_attributes" "-Dwarnings" "-Znormalize-docs" "-Z" "unstable-options" "--test" "/checkout/src/doc/unstable-book/src/language-features/lint-reasons.md" "--test-args" ""

stdout ----

running 6 tests
running 6 tests
test /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 23) ... FAILED
test /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Expecting_lint_emissions (line 99) ... FAILED
test /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 34) ... FAILED
test /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 53) ... FAILED
test /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Expecting_lint_emissions (line 76) ... ok
test /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Expecting_lint_emissions (line 87) ... ok
failures:


---- /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 23) stdout ----
error: unused variable: `unused`
  |
  |
6 |     let unused = "How much wood would a woodchuck chuck?";
  |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused`
  = note: unused variables, should be removed
note: the lint level is defined here
 --> /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md:27:12
  |
  |
5 |     #[deny(unused_variables, reason = "unused variables, should be removed")]

error: aborting due to previous error

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Expecting_lint_emissions (line 99) stdout ----
error: unknown start of token: `
 --> /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md:106:11
  |
9 |   = note: `#[warn(unfulfilled_lint_expectations)]` on by default
  |           ^
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
9 |   = note: '#[warn(unfulfilled_lint_expectations)]` on by default

error: unknown start of token: `
 --> /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md:106:50
  |
  |
9 |   = note: `#[warn(unfulfilled_lint_expectations)]` on by default
  |                                                  ^
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
9 |   = note: `#[warn(unfulfilled_lint_expectations)]' on by default


error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `lint`
  |
3 | warning: this lint expectation is unfulfilled
  |        -      ^^^^ expected one of 8 possible tokens
  |        |
  |        |
  |        tried to parse a type due to this type ascription
  |
  = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: aborting due to 3 previous errors

Couldn't compile the test.
Couldn't compile the test.
---- /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 34) stdout ----
 --> /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md:35:25
  |
3 | error: unused variable: `unused`
  |                         ^
  |                         ^
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
3 | error: unused variable: 'unused`

error: unknown start of token: `
 --> /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md:35:32
  |
  |
3 | error: unused variable: `unused`
  |                                ^
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
3 | error: unused variable: `unused'
  |                                ~

error: unknown start of token: `
error: unknown start of token: `
 --> /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md:39:80
  |
7 |   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused`
  |
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
7 |   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: '_unused`

error: unknown start of token: `
 --> /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md:39:88
  |
  |
7 |   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused`
  |
  |
help: Unicode character '`' (Grave Accent) looks like ''' (Single Quote), but it is not
  |
7 |   |         ^^^^^^ help: if this is intentional, prefix it with an underscore: `_unused'


error: expected one of `!`, `(`, `.`, `::`, `;`, `<`, `?`, or `}`, found `variable`
  |
3 | error: unused variable: `unused`
  |      -        ^^^^^^^^ expected one of 8 possible tokens
  |      |
  |      |
  |      tried to parse a type due to this type ascription
  |
  = note: `#![feature(type_ascription)]` lets you annotate an expression with a type: `<expr>: <type>`

error: aborting due to 5 previous errors

Couldn't compile the test.
Couldn't compile the test.
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
    /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Expecting_lint_emissions (line 99)
    /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 23)
    /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 34)
    /checkout/src/doc/unstable-book/src/language-features/lint-reasons.md - The_tracking_issue_for_this_feature_is__::Reason_information_on_lint_level_attributes (line 53)
test result: FAILED. 2 passed; 4 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.17s


stderr ----
