plain
  SCCACHE_BUCKET: rust-lang-ci-sccache2
  TOOLSTATE_REPO: https://github.com/rust-lang-nursery/rust-toolstate
  CACHE_DOMAIN: ci-caches.rust-lang.org
  EXTRA_VARIABLES: {
 "CI_ONLY_WHEN_SUBMODULES_CHANGED": 1
##[endgroup]
adding extra environment variable CI_ONLY_WHEN_SUBMODULES_CHANGED
linux builder detected, using docker to run the build
##[group]Run src/ci/scripts/should-skip-this.sh
---
Successfully built db676c5d9ab4
Successfully tagged rust-ci:latest
Built container sha256:db676c5d9ab4d58305c6a920bb3ebfa375163edb41273e312ad2e1e97584dc32
Uploading finished image to https://ci-caches.rust-lang.org/docker/cca5ab1355739cee305bf7bb1727157114f95df8dcc3157d6ce2e2e4d892b23b9437e67d3e97a82778d1aea1b4d7c4fc751dc7a6b95357530370b5f4a2cd49e3
upload failed: - to s3://rust-lang-ci-sccache2/docker/cca5ab1355739cee305bf7bb1727157114f95df8dcc3157d6ce2e2e4d892b23b9437e67d3e97a82778d1aea1b4d7c4fc751dc7a6b95357530370b5f4a2cd49e3 Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
     Running unittests (build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/rls-8f364f608825ff10)

running 69 tests

---message---
the operation is ineffective. Consider reducing it to `1`

note: #[warn(identity_op)] implied by #[warn(clippy)]
help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/v0.0.186/index.html#identity_op
[
test actions::diagnostics::diagnostic_message_test::message_cannot_find_type ... ok
    (
test actions::diagnostics::diagnostic_message_test::macro_expected_token_nested_expansion ... ok
---
test server::io::tests::read_message_fails_when_content_is_not_valid_utf8 ... ok
                    character: 42,
                },
            },
            severity: Some(
            ),
            code: Some(
                String(
                    "const_static_lifetime",
                    "const_static_lifetime",
                ),
            ),
            source: Some(
                "clippy",
            ),
            message: "Constants have by default a `'static` lifetime\n\nnote: #[warn(const_static_lifetime)] implied by #[warn(clippy)]\nhelp: for further information visit https://rust-lang-nursery.github.io/rust-clippy/v0.0.187/index.html#const_static_lifetime",
            related_information: Some(
                [],
        },
        [
            Suggestion {
                range: Range {
---
                        line: 354,
                        character: 46,
                    },
                },
                new_text: "&str",
                label: "Line 355: Change to `&str`",
        ],
    ),
]
[
---
                    line: 157,
                    character: 38,
                },
            },
            severity: Some(
            ),
            code: Some(
                String(
                    "E0412",
                    "E0412",
                ),
            ),
            source: Some(
                "rustc",
            ),
            message: "cannot find type `HashSet` in this scope\n\nnot found in this scope",
            related_information: Some(
                    DiagnosticRelatedInformation {
                        location: Location {
                        location: Location {
                            uri: "file:///checkout/src/tools/rls/src/actions/post_build.rs",
                            range: Range {
                                start: Position {
                                    line: 157,
                                    character: 31,
                                end: Position {
                                    line: 157,
                                    character: 38,
                                },
---
                        line: 14,
                        character: 0,
                    },
                },
                new_text: "use std::collections::HashSet;\n",
                label: "Line 15: Add `use std::collections::HashSet;\n`",
            Suggestion {
                range: Range {
                    start: Position {
                        line: 14,
                        line: 14,
                        character: 0,
                    },
                    end: Position {
                        line: 14,
                        character: 0,
                    },
                },
                new_text: "use std::collections::hash_set::HashSet;\n",
                label: "Line 15: Add `use std::collections::hash_set::HashSet;\n`",
        ],
    ),
]
[
---
                    line: 133,
                    character: 29,
                },
            },
            severity: Some(
            ),
            code: Some(
                String(
                    "E0596",
                    "E0596",
                ),
            ),
            source: Some(
                "rustc",
            ),
            message: "cannot borrow immutable local variable `string` as mutable\n\ncannot borrow mutably",
            related_information: Some(
                    DiagnosticRelatedInformation {
                        location: Location {
                        location: Location {
                            uri: "file:///checkout/src/tools/rls/src/lib.rs",
                            range: Range {
                                start: Position {
                                    line: 132,
                                    character: 12,
                                end: Position {
                                    line: 132,
                                    character: 18,
                                },
                                },
                            },
                        },
                        message: "consider changing this to `mut string`",
                    DiagnosticRelatedInformation {
                        location: Location {
                        location: Location {
                            uri: "file:///checkout/src/tools/rls/src/lib.rs",
                            range: Range {
                                start: Position {
                                    line: 133,
                                    character: 23,
                                end: Position {
                                    line: 133,
                                    character: 29,
                                },
                                },
                            },
                        },
                        message: "cannot borrow mutably",
                ],
            ),
        },
        [
---
                        line: 132,
                        character: 18,
                    },
                },
                new_text: "mut string",
                label: "Line 133: Change to `mut string`",
        ],
    ),
]
[
---
                    line: 2,
                    character: 27,
                },
            },
            severity: Some(
            ),
            code: Some(
                String(
                    "E0599",
                    "E0599",
                ),
            ),
            source: Some(
                "rustc",
            ),
            message: "no method named `write_fmt` found for type `std::string::String` in the current scope\n\nhelp: items from traits can only be used if the trait is in scope",
            related_information: Some(
                [],
        },
        [
            Suggestion {
                range: Range {
---
                        line: 0,
                        character: 0,
                    },
                },
                new_text: "use std::fmt::Write;\n\n",
                label: "Line 1: Add `use std::fmt::Write;\n\n`",
        ],
    ),
]
]
src_paths: SrcPaths(
    [
        "/my/repo/build.rs",
        "/my/repo/src/lib.rs",
)
)
plan: ExternalPlan { units: {361655349160679368: Invocation { deps: [], outputs: [], links: {}, command: ProcessBuilder { program: "rustc", args: ["--crate-name", "build_script_build", "/my/repo/build.rs"], env: {}, cwd: None, jobserver: None, display_env_vars: false }, src_path: Some("/my/repo/build.rs") }, 4824325129323586461: Invocation { deps: [0], outputs: [], links: {}, command: ProcessBuilder { program: "rustc", args: ["--crate-name", "repo", "/my/repo/src/lib.rs"], env: {}, cwd: None, jobserver: None, display_env_vars: false }, src_path: Some("/my/repo/src/lib.rs") }}, deps: {4824325129323586461: {361655349160679368}}, rev_deps: {361655349160679368: {4824325129323586461}} }
src_paths: SrcPaths(
    [
        "/my/repo/build.rs",
        "/my/repo/src/lib.rs",
)
)
plan: ExternalPlan { units: {361655349160679368: Invocation { deps: [], outputs: [], links: {}, command: ProcessBuilder { program: "rustc", args: ["--crate-name", "build_script_build", "/my/repo/build.rs"], env: {}, cwd: None, jobserver: None, display_env_vars: false }, src_path: Some("/my/repo/build.rs") }, 4824325129323586461: Invocation { deps: [0], outputs: [], links: {}, command: ProcessBuilder { program: "rustc", args: ["--crate-name", "repo", "/my/repo/src/lib.rs"], env: {}, cwd: None, jobserver: None, display_env_vars: false }, src_path: Some("/my/repo/src/lib.rs") }}, deps: {4824325129323586461: {361655349160679368}}, rev_deps: {361655349160679368: {4824325129323586461}} }
src_paths: SrcPaths(
    [
        "/my/repo/src/lib.rs",
        "/my/repo/build.rs",
)
)
plan: ExternalPlan { units: {4824325129323586461: Invocation { deps: [0], outputs: [], links: {}, command: ProcessBuilder { program: "rustc", args: ["--crate-name", "repo", "/my/repo/src/lib.rs"], env: {}, cwd: None, jobserver: None, display_env_vars: false }, src_path: Some("/my/repo/src/lib.rs") }, 361655349160679368: Invocation { deps: [], outputs: [], links: {}, command: ProcessBuilder { program: "rustc", args: ["--crate-name", "build_script_build", "/my/repo/build.rs"], env: {}, cwd: None, jobserver: None, display_env_vars: false }, src_path: Some("/my/repo/build.rs") }}, deps: {4824325129323586461: {361655349160679368}}, rev_deps: {361655349160679368: {4824325129323586461}} }

test result: ok. 69 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s

     Running tests/client.rs (build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/client-c22ae4f720a9c514)
     Running tests/client.rs (build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/client-c22ae4f720a9c514)
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t6/dependency_typo"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t3/backup_exclusion_workspace"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"all_features": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/features"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"all_targets": Bool(true), "cfg_test": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t1/bin_lib"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/borrow_error"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t7/simple_workspace"), "rootUri": Null})})
running 51 tests
test client_completion ... ignored
test client_completion_suggests_arguments_in_statements ... ignored
test client_deglob ... ignored
test client_deglob ... ignored
test client_find_all_refs_test ... ignored
test client_find_definitions ... ignored
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16776"), String("rls.deglobImports-16776")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16773"), String("rls.deglobImports-16773")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16775"), String("rls.deglobImports-16775")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16777"), String("rls.deglobImports-16777")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16779"), String("rls.deglobImports-16779")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16778"), String("rls.deglobImports-16778")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Sending: Object({"jsonrpc": String("2.0"), "method": String("workspace/didChangeConfiguration"), "params": Object({"settings": Object({"rust": Object({"DupLicated": String("DupLicated"), "all_targets": Bool(false), "dup-licated": String("dup-licated"), "dup_licated": String("dup_lacated"), "dup_val": Bool(false), "features": Array([String("some_feature")]), "unknown1": Number(1), "unknown2": Bool(false)})})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("Unknown RLS configuration: `dup_licated` ,`dup_val` ,`unknown1` ,`unknown2` "), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("Duplicated RLS configuration: dup_licated: DupLicated, ,dup-licated, ,dup_licated, ; "), "type": Number(2)})})
Processing message: Object({"id": Number(1), "jsonrpc": String("2.0"), "method": String("client/unregisterCapability"), "params": Object({"unregisterations": Array([Object({"id": String("rls-range-formatting"), "method": String("textDocument/rangeFormatting")})])})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t14/client_handle_utf16_unit_text_edits"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t5/simple_workspace"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"build_bin": String("bin_lib"), "cfg_test": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t2/bin_lib"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t12/common"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"all_targets": Bool(false)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t9/find_all_refs_no_cfg_test"), "rootUri": Null})})
Sending: Object({"id": Number(1337), "jsonrpc": String("2.0"), "method": String("textDocument/definition"), "params": Object({"position": Object({"character": Number(0), "line": Number(0)}), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t10/common/src/main.rs")})})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t13/client_format_utf16_range"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t11/find_impls"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"features": Array([String("bar"), String("baz")])})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t8/features"), "rootUri": Null})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"all_targets": Bool(false)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t15/common"), "rootUri": Null})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16816"), String("rls.deglobImports-16816")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16819"), String("rls.deglobImports-16819")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16817"), String("rls.deglobImports-16817")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16818"), String("rls.deglobImports-16818")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16822"), String("rls.deglobImports-16822")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16821"), String("rls.deglobImports-16821")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16820"), String("rls.deglobImports-16820")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"error": Object({"code": Number(-32002), "message": String("not yet received `initialize` request")}), "id": Number(1337), "jsonrpc": String("2.0")})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16825"), String("rls.deglobImports-16825")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16824"), String("rls.deglobImports-16824")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("borrow_error"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("features"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("features cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("borrow_error cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("find_all_refs_no_cfg_test"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `z`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_z`"), "range": Object({"end": Object({"character": Number(9), "line": Number(3)}), "start": Object({"character": Number(8), "line": Number(3)})}), "severity": Number(2), "source": String("rustc")}), Object({"code": String("E0499"), "message": String("cannot borrow `x` as mutable more than once at a time\n\nfirst mutable borrow occurs here"), "range": Object({"end": Object({"character": Number(18), "line": Number(2)}), "start": Object({"character": Number(12), "line": Number(2)})}), "severity": Number(3), "source": String("rustc")}), Object({"code": String("E0499"), "message": String("cannot borrow `x` as mutable more than once at a time\n\nsecond mutable borrow occurs here"), "range": Object({"end": Object({"character": Number(18), "line": Number(3)}), "start": Object({"character": Number(12), "line": Number(3)})}), "severity": Number(1), "source": String("rustc")}), Object({"code": String("E0499"), "message": String("cannot borrow `x` as mutable more than once at a time\n\nfirst borrow later used here"), "range": Object({"end": Object({"character": Number(11), "line": Number(4)}), "start": Object({"character": Number(4), "line": Number(4)})}), "severity": Number(3), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t4/borrow_error/src/main.rs")})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Sending: Object({"id": Number(42), "jsonrpc": String("2.0"), "method": String("textDocument/references"), "params": Object({"context": Object({"includeDeclaration": Bool(true)}), "position": Object({"character": Number(7), "line": Number(0)}), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t9/find_all_refs_no_cfg_test/src/main.rs")})})})
Processing message: Object({"id": Number(42), "jsonrpc": String("2.0"), "result": Array([Object({"range": Object({"end": Object({"character": Number(10), "line": Number(0)}), "start": Object({"character": Number(7), "line": Number(0)})}), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t9/find_all_refs_no_cfg_test/src/main.rs")}), Object({"range": Object({"end": Object({"character": Number(18), "line": Number(13)}), "start": Object({"character": Number(15), "line": Number(13)})}), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t9/find_all_refs_no_cfg_test/src/main.rs")})])})
[src/tools/rls/tests/client.rs:1671] range = Range {
    start: Position {
        line: 0,
        character: 7,
    end: Position {
        line: 0,
        character: 10,
    },
    },
}
[src/tools/rls/tests/client.rs:1671] range = Range {
    start: Position {
        line: 13,
        character: 15,
    end: Position {
        line: 13,
        character: 18,
    },
    },
}
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
test client_find_all_refs_no_cfg_test ... ok
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"all_targets": Bool(false)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t16/common"), "rootUri": Null})})
test client_hover_after_src_line_change ... ignored
test client_hover_after_src_line_change ... ignored
Sending: Object({"jsonrpc": String("2.0"), "method": String("workspace/didChangeConfiguration"), "params": Object({"settings": Object({})})})
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t17/common"), "rootUri": Null})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-17161"), String("rls.deglobImports-17161")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-17163"), String("rls.deglobImports-17163")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
test client_all_features ... ok
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t18/simple_workspace"), "rootUri": Null})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-17184"), String("rls.deglobImports-17184")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"error": Object({"code": Number(-32002), "message": String("not yet received `initialize` request")}), "id": Number(99999), "jsonrpc": String("2.0")})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
test client_fail_uninitialized_request ... ok
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t19/infer_lib"), "rootUri": Null})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-17209"), String("rls.deglobImports-17209")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'client_all_targets' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:265:28
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'client_dependency_typo_and_fix' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:265:28thread '
client_cargo_target_directory_is_excluded_from_backups' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:265:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
thread 'client_did_change_configuration_duplicated_and_unknown_settings' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:265:28
test client_did_change_configuration_duplicated_and_unknown_settings ... FAILED
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"DupLicated": String("DupLicated"), "all_targets": Bool(false), "dup-licated": String("dup-licated"), "dup_licated": String("dup_lacated"), "dup_val": Bool(false), "features": Array([String("some_feature")]), "unknown1": Number(1), "unknown2": Bool(false), "use_crate_blacklist": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t20/simple_workspace"), "rootUri": Null})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("Unknown RLS configuration: `dup_licated` ,`dup_val` ,`unknown1` ,`unknown2` "), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("RLS configuration option `use_crate_blacklist` is deprecated: use `crate_blacklist` instead"), "type": Number(2)})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/showMessage"), "params": Object({"message": String("Duplicated RLS configuration: dup_licated: DupLicated, ,dup-licated, ,dup_licated, ; "), "type": Number(2)})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-17224"), String("rls.deglobImports-17224")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
thread 'client_changing_workspace_lib_retains_diagnostics' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/client.rs:248:59
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
thread 'client_handle_utf16_unit_text_editsthread '' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())client_bin_lib_project', ' panicked at 'src/tools/rls/tests/support/client/mod.rscalled `Result::unwrap()` on an `Err` value: Elapsed(()):', thread '265src/tools/rls/tests/support/client/mod.rs::client_goto_def28265' panicked at '
:called `Result::unwrap()` on an `Err` value: Elapsed(())28', 
src/tools/rls/tests/support/client/mod.rs:265:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
thread 'client_find_impls' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:265:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
thread 'client_featuresthread '' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())client_format_utf16_range', ' panicked at 'src/tools/rls/tests/support/client/mod.rscalled `Result::unwrap()` on an `Err` value: Elapsed(()):', 265src/tools/rls/tests/support/client/mod.rs::28265
:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
thread 'client_highlight' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:265:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
thread 'thread 'client_ignore_uninitialized_notificationclient_hover' panicked at '' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())called `Result::unwrap()` on an `Err` value: Elapsed(())', ', src/tools/rls/tests/support/client/mod.rssrc/tools/rls/tests/support/client/mod.rs::265265::2828

Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
thread 'client_implicit_workspace_pick_up_lib_changes' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/client.rs:354:33
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
thread 'client_infer_lib' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:265:28
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
thread 'client_all_targets' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:265:28
stack backtrace:
   0:     0x7f39b3cdc270 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f39b3d4d70d - core::fmt::write::hf00778d011964c9e
   2:     0x7f39b3cd0635 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f39b3ce0807 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f39b3ce021b - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f39b3ce0f02 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   6:     0x7f39b3ce0b37 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   7:     0x7f39b3cdc70c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   8:     0x7f39b3ce0a99 - rust_begin_unwind
   9:     0x7f39b3ca4e11 - core::panicking::panic_fmt::hacef841ef98e28f3
  10:     0x7f39b3ca5003 - core::result::unwrap_failed::heca1ed5f139fb358
thread 'thread 'client_cargo_target_directory_is_excluded_from_backupsclient_dependency_typo_and_fix' panicked at '' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())called `Result::unwrap()` on an `Err` value: Elapsed(())', ', src/tools/rls/tests/support/client/mod.rssrc/tools/rls/tests/support/client/mod.rs::265265::2828

  11:     0x5639e2d8bce6 - <client::support::client::RlsHandle<T> as core::ops::drop::Drop>::drop::hfea83b6446cd3e19
  12:     0x5639e2d4c3a3 - core::ptr::drop_in_place<client::support::client::RlsHandle<client::support::client::child_process::ChildProcess>>::h7c46f5ccca7d2a8c
  13:     0x5639e2d37763 - core::ops::function::FnOnce::call_once::h4aaf6f17396f9ac2
  14:     0x7f39b400d896 - test::__rust_begin_short_backtrace::h073a2fd87705b6a3
  15:     0x7f39b400c1cc - test::run_test::run_test_inner::{{closure}}::he37efac29601ad23
  16:     0x7f39b3fe0b62 - std::sys_common::backtrace::__rust_begin_short_backtrace::he959c3456d23f62c
  17:     0x7f39b3fe5a28 - core::ops::function::FnOnce::call_once{{vtable.shim}}::h721ef1e3b09cf284
  18:     0x7f39b3cf1447 - std::sys::unix::thread::Thread::new::thread_start::hf1a920914bd7fc85
  19:     0x7f39b38276ba - start_thread
  20:     0x7f39b303e4dd - clone
  21:                0x0 - <unknown>
thread panicked while panicking. aborting.
   0: error: test failed, to rerun pass '--test client'

Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/client-c22ae4f720a9c514 --nocapture` (signal: 4, SIGILL: illegal instruction)

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--" "--nocapture"
expected success, got: exit code: 101

---
  - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--" "--nocapture"

failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --no-fail-fast --test-args=--nocapture src/tools/rls
Build completed unsuccessfully in 0:27:28
    Blocking waiting for file lock on package cache
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
    Finished dev [unoptimized + debuginfo] target(s) in 2.88s
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "PoisonError { inner: .. }"', src/tools/rls/rls/src/build/cargo.rs:427:63
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "PoisonError { inner: .. }"', src/tools/rls/rls/src/build/cargo.rs:427:63
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'failed to send progress update: "SendError(..)"', src/tools/rls/rls/src/build/cargo.rs:434:18
thread '<unnamed>' panicked at 'called `Result::unwrap()` on an `Err` value: "PoisonError { inner: .. }"', src/tools/rls/rls/src/build/cargo.rs:427:63
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'child threads shouldn't panic: Any', src/tools/cargo/src/cargo/core/compiler/job_queue.rs:465:10
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
error: Tool `book` was not recorded in tool state.
error: Tool `nomicon` was not recorded in tool state.
error: Tool `reference` was not recorded in tool state.
error: Tool `rust-by-example` was not recorded in tool state.
error: Tool `edition-guide` was not recorded in tool state.
error: Tool `rustfmt` was not recorded in tool state.
error: Tool `miri` was not recorded in tool state.
error: Tool `embedded-book` was not recorded in tool state.
{"rls":"test-fail"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
