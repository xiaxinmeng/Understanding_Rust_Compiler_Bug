`. In practice, this means that if a project does not set `doctest = false` in their manifest, `cargo test --include-ignored` will fail due to non-test documentation examples. The only workarounds short of disabling doctests are to:

- Note which non doctest tests are ignored and request them with `--ignored name_of_test`,
- Use a test runner such as `cargo nextest` which does not support doctests,
- Test each target individually (`cargo test --lib --bin --tests --examples --benches`) and set `--include-ignored` for everything but `--doc`, or
- Fall back to source edits, which is basically required if you want to actually test an ignored doctest.

This PR proposes adding 