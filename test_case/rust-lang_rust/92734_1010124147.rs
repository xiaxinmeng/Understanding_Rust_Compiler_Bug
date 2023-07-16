plain
  IMAGE: x86_64-gnu-tools
##[endgroup]
From https://github.com/rust-lang/rust
 * branch              beta       -> FETCH_HEAD
Searching for toolstate changes between 02822334e9bd22df33c0692cbaade5a5b5449130 and 5ee904e9a8bf30894430e4a94b86504abc9a365c
Submodules were updated
##[group]Run src/ci/scripts/verify-channel.sh
src/ci/scripts/verify-channel.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
---
  CACHE_DOMAIN: ci-caches.rust-lang.org
  CI_ONLY_WHEN_SUBMODULES_CHANGED: 1
  IMAGE: x86_64-gnu-tools
##[endgroup]
fatal: unknown commit 53fd98ca776cb875bc9e5514f56b52eb74f9e7a9
All commits in `HEAD` are present in `master`
src/ci/scripts/run-build-from-ci.sh
shell: /bin/bash --noprofile --norc -e -o pipefail {0}
env:
  CI_JOB_NAME: x86_64-gnu-tools
---
test client_all_features ... ok
test client_cargo_target_directory_is_excluded_from_backups ... ok
test client_borrow_error ... ok
Warning: can't set `version = Two`, unstable features are only available in nightly channel.
Warning: can't set `ignore = IgnoreList { path_set: {"src/doc/book", "library/backtrace", "compiler/rustc_codegen_gcc", "src/doc/reference", "/build-*/", "src/llvm-project", "src/doc/embedded-book", "compiler/rustc_codegen_cranelift/y.rs", "src/doc/rustc-dev-guide", "src/doc/rust-by-example", "/vendor/", "src/tools/rust-analyzer", "compiler/rustc_codegen_cranelift/example", "library/stdarch", "compiler/rustc_codegen_cranelift/scripts", "src/doc/nomicon", "src/tools/rust-installer", "src/tools/rustfmt", "src/test", "src/tools/clippy", "library/portable-simd", "src/tools/cargo", "src/doc/edition-guide", "src/tools/miri", "src/tools/rls", "/build/", "/*-build/"}, rustfmt_toml_path: "" }`, unstable features are only available in nightly channel.
test client_format_utf16_range ... ok
test client_find_all_refs_no_cfg_test ... ok
test client_goto_def ... ok
test client_find_impls ... ok
---
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-4928"), String("rls.deglobImports-4928")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"message": String("no matching package found\nsearched package name: `auto-cfg`\nperhaps you meant:      auto_cfg\nlocation searched: registry `crates-io`\nrequired by package `dependency_typo v0.1.0 (/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t6/dependency_typo)`"), "range": Object({"end": Object({"character": Number(0), "line": Number(9999)}), "start": Object({"character": Number(0), "line": Number(0)})}), "severity": Number(1)})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t6/dependency_typo/Cargo.toml")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
thread 'client_dependency_typo_and_fix' panicked at 'assertion failed: diag.diagnostics[0].message.contains(\"no matching package named `auto-cfg`\")', src/tools/rls/tests/client.rs:682:5
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})

---
Verifying status of edition-guide...
Verifying status of rls...
This PR updated 'src/tools/rls', verifying if status is 'test-pass'...

We detected that this PR updated 'rls', but its tests failed.

If you do intend to update 'rls', please check the error messages above and
commit another update.

If you do NOT intend to update 'rls', please ensure you did not accidentally
change the submodule at 'src/tools/rls'. You may ask your reviewer for the
proper steps.
