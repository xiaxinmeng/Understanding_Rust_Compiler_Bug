plain
running 1575 tests
........................................................................................   88/1575
........................................................................................  176/1575
........................................................................................  264/1575
....................................................F.F.................................  352/1575
........................................................................................  528/1575
........................................................................................  616/1575
..........................................ii............................................  704/1575
........................................................................................  792/1575
---
thread 'iter::adapters::intersperse::test_intersperse_fold' panicked at 'assertion failed: `(left == right)`
  left: `2005`,
 right: `0`', library/core/tests/iter/adapters/intersperse.rs:118:5

---- iter::adapters::intersperse::test_intersperse_size_hint stdout ----
thread 'iter::adapters::intersperse::test_intersperse_size_hint' panicked at 'assertion failed: `(left == right)`
  left: `(5, Some(5))`,
error: test failed, to rerun pass `-p core --test coretests`
 right: `(7, Some(7))`', library/core/tests/iter/adapters/intersperse.rs:28:5

failures:
    iter::adapters::intersperse::test_intersperse_fold
    iter::adapters::intersperse::test_intersperse_size_hint
