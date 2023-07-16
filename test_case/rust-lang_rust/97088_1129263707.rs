plain
test client_bin_lib_project ... FAILED
test client_fail_uninitialized_request ... FAILED
test client_handle_utf16_unit_text_edits ... FAILED
test client_find_all_refs_no_cfg_test ... FAILED
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
test client_find_impls ... FAILED
test client_format_utf16_range ... FAILED
test client_goto_def ... FAILED
test client_features ... FAILED
test client_features ... FAILED
test client_highlight ... FAILED
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
test client_init_duplicated_and_unknown_settings ... FAILED
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
test client_infer_lib ... FAILED
test client_infer_lib ... FAILED
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test client_implicit_workspace_pick_up_lib_changes ... FAILED
test client_implicit_workspace_pick_up_lib_changes ... FAILED
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
test client_init_with_configuration_kebab_case ... FAILED
test client_init_with_configuration_kebab_case ... FAILED
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
test client_init_with_configuration_mixed_case ... FAILED
test client_init_with_configuration_mixed_case ... FAILED
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
test client_invalid_member_dependency_resolution ... FAILED
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
test client_lens_run ... FAILED
test client_multiple_binaries ... FAILED
test client_no_default_features ... FAILED
test client_invalid_toml_manifest ... FAILED
test client_invalid_toml_manifest ... FAILED
test client_omit_init_build ... FAILED
test client_invalid_member_toml_manifest ... FAILED
test client_reformat ... FAILED
test client_shutdown ... FAILED
test client_test_infer_custom_bin ... FAILED
test client_test_infer_bin ... FAILED
test client_reformat_with_range ... FAILED
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
test client_parse_error_on_malformed_input ... ok
test client_test_complete_self_crate_name ... FAILED
test client_test_simple_workspace ... FAILED
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
test client_workspace_symbol_duplicates ... FAILED
test client_workspace_symbol ... FAILED
test client_use_statement_completion_doesnt_suggest_arguments ... FAILED
test client_use_statement_completion_doesnt_suggest_arguments ... FAILED

failures:

---- client_cargo_target_directory_is_excluded_from_backups stdout ----
thread 'client_cargo_target_directory_is_excluded_from_backups' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39

---- client_dependency_typo_and_fix stdout ----
---- client_dependency_typo_and_fix stdout ----
thread 'client_dependency_typo_and_fix' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_did_change_configuration_duplicated_and_unknown_settings stdout ----
---- client_did_change_configuration_duplicated_and_unknown_settings stdout ----
thread 'client_did_change_configuration_duplicated_and_unknown_settings' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_borrow_error stdout ----
---- client_borrow_error stdout ----
thread 'client_borrow_error' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_changing_workspace_lib_retains_diagnostics stdout ----
---- client_changing_workspace_lib_retains_diagnostics stdout ----
thread 'client_changing_workspace_lib_retains_diagnostics' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_all_features stdout ----
---- client_all_features stdout ----
thread 'client_all_features' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_all_targets stdout ----
---- client_all_targets stdout ----
thread 'client_all_targets' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_bin_lib_project stdout ----
---- client_bin_lib_project stdout ----
thread 'client_bin_lib_project' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_fail_uninitialized_request stdout ----
---- client_fail_uninitialized_request stdout ----
thread 'client_fail_uninitialized_request' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_handle_utf16_unit_text_edits stdout ----
---- client_handle_utf16_unit_text_edits stdout ----
thread 'client_handle_utf16_unit_text_edits' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_find_all_refs_no_cfg_test stdout ----
---- client_find_all_refs_no_cfg_test stdout ----
thread 'client_find_all_refs_no_cfg_test' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_find_impls stdout ----
---- client_find_impls stdout ----
thread 'client_find_impls' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_format_utf16_range stdout ----
---- client_format_utf16_range stdout ----
thread 'client_format_utf16_range' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_goto_def stdout ----
---- client_goto_def stdout ----
thread 'client_goto_def' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_features stdout ----
---- client_features stdout ----
thread 'client_features' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_highlight stdout ----
---- client_highlight stdout ----
thread 'client_highlight' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_hover stdout ----
---- client_hover stdout ----
thread 'client_hover' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_init_duplicated_and_unknown_settings stdout ----
---- client_init_duplicated_and_unknown_settings stdout ----
thread 'client_init_duplicated_and_unknown_settings' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_ignore_uninitialized_notification stdout ----
---- client_ignore_uninitialized_notification stdout ----
thread 'client_ignore_uninitialized_notification' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_infer_lib stdout ----
---- client_infer_lib stdout ----
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'mainthread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'client_infer_lib' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_init_with_configuration_camel_case stdout ----
---- client_init_with_configuration_camel_case stdout ----
thread 'client_init_with_configuration_camel_case' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_implicit_workspace_pick_up_lib_changes stdout ----
---- client_implicit_workspace_pick_up_lib_changes stdout ----
thread 'client_implicit_workspace_pick_up_lib_changes' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_init_with_configuration_kebab_case stdout ----
---- client_init_with_configuration_kebab_case stdout ----
thread 'client_init_with_configuration_kebab_case' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_init_with_configuration_mixed_case stdout ----
---- client_init_with_configuration_mixed_case stdout ----
thread 'client_init_with_configuration_mixed_case' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_invalid_member_dependency_resolution stdout ----
---- client_invalid_member_dependency_resolution stdout ----
thread 'client_invalid_member_dependency_resolution' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_init_with_configuration_snake_case stdout ----
---- client_init_with_configuration_snake_case stdout ----
thread 'client_init_with_configuration_snake_case' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_lens_run stdout ----
---- client_lens_run stdout ----
thread 'client_lens_run' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_multiple_binaries stdout ----
---- client_multiple_binaries stdout ----
thread 'client_multiple_binaries' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_no_default_features stdout ----
---- client_no_default_features stdout ----
thread 'client_no_default_features' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_invalid_toml_manifest stdout ----
---- client_invalid_toml_manifest stdout ----
thread 'client_invalid_toml_manifest' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_omit_init_build stdout ----
---- client_omit_init_build stdout ----
thread 'client_omit_init_build' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_invalid_member_toml_manifest stdout ----
---- client_invalid_member_toml_manifest stdout ----
thread 'client_invalid_member_toml_manifest' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_reformat stdout ----
---- client_reformat stdout ----
thread 'client_reformat' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_shutdown stdout ----
---- client_shutdown stdout ----
thread 'client_shutdown' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_test_infer_custom_bin stdout ----
---- client_test_infer_custom_bin stdout ----
thread 'client_test_infer_custom_bin' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_test_infer_bin stdout ----
---- client_test_infer_bin stdout ----
thread 'client_test_infer_bin' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_reformat_with_range stdout ----
---- client_reformat_with_range stdout ----
thread 'client_reformat_with_range' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_rename stdout ----
---- client_rename stdout ----
thread 'client_rename' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_test_complete_self_crate_name stdout ----
---- client_test_complete_self_crate_name stdout ----
thread 'client_test_complete_self_crate_name' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_test_simple_workspace stdout ----
---- client_test_simple_workspace stdout ----
thread 'client_test_simple_workspace' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_test_infer_lib stdout ----
---- client_test_infer_lib stdout ----
thread 'client_test_infer_lib' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_workspace_symbol_duplicates stdout ----
---- client_workspace_symbol_duplicates stdout ----
thread 'client_workspace_symbol_duplicates' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_workspace_symbol stdout ----
---- client_workspace_symbol stdout ----
thread 'client_workspace_symbol' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39
---- client_use_statement_completion_doesnt_suggest_arguments stdout ----
---- client_use_statement_completion_doesnt_suggest_arguments stdout ----
thread 'client_use_statement_completion_doesnt_suggest_arguments' panicked at 'called `Option::unwrap()` on a `None` value', src\tools\rls\tests\support\client\child_process.rs:51:39

failures:
    client_all_features
    client_all_targets
---
    client_workspace_symbol_duplicates

test result: FAILED. 1 passed; 44 failed; 6 ignored; 0 measured; 0 filtered out; finished in 0.14s

[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at '[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
error: test failed, to rerun pass '--test client'
error: test failed, to rerun pass '--test client'
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2022-05-17T19:26:22Z ERROR rls::server] Can't read message
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 232, kind: BrokenPipe, message: "The pipe is being closed." }', src\tools\rls\rls/src\server\io.rs:190:38
[TIMING] test::Rls { stage: 2, host: x86_64-pc-windows-msvc } -- 103.636
Building stage2 tool miri (x86_64-pc-windows-msvc)
warning: profiles for the non root package will be ignored, specify profiles at the workspace root:
package:   D:\a\rust\rust\src\tools\rls\racer\Cargo.toml
---
Verifying status of edition-guide...
Verifying status of rls...
This PR updated 'src/tools/rls', verifying if status is 'test-pass'...

We detected that this PR updated 'rls', but its tests failed.

If you do intend to update 'rls', please check the error messages above and
commit another update.

If you do NOT intend to update 'rls', please ensure you did not accidentally
change the submodule at 'src/tools/rls'. You may ask your reviewer for the
Build completed unsuccessfully in 0:00:01
