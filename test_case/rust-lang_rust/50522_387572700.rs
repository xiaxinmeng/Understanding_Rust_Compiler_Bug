plain
[00:03:01] travis_time:end:07a10d7c:start=1525816899060245135,finish=1525816899099536354,duration=39291219
travis_fold:start:make-prepare
travis_time:start:1fa60bd4
Attempting with retry: make prepare
[00:03:01] downloading https://dev-static.rust-lang.org/dist/2018-05-07/rust-std-1.26.0-x86_64-unknown-linux-gnu.tar.gz
                                                                           0.3%
                                                                           1.1%
#                                                                          2.0%
##                                                                         3.6%
---
####################################################                      72.4%
#################################################################         90.7%
######################################################################## 100.0%
[00:03:06] extracting /checkout/obj/build/cache/2018-05-07/rust-std-1.26.0-x86_64-unknown-linux-gnu.tar.gz
[00:03:07] downloading https://dev-static.rust-lang.org/dist/2018-05-07/rustc-1.26.0-x86_64-unknown-linux-gnu.tar.gz
##                                                                         4.0%
##########                                                                14.6%
##################                                                        25.6%
######################                                                    30.7%
---
###################################################################       94.1%
#######################################################################   98.8%
######################################################################## 100.0%
[00:03:14] extracting /checkout/obj/build/cache/2018-05-07/rustc-1.26.0-x86_64-unknown-linux-gnu.tar.gz
[00:03:14] downloading https://dev-static.rust-lang.org/dist/2018-05-07/cargo-0.27.0-x86_64-unknown-linux-gnu.tar.gz
###############                                                           21.9%
######################################################################## 100.0%
[00:03:15] extracting /checkout/obj/build/cache/2018-05-07/cargo-0.27.0-x86_64-unknown-linux-gnu.tar.gz
[00:03:15]     Updating registry `https://github.com/rust-lang/crates.io-index`
---
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
