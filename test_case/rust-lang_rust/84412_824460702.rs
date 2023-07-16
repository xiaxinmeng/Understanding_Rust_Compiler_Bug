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
Successfully built d38be7e22421
Successfully tagged rust-ci:latest
Built container sha256:d38be7e224212002c97900a599b5950b78e69f06495b9babe22ee4dabd006ecf
Uploading finished image to https://ci-caches.rust-lang.org/docker/8c5d44d66b29bbb836b4e0d9503604716245cff6961c60436d51a0b894f3aecf094f7312f2edc796936d29342d7cdcd06e30510e443d59d5e32caf0ecb5387ef
upload failed: - to s3://rust-lang-ci-sccache2/docker/8c5d44d66b29bbb836b4e0d9503604716245cff6961c60436d51a0b894f3aecf094f7312f2edc796936d29342d7cdcd06e30510e443d59d5e32caf0ecb5387ef Unable to locate credentials
[CI_JOB_NAME=x86_64-gnu-tools]
---
test actions::diagnostics::diagnostic_message_test::message_cannot_find_type ... ok
test actions::diagnostics::diagnostic_message_test::message_clippy_identity_op ... 
---message---
[
the operation is ineffective. Consider reducing it to `1`
        Diagnostic {
            range: Range {


note: #[warn(identity_op)] implied by #[warn(clippy)]
help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/v0.0.186/index.html#identity_op
ok
test actions::diagnostics::diagnostic_message_test::message_consider_borrowing ... ok
test actions::diagnostics::diagnostic_message_test::message_mismatched_types ... ok
test actions::diagnostics::diagnostic_message_test::message_move_out_of_borrow ... ok
---
                    line: 354,
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
test actions::diagnostics::diagnostic_message_test::message_type_annotations_needed ... ok
test actions::diagnostics::diagnostic_message_test::message_unused_use ... ok
test actions::diagnostics::diagnostic_message_test::message_use_after_move ... ok
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
test actions::format::tests::calc_text_edits ... ok
---
test actions::hover::test::test_extract_docs_module_docs ... ok
test actions::hover::test::test_extract_docs_module_docs_no_copyright ... ok
test actions::hover::test::test_extract_docs_module_docs_with_attribute ... ok
test actions::hover::test::test_format_method ... ok
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
src_paths: SrcPaths(
    [
        "/my/repo/build.rs",
        "/my/repo/src/lib.rs",
)
)
plan: ExternalPlan { units: {361655349160679368: Invocation { deps: [], outputs: [], links: {}, command: ProcessBuilder { program: "rustc", args: ["--crate-name", "build_script_build", "/my/repo/build.rs"], env: {}, cwd: None, jobserver: None, display_env_vars: false }, src_path: Some("/my/repo/build.rs") }, 4824325129323586461: Invocation { deps: [0], outputs: [], links: {}, command: ProcessBuilder { program: "rustc", args: ["--crate-name", "repo", "/my/repo/src/lib.rs"], env: {}, cwd: None, jobserver: None, display_env_vars: false }, src_path: Some("/my/repo/src/lib.rs") }}, deps: {4824325129323586461: {361655349160679368}}, rev_deps: {361655349160679368: {4824325129323586461}} }
test actions::hover::test::test_format_object ... ok
test actions::hover::test::test_noindent ... ok
test actions::hover::test::test_process_docs_bash_block ... ok
test actions::hover::test::test_process_docs_racer_returns_extra_slashes ... ok
---
test result: ok. 69 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.01s


running 51 tests
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"all_features": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/features"), "rootUri": Null})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16689"), String("rls.deglobImports-16689")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("features cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("features"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
test client_all_features ... ok
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"all_targets": Bool(true), "cfg_test": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/bin_lib"), "rootUri": Null})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16732"), String("rls.deglobImports-16732")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin_lib"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin_lib cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("tests cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin_lib"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin_lib cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `unused_var`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_unused_var`"), "range": Object({"end": Object({"character": Number(18), "line": Number(2)}), "start": Object({"character": Number(8), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/bin_lib/tests/tests.rs")})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
test client_all_targets ... ok
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "initializationOptions": Object({"settings": Object({"rust": Object({"build_bin": String("bin_lib"), "cfg_test": Bool(true)})})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/bin_lib"), "rootUri": Null})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16796"), String("rls.deglobImports-16796")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin_lib"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin_lib cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin_lib"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("tests cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("bin_lib cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `unused_var`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_unused_var`"), "range": Object({"end": Object({"character": Number(18), "line": Number(2)}), "start": Object({"character": Number(8), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/bin_lib/tests/tests.rs")})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
test client_bin_lib_project ... ok
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/borrow_error"), "rootUri": Null})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16860"), String("rls.deglobImports-16860")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("borrow_error cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("borrow_error"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `z`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_z`"), "range": Object({"end": Object({"character": Number(9), "line": Number(3)}), "start": Object({"character": Number(8), "line": Number(3)})}), "severity": Number(2), "source": String("rustc")}), Object({"code": String("E0499"), "message": String("cannot borrow `x` as mutable more than once at a time\n\nfirst mutable borrow occurs here"), "range": Object({"end": Object({"character": Number(18), "line": Number(2)}), "start": Object({"character": Number(12), "line": Number(2)})}), "severity": Number(3), "source": String("rustc")}), Object({"code": String("E0499"), "message": String("cannot borrow `x` as mutable more than once at a time\n\nsecond mutable borrow occurs here"), "range": Object({"end": Object({"character": Number(18), "line": Number(3)}), "start": Object({"character": Number(12), "line": Number(3)})}), "severity": Number(1), "source": String("rustc")}), Object({"code": String("E0499"), "message": String("cannot borrow `x` as mutable more than once at a time\n\nfirst borrow later used here"), "range": Object({"end": Object({"character": Number(11), "line": Number(4)}), "start": Object({"character": Number(4), "line": Number(4)})}), "severity": Number(3), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/borrow_error/src/main.rs")})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
test client_borrow_error ... ok
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/backup_exclusion_workspace"), "rootUri": Null})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16899"), String("rls.deglobImports-16899")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("foo"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
test client_cargo_target_directory_is_excluded_from_backups ... ok
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace"), "rootUri": Null})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-16942"), String("rls.deglobImports-16942")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("library"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("library cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("binary cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": String("binary"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `unused`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_unused`"), "range": Object({"end": Object({"character": Number(30), "line": Number(2)}), "start": Object({"character": Number(24), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")}), Object({"code": String("unused_variables"), "message": String("unused variable: `test_val`\n\nhelp: if this is intentional, prefix it with an underscore: `_test_val`"), "range": Object({"end": Object({"character": Number(36), "line": Number(9)}), "start": Object({"character": Number(28), "line": Number(9)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/library/src/lib.rs")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `val`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_val`"), "range": Object({"end": Object({"character": Number(27), "line": Number(4)}), "start": Object({"character": Number(24), "line": Number(4)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/binary/src/main.rs")})})
Sending: Object({"jsonrpc": String("2.0"), "method": String("textDocument/didChange"), "params": Object({"contentChanges": Array([Object({"range": Object({"end": Object({"character": Number(41), "line": Number(1)}), "start": Object({"character": Number(38), "line": Number(1)})}), "rangeLength": Number(3), "text": String("u64")})]), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/library/src/lib.rs"), "version": Number(0)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": String("library cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": String("library"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": String("binary"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_3"), "message": String("binary cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_3"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_2"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0308"), "message": String("mismatched types\n\nexpected `u32`, found `u64`\n\nhelp: you can convert a `u64` to a `u32` and panic if the converted value doesn't fit: `super::fetch_u32().try_into().unwrap()`"), "range": Object({"end": Object({"character": Number(62), "line": Number(9)}), "start": Object({"character": Number(44), "line": Number(9)})}), "severity": Number(1), "source": String("rustc")}), Object({"code": String("E0308"), "message": String("mismatched types\n\nexpected due to this"), "range": Object({"end": Object({"character": Number(41), "line": Number(9)}), "start": Object({"character": Number(38), "line": Number(9)})}), "severity": Number(3), "source": String("rustc")}), Object({"code": String("unused_variables"), "message": String("unused variable: `unused`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_unused`"), "range": Object({"end": Object({"character": Number(30), "line": Number(2)}), "start": Object({"character": Number(24), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/library/src/lib.rs")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("E0308"), "message": String("mismatched types\n\nexpected `u32`, found `u64`\n\nhelp: you can convert a `u64` to a `u32` and panic if the converted value doesn't fit: `library::fetch_u32().try_into().unwrap()`"), "range": Object({"end": Object({"character": Number(55), "line": Number(4)}), "start": Object({"character": Number(35), "line": Number(4)})}), "severity": Number(1), "source": String("rustc")}), Object({"code": String("E0308"), "message": String("mismatched types\n\nexpected due to this"), "range": Object({"end": Object({"character": Number(32), "line": Number(4)}), "start": Object({"character": Number(29), "line": Number(4)})}), "severity": Number(3), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/binary/src/main.rs")})})
Sending: Object({"jsonrpc": String("2.0"), "method": String("textDocument/didChange"), "params": Object({"contentChanges": Array([Object({"range": Object({"end": Object({"character": Number(41), "line": Number(1)}), "start": Object({"character": Number(38), "line": Number(1)})}), "rangeLength": Number(3), "text": String("u32")})]), "textDocument": Object({"uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/library/src/lib.rs"), "version": Number(1)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_5"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_5"), "message": String("library cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_0"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_5"), "message": String("library"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_5"), "message": String("binary"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_5"), "message": String("binary cfg(test)"), "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_5"), "message": Null, "percentage": Null, "title": String("Building")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_4"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `unused`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_unused`"), "range": Object({"end": Object({"character": Number(30), "line": Number(2)}), "start": Object({"character": Number(24), "line": Number(2)})}), "severity": Number(2), "source": String("rustc")}), Object({"code": String("unused_variables"), "message": String("unused variable: `test_val`\n\nhelp: if this is intentional, prefix it with an underscore: `_test_val`"), "range": Object({"end": Object({"character": Number(36), "line": Number(9)}), "start": Object({"character": Number(28), "line": Number(9)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/library/src/lib.rs")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("textDocument/publishDiagnostics"), "params": Object({"diagnostics": Array([Object({"code": String("unused_variables"), "message": String("unused variable: `val`\n\nnote: `#[warn(unused_variables)]` on by default\nhelp: if this is intentional, prefix it with an underscore: `_val`"), "range": Object({"end": Object({"character": Number(27), "line": Number(4)}), "start": Object({"character": Number(24), "line": Number(4)})}), "severity": Number(2), "source": String("rustc")})]), "uri": String("file:///checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/simple_workspace/binary/src/main.rs")})})
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_2"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Bool(true), "id": String("progress_4"), "message": Null, "percentage": Null, "title": String("Indexing")})})
Processing message: Object({"id": Number(99999), "jsonrpc": String("2.0"), "result": Null})
Sending: Object({"jsonrpc": String("2.0"), "method": String("exit"), "params": Null})
test client_completion ... ignored
test client_completion_suggests_arguments_in_statements ... ignored
test client_deglob ... ignored
test client_deglob ... ignored
Sending: Object({"id": Number(0), "jsonrpc": String("2.0"), "method": String("initialize"), "params": Object({"capabilities": Object({"window": Object({"progress": Bool(true)})}), "processId": Null, "rootPath": String("/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/rlsit/t0/dependency_typo"), "rootUri": Null})})
Processing message: Object({"id": Number(0), "jsonrpc": String("2.0"), "result": Object({"capabilities": Object({"codeActionProvider": Bool(true), "codeLensProvider": Object({"resolveProvider": Bool(false)}), "completionProvider": Object({"resolveProvider": Bool(true), "triggerCharacters": Array([String("."), String(":")])}), "definitionProvider": Bool(true), "documentFormattingProvider": Bool(true), "documentHighlightProvider": Bool(true), "documentRangeFormattingProvider": Bool(false), "documentSymbolProvider": Bool(true), "executeCommandProvider": Object({"commands": Array([String("rls.applySuggestion-17031"), String("rls.deglobImports-17031")])}), "hoverProvider": Bool(true), "implementationProvider": Bool(true), "referencesProvider": Bool(true), "renameProvider": Bool(true), "textDocumentSync": Number(2), "workspaceSymbolProvider": Bool(true)})})})
Processing message: Object({"jsonrpc": String("2.0"), "method": String("window/progress"), "params": Object({"done": Null, "id": String("progress_1"), "message": Null, "percentage": Null, "title": String("Building")})})
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:265:28
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   2: core::result::unwrap_failed
   3: client::support::client::RlsHandle<T>::wait_for_diagnostics
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
Sending: Object({"id": Number(99999), "jsonrpc": String("2.0"), "method": String("shutdown"), "params": Null})
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Elapsed(())', src/tools/rls/tests/support/client/mod.rs:265:28
stack backtrace:
   0:     0x7f22f347e270 - <std::sys_common::backtrace::_print::DisplayBacktrace as core::fmt::Display>::fmt::h2877a41aa3329311
   1:     0x7f22f34ef70d - core::fmt::write::hf00778d011964c9e
   2:     0x7f22f3472635 - std::io::Write::write_fmt::h5b42d43180d8e64f
   3:     0x7f22f3482807 - std::panicking::default_hook::{{closure}}::he6f3377799392d07
   4:     0x7f22f348221b - std::panicking::default_hook::h4141d462796cd3cb
   5:     0x7f22f3482f02 - std::panicking::rust_panic_with_hook::ha60b3f29ef8dea1d
   6:     0x7f22f3482b37 - std::panicking::begin_panic_handler::{{closure}}::ha8d6c5291f753f4d
   7:     0x7f22f347e70c - std::sys_common::backtrace::__rust_end_short_backtrace::h43f6374aaa3b58b3
   8:     0x7f22f3482a99 - rust_begin_unwind
   9:     0x7f22f3446e11 - core::panicking::panic_fmt::hacef841ef98e28f3
  10:     0x7f22f3447003 - core::result::unwrap_failed::heca1ed5f139fb358
  11:     0x55c5de539ce6 - <client::support::client::RlsHandle<T> as core::ops::drop::Drop>::drop::hfea83b6446cd3e19
  12:     0x55c5de4fa3a3 - core::ptr::drop_in_place<client::support::client::RlsHandle<client::support::client::child_process::ChildProcess>>::h7c46f5ccca7d2a8c
  13:     0x55c5de4f2bfe - core::ops::function::FnOnce::call_once::he41fe5ea6b232188
  14:     0x7f22f37af896 - test::__rust_begin_short_backtrace::h073a2fd87705b6a3
  15:     0x7f22f37ae1cc - test::run_test::run_test_inner::{{closure}}::he37efac29601ad23
  16:     0x7f22f37ad54c - test::run_test::run_test_inner::h0559f099aa3709c2
  17:     0x7f22f37abdf4 - test::run_test::h994aea91409f35f5
  18:     0x7f22f37a5c80 - test::run_tests::hd0939c6cba7f712b
  19:     0x7f22f3795bd0 - test::console::run_tests_console::h8d1d018553378969
  20:     0x7f22f37a3817 - test::test_main::he1a0c04e64df7e8e
  21:     0x7f22f37a49b2 - test::test_main_static::hee6c4e5cf6d18e87
  22:     0x55c5de4cdaf6 - std::sys_common::backtrace::__rust_begin_short_backtrace::h5394b7985aeeb4fa
  23:     0x55c5de4cdb0c - std::rt::lang_start::{{closure}}::h222ea156f9d8e476
  24:     0x7f22f348343a - std::rt::lang_start_internal::h3607458db37df357
  25:     0x55c5de5029f5 - main
  26:     0x7f22f26f9840 - __libc_start_main
  27:     0x55c5de484d19 - _start
  28:                0x0 - <unknown>
thread panicked while panicking. aborting.
error: test failed, to rerun pass '--test client'
Caused by:
Caused by:
  process didn't exit successfully: `/checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/x86_64-unknown-linux-gnu/release/deps/client-c22ae4f720a9c514 --nocapture` (signal: 4, SIGILL: illegal instruction)

command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--" "--nocapture"
expected success, got: exit code: 101

---
  - "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-Zbinary-dep-depinfo" "-j" "16" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/tools/rls/Cargo.toml" "--features" "rustc-workspace-hack/all-static" "--" "--nocapture"

failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 --no-fail-fast --test-args=--nocapture src/tools/rls
Build completed unsuccessfully in 0:28:56
    Blocking waiting for file lock on package cache
thread 'progress-notifier' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   2: core::result::unwrap_failed
   3: <rls::server::io::StdioOutput as rls::server::io::Output>::response
   4: rls::server::io::Output::notify
   5: <rls::actions::progress::BuildDiagnosticsNotifier<O> as rls::actions::progress::DiagnosticsNotifier>::notify_end_diagnostics
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread '<unnamed>' panicked at 'progress-notifier panicked!: Any', src/tools/rls/rls/src/build/mod.rs:417:36
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   2: core::result::unwrap_failed
   3: rls::build::BuildQueue::run_thread
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Os { code: 32, kind: BrokenPipe, message: "Broken pipe" }', src/tools/rls/rls/src/server/io.rs:190:38
   0: rust_begin_unwind
   1: core::panicking::panic_fmt
   2: core::result::unwrap_failed
   2: core::result::unwrap_failed
   3: <rls::server::io::StdioOutput as rls::server::io::Output>::response
   4: rls::server::io::Output::success
   5: rls::server::LsService<O>::dispatch_message
   6: rls::server::run_server
   7: rls::main_inner
   8: rls::main
    Finished dev [unoptimized + debuginfo] target(s) in 7.30s
    Finished dev [unoptimized + debuginfo] target(s) in 7.30s
error: Tool `book` was not recorded in tool state.
error: Tool `nomicon` was not recorded in tool state.
error: Tool `reference` was not recorded in tool state.
error: Tool `rust-by-example` was not recorded in tool state.
error: Tool `edition-guide` was not recorded in tool state.
error: Tool `rustfmt` was not recorded in tool state.
error: Tool `miri` was not recorded in tool state.
error: Tool `embedded-book` was not recorded in tool state.
{"rls":"test-fail"}failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test --stage 2 check-tools
