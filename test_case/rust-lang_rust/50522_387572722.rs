plain
[01:22:02] test unit_tests::test_format_code_block ... ok
[01:22:02] test unit_tests::test_format_code_block_fail ... ok
[01:22:02] test unit_tests::test_format_snippet ... ok
[01:22:02] test unit_tests::test_no_panic_on_format_snippet_and_format_code_block ... ok
[01:22:02] error[E0583]: file not found for module `bar`
[01:22:02]  --> tests/target/issue-2673-nonmodrs-mods/foo.rs:2:5
[01:22:02]   |
[01:22:02] 2 | mod bar;
[01:22:02]   |
[01:22:02]   |
[01:22:02]   = help: name the file either bar.rs or bar/mod.rs inside the directory "tests/target/issue-2673-nonmodrs-mods"
[01:22:02] test test::system_tests ... ok
[01:22:02] test test::idempotence_tests ... FAILED
[01:22:02] test test::self_tests ... ok
[01:22:02] 
[01:22:02] 
[01:22:02] failures:
[01:22:02] 
[01:22:02] ---- test::idempotence_tests stdout ----
[01:22:02]  Warning: can't set `reorder_imports = true`, unstable features are only available in nightly channel.
[01:22:02] Warning: can't set `skip_children = true`, unstable features are only available in nightly channel.
[01:22:02] Warning: can't set `indent_style = Block`, unstable features are only available in nightly channel.
[01:22:02] Warning: can't set `comment_width = 80`, unstable features are only available in nightly channel.
[01:22:02] Warning: can't set `format_strings = true`, unstable features are only available in nightly channel.
[01:22:02] Warning: can't set `reorder_imports = false`, unstable features are only available in nightly channel.
[01:22:02] Warning: can't set `fn_args_density = Tall`, unstable features are only available in nightly channel.
[01:22:02] Warning: can't set `brace_style = SameLineWhere`, unstable features are only available in nightly channel.
[01:22:02] Warning: can't set `trailing_comma = Vertical`, unstable features are only available in nightly channel.
[01:22:02] Warning: can't set `report_todo = Always`, unstable features are only available in nightly channel.
[01:22:02] Warning: can't set `report_fixme = Never`, unstable features are only available in nightly channel.
[01:22:02] Ran 325 idempotent tests.
[01:22:02] thread 'test::idempotence_tests' panicked at 'assertion failed: `(left == right)`
[01:22:02]   left: `1`,
[01:22:02]  right: `0`: 1 idempotent tests failed', tools/rustfmt/src/test/mod.rs:200:5
[01:22:02] 
[01:22:02] 
[01:22:02] failures:
[01:22:02]     test::idempotence_tests
