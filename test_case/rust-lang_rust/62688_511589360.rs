plain
2019-07-15T19:44:43.3746609Z do so (now or later) by using -b with the checkout command again. Example:
2019-07-15T19:44:43.3746685Z 
2019-07-15T19:44:43.3746957Z   git checkout -b <new-branch-name>
2019-07-15T19:44:43.3747009Z 
2019-07-15T19:44:43.3747382Z HEAD is now at ff5921fa6 Auto merge of #62688 - topecongiro:rustfmt-1.3.3, r=alexcrichton
2019-07-15T19:44:43.3883280Z ##[section]Starting: Collect CPU-usage statistics in the background
2019-07-15T19:44:43.3886891Z ==============================================================================
2019-07-15T19:44:43.3887162Z Task         : Bash
2019-07-15T19:44:43.3887257Z Description  : Run a Bash script on macOS, Linux, or Windows
---
2019-07-15T22:05:43.5674125Z test config::test::test_dump_default_config ... ok
2019-07-15T22:05:43.5674225Z test config::test::test_config_used_to_toml ... ok
2019-07-15T22:05:43.5674346Z test config::test::test_print_docs_exclude_unstable ... ok
2019-07-15T22:05:43.5674435Z test config::test::test_print_docs_include_unstable ... ok
2019-07-15T22:05:43.5674546Z test emitter::checkstyle::xml::tests::other_characters_are_not_escaped ... ok
2019-07-15T22:05:43.5674633Z test config::test::test_was_set ... ok
2019-07-15T22:05:43.5674749Z test emitter::checkstyle::xml::tests::special_characters_are_escaped_in_string_with_other_characters ... ok
2019-07-15T22:05:43.5674898Z test emitter::checkstyle::xml::tests::special_characters_are_escaped ... ok
2019-07-15T22:05:43.5683066Z test formatting::newline_style::tests::applies_unix_newlines_to_string_with_unix_and_windows_newlines ... ok
2019-07-15T22:05:43.5684363Z test formatting::newline_style::tests::applies_unix_newlines ... ok
2019-07-15T22:05:43.5684549Z test formatting::newline_style::tests::applying_unix_newlines_changes_nothing_for_unix_newlines ... ok
2019-07-15T22:05:43.5684702Z test formatting::newline_style::tests::applies_windows_newlines_to_string_with_unix_and_windows_newlines ... ok
---
2019-07-15T22:05:45.0661177Z test print_config ... FAILED
2019-07-15T22:05:45.0661246Z 
2019-07-15T22:05:45.0661341Z failures:
2019-07-15T22:05:45.0661378Z 
2019-07-15T22:05:45.0661893Z ---- print_config stdout ----
2019-07-15T22:05:45.0662191Z thread 'print_config' panicked at 'stdout:
2019-07-15T22:05:45.0662318Z stderr:
2019-07-15T22:05:45.0662318Z stderr:
2019-07-15T22:05:45.0662577Z Read-only file system (os error 30)
2019-07-15T22:05:45.0662897Z ', src/tools/rustfmt/tests/rustfmt/main.rs:70:5
2019-07-15T22:05:45.0663059Z 
2019-07-15T22:05:45.0663103Z 
2019-07-15T22:05:45.0663161Z failures:
2019-07-15T22:05:45.0663236Z     print_config
---
2019-07-15T22:09:44.5067276Z Verifying status of rls...
2019-07-15T22:09:44.5079385Z Verifying status of rustfmt...
2019-07-15T22:09:44.5092586Z This PR updated 'src/tools/rustfmt', verifying if status is 'test-pass'...
2019-07-15T22:09:44.5103197Z 
2019-07-15T22:09:44.5105992Z ⚠️ We detected that this PR updated 'rustfmt', but its tests failed.
2019-07-15T22:09:44.5106106Z 
2019-07-15T22:09:44.5106466Z If you do intend to update 'rustfmt', please check the error messages above and
2019-07-15T22:09:44.5106572Z commit another update.
2019-07-15T22:09:44.5106631Z 
2019-07-15T22:09:44.5106927Z If you do NOT intend to update 'rustfmt', please ensure you did not accidentally
2019-07-15T22:09:44.5107254Z change the submodule at 'src/tools/rustfmt'. You may ask your reviewer for the
2019-07-15T22:09:44.5107334Z proper steps.
2019-07-15T22:09:45.0963579Z ##[error]Bash exited with code '3'.
2019-07-15T22:09:45.0994841Z ##[section]Starting: Upload CPU usage statistics
2019-07-15T22:09:45.1003991Z ==============================================================================
2019-07-15T22:09:45.1004098Z Task         : Bash
2019-07-15T22:09:45.1004168Z Description  : Run a Bash script on macOS, Linux, or Windows
