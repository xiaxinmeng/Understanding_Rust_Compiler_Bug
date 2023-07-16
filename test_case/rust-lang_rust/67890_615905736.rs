
src/librustc_feature/active.rs:561: trailing whitespace
src/librustc_trait_selection/traits/error_reporting/mod.rs:561: trailing whitespace
tidy error: Found 1 features without a gate test.
Expected a gate test for the feature 'lazy_normalization_consts'.
Hint: create a failing test file named 'feature-gate-lazy_normalization_consts.rs'
in the 'ui' test suite, with its failures due to
missing usage of `#![feature(lazy_normalization_consts)]`.
Hint: If you already have such a test and don't want to rename it,
you can also add a // gate-test-lazy_normalization_consts line to the test file.
