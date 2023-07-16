plain
test client_omit_init_build ... ok
test client_handle_utf16_unit_text_edits ... ok
test client_invalid_member_dependency_resolution ... ok
test client_init_duplicated_and_unknown_settings ... FAILED
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any { .. }', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
test client_implicit_workspace_pick_up_lib_changes ... FAILED
test client_init_with_configuration_kebab_case ... FAILED
test client_init_with_configuration_snake_case ... FAILED
test client_infer_lib ... FAILED
---

failures:

---- client_init_duplicated_and_unknown_settings stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"DupLicated": String("DupLicated"), "all_targets": Bool(false), "dup-licated": String("dup-licated"), "dup_licated": String("dup_lacated"), "dup_val": Bool(false), "features": Array([String("some_feature")]), "unknown1": Number(1), "unknown2": Bool(false), "use_crate_blacklist": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t20/simple_workspace"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("Unknown RLS configuration: `dup_licated` ,`dup_val` ,`unknown1` ,`unknown2` "), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("RLS configuration option `use_crate_blacklist` is deprecated: use `crate_blacklist` instead"), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("Duplicated RLS configuration: dup_licated: DupLicated, ,dup-licated, ,dup_licated, ; "), "type": Number(2)})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_init_duplicated_and_unknown_settings' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28


---- client_dependency_typo_and_fix stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/dependency_typo"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"message": String("no matching package found\nsearched package name: `auto-cfg`\nperhaps you meant:      auto_cfg\nlocation searched: registry `crates-io`\nrequired by package `dependency_typo v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/dependency_typo)`"), "range": Object({"end": Object({"character": Number(0), "line": Number(9999)}), "start": Object({"character": Number(0), "line": Number(0)})}), "severity": Number(1)})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/dependency_typo/Cargo.toml")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Sending: Object({"jsonrpc": String("2.0"), "method": String("workspace/didChangeWatchedFiles"), "params": Object({"changes": Array([Object({"type": Number(2), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/dependency_typo/Cargo.toml")})])})})
thread 'client_dependency_typo_and_fix' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_3"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_2"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_2"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"message": String("failed to select a version for the requirement `autocfg = \"^0.5555\"`\ncandidate versions found which didn't match: 1.1.0, 1.0.1, 1.0.0, ...\nlocation searched: crates.io index\nrequired by package `dependency_typo v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/dependency_typo)`"), "range": Object({"end": Object({"character": Number(0), "line": Number(9999)}), "start": Object({"character": Number(0), "line": Number(0)})}), "severity": Number(1)})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/dependency_typo/Cargo.toml")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_implicit_workspace_pick_up_lib_changes stdout ----
---- client_implicit_workspace_pick_up_lib_changes stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t18/simple_workspace"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_implicit_workspace_pick_up_lib_changes' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/client.rs:354:33
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("inner"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("binary cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("binary"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `val`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_val`"), "range": Object({"end": Object({"character": Number(27), "line": Number(4)}), "start": Object({"character": Number(24), "line": Number(4)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t18/simple_workspace/src/main.rs")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_init_with_configuration_kebab_case stdout ----
---- client_init_with_configuration_kebab_case stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"all-targets": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t22/config_cases"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_init_with_configuration_kebab_case' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0412"), "message": String("cannot find type `PathBuf` in this scope\n\nnot found in this scope"), "range": Object({"end": Object({"character": Number(47), "line": Number(4)}), "start": Object({"character": Number(40), "line": Number(4)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t22/config_cases/src/main.rs")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_init_with_configuration_snake_case stdout ----
---- client_init_with_configuration_snake_case stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"all_targets": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t24/config_cases"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_init_with_configuration_snake_case' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0412"), "message": String("cannot find type `PathBuf` in this scope\n\nnot found in this scope"), "range": Object({"end": Object({"character": Number(47), "line": Number(4)}), "start": Object({"character": Number(40), "line": Number(4)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t24/config_cases/src/main.rs")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_infer_lib stdout ----
---- client_infer_lib stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t19/infer_lib"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_infer_lib' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("infer_lib"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("infer_lib"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("infer_lib cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("dead_code"), "message": String("struct `UnusedLib` is never constructed\n\nnote: `#[warn(dead_code)]` on by default"), "range": Object({"end": Object({"character": Number(16), "line": Number(0)}), "start": Object({"character": Number(7), "line": Number(0)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t19/infer_lib/src/lib.rs")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_ignore_uninitialized_notification stdout ----
---- client_ignore_uninitialized_notification stdout ----
Sending: Object({"jsonrpc": String("2.0"), "method": String("workspace/didChangeConfiguration"), "params": Object({"settings": Object({})})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t17/common"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_ignore_uninitialized_notification' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
---
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_init_with_configuration_mixed_case stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"allTargets": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t23/config_cases"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_init_with_configuration_mixed_case' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0412"), "message": String("cannot find type `PathBuf` in this scope\n\nnot found in this scope"), "range": Object({"end": Object({"character": Number(47), "line": Number(4)}), "start": Object({"character": Number(40), "line": Number(4)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t23/config_cases/src/main.rs")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_no_default_features stdout ----
---- client_no_default_features stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"features": Array([String("foo"), String("bar")]), "no_default_features": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t30/features"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_no_default_features' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("features cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("features cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("features"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0422"), "message": String("cannot find struct, variant or union type `Baz` in this scope\n\nsimilarly named struct `Bar` defined here"), "range": Object({"end": Object({"character": Number(15), "line": Number(4)}), "start": Object({"character": Number(0), "line": Number(4)})}), "severity": Number(3), "source": String("rustc")}), Object({"code": String("E0422"), "message": String("cannot find struct, variant or union type `Baz` in this scope\n\nhelp: a struct with a similar name exists: `Bar`"), "range": Object({"end": Object({"character": Number(7), "line": Number(12)}), "start": Object({"character": Number(4), "line": Number(12)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t30/features/src/main.rs")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_lens_run stdout ----
---- client_lens_run stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({}), "initializationOptions": Object({"cmdRun": Bool(true)}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t28/lens_run"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_lens_run' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("run"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("run"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("run cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0433"), "message": String("failed to resolve: use of undeclared crate or module `tokio`\n\nuse of undeclared crate or module `tokio`"), "range": Object({"end": Object({"character": Number(7), "line": Number(8)}), "start": Object({"character": Number(2), "line": Number(8)})}), "severity": Number(1), "source": String("rustc")}), Object({"code": String(""), "message": String("cannot find attribute `dummy_ext` in this scope"), "range": Object({"end": Object({"character": Number(11), "line": Number(13)}), "start": Object({"character": Number(2), "line": Number(13)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t28/lens_run/src/main.rs")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_init_with_configuration_camel_case stdout ----
---- client_init_with_configuration_camel_case stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"AllTargets": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t21/config_cases"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_init_with_configuration_camel_case' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0412"), "message": String("cannot find type `PathBuf` in this scope\n\nnot found in this scope"), "range": Object({"end": Object({"character": Number(47), "line": Number(4)}), "start": Object({"character": Number(40), "line": Number(4)})}), "severity": Number(1), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t21/config_cases/src/main.rs")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_multiple_binaries stdout ----
---- client_multiple_binaries stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"build_bin": String("bin2")})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t29/multiple_bins"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_multiple_binaries' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin2"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin2"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin1"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin2 cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin1 cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `bin_name2`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_bin_name2`"), "range": Object({"end": Object({"character": Number(17), "line": Number(1)}), "start": Object({"character": Number(8), "line": Number(1)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t29/multiple_bins/src/main2.rs")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `bin_name1`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_bin_name1`"), "range": Object({"end": Object({"character": Number(17), "line": Number(1)}), "start": Object({"character": Number(8), "line": Number(1)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t29/multiple_bins/src/main.rs")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_rename stdout ----
---- client_rename stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"all_targets": Bool(false)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t34/common"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_rename' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("completion"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("completion"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_reformat_with_range stdout ----
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t33/reformat_with_range"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_reformat_with_range' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("reformat_with_range cfg(test)"), "percentage": Null, "title": String("Building")})})
---
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---- client_reformat stdout ----
error: test failed, to rerun pass '--test client'
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t32/reformat"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("The RLS is being deprecated in favor of rust-analyzer. Current users of RLS should begin migrating to using rust-analyzer instead."), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_reformat' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:262:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("reformat"), "percentage": Null, "title": String("Building")})})
---
Verifying status of rls...
Verifying status of miri...
Verifying status of embedded-book...
Cloning into 'rust-toolstate'...
error: Tool `rls` should be test-pass but is test-fail
