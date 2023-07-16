plain
[01:07:45]    Compiling cargo v0.28.0
[01:07:45]    Compiling cargo v0.30.0 (file:///checkout/src/tools/cargo)
[01:14:08]    Compiling racer v2.1.2
[01:15:42]    Compiling rls-vfs v0.4.6
[01:15:51] warning[E0502]: cannot borrow `packages.0` as mutable because it is also borrowed as immutable
[01:15:51]     |
[01:15:51] 435 |            match packages {
[01:15:51]     |   _________-
[01:15:51]     |  |_________|
[01:15:51]     |  |_________|
[01:15:51]     | ||
[01:15:51] 436 | ||             Some(ref mut packages) if packages.len() == 1 => {
[01:15:51]     | ||                  ^^^^^^^^^^^^^^^^ mutable borrow occurs here
[01:15:51] 437 | ||                 PackageArg::Package(packages.drain().next().unwrap())
[01:15:51] 438 | ||             }
[01:15:51] 439 | ||             _ => PackageArg::All,
[01:15:51] 440 | ||         }
[01:15:51]     | ||         -
[01:15:51]     | ||_________|
[01:15:51]     | |__________immutable borrow occurs here
[01:15:51]     |
[01:15:51]     = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
[01:15:51]             It represents potential unsoundness in your code.
[01:15:51]             This warning will become a hard error in the future.
[01:15:51]             This warning will become a hard error in the future.
[01:15:51] 
[01:15:51] warning[E0502]: cannot borrow `self.target_dir.0.0` as mutable because it is also borrowed as immutable
[01:15:51]    --> tools/rls/src/config.rs:246:40
[01:15:51]     |
[01:15:51] 244 |            match self.target_dir {
[01:15:51]     |   _________-
[01:15:51]     |  |_________|
[01:15:51]     | ||
[01:15:51] 245 | ||             // We require an absolute path, so adjust a relative one if it's passed.
[01:15:51] 246 | ||             Inferrable::Specified(Some(ref mut path)) if path.is_relative() => {
[01:15:51]     | ||                                        ^^^^^^^^^^^^ mutable borrow occurs here
[01:15:51] 247 | ||                 *path = project_dir.join(&path);
[01:15:51] 248 | ||             }
[01:15:51] 249 | ||             _ => {},
[01:15:51] 250 | ||         }
[01:15:51]     | ||         -
[01:15:51]     | ||_________|
[01:15:51]     | |__________immutable borrow occurs here
[01:15:51]     |
[01:15:51]     = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
[01:15:51]             It represents potential unsoundness in your code.
[01:15:51]             This warning will become a hard error in the future.
[01:15:51]             This warning will become a hard error in the future.
[01:15:51] 
[01:17:29]     Finished release [optimized] target(s) in 14m 34s
[01:17:29] travis_fold:end:stage2-rls

[01:17:29] travis_time:end:stage2-rls:start=1532715616136288672,finish=1532716490702787784,duration=874566499112

[01:17:29]    Compiling rls v0.129.0 (file:///checkout/src/tools/rls)
[01:17:36] warning[E0502]: cannot borrow `packages.0` as mutable because it is also borrowed as immutable
[01:17:36]     |
[01:17:36] 435 |            match packages {
[01:17:36]     |   _________-
[01:17:36]     |  |_________|
[01:17:36]     |  |_________|
[01:17:36]     | ||
[01:17:36] 436 | ||             Some(ref mut packages) if packages.len() == 1 => {
[01:17:36]     | ||                  ^^^^^^^^^^^^^^^^ mutable borrow occurs here
[01:17:36] 437 | ||                 PackageArg::Package(packages.drain().next().unwrap())
[01:17:36] 438 | ||             }
[01:17:36] 439 | ||             _ => PackageArg::All,
[01:17:36] 440 | ||         }
[01:17:36]     | ||         -
[01:17:36]     | ||_________|
[01:17:36]     | |__________immutable borrow occurs here
[01:17:36]     |
[01:17:36]     = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
[01:17:36]             It represents potential unsoundness in your code.
[01:17:36]             This warning will become a hard error in the future.
[01:17:36]             This warning will become a hard error in the future.
[01:17:36] 
[01:17:36] warning[E0502]: cannot borrow `self.target_dir.0.0` as mutable because it is also borrowed as immutable
[01:17:36]    --> tools/rls/src/config.rs:246:40
[01:17:36]     |
[01:17:36] 244 |            match self.target_dir {
[01:17:36]     |   _________-
[01:17:36]     |  |_________|
[01:17:36]     | ||
[01:17:36] 245 | ||             // We require an absolute path, so adjust a relative one if it's passed.
[01:17:36] 246 | ||             Inferrable::Specified(Some(ref mut path)) if path.is_relative() => {
[01:17:36]     | ||                                        ^^^^^^^^^^^^ mutable borrow occurs here
[01:17:36] 247 | ||                 *path = project_dir.join(&path);
[01:17:36] 248 | ||             }
[01:17:36] 249 | ||             _ => {},
[01:17:36] 250 | ||         }
[01:17:36]     | ||         -
[01:17:36]     | ||_________|
[01:17:36]     | |__________immutable borrow occurs here
[01:17:36]     |
[01:17:36]     = warning: This error has been downgraded to a warning for backwards compatibility with previous releases.
[01:17:36]             It represents potential unsoundness in your code.
[01:17:36]             This warning will become a hard error in the future.
---
[01:19:16] test server::test::test_use_root_path ... ok
[01:19:16] test server::test::test_use_root_uri ... ok
[01:19:16] test test::fail_uninitialized_request ... FAILED
[01:19:16] test test::ignore_uninitialized_notification ... FAILED
[01:19:16] test test::lens::test_lens_run ... FAILED
[01:19:16] test test::test_all_targets ... FAILED
[01:19:16] test test::test_bin_lib_project ... FAILED
[01:19:16] test test::test_borrow_error ... FAILED
[01:19:16] test test::test_deglob ... FAILED
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_1\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_1\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_0\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_3\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_3\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_2\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16] thread 'test::ignore_uninitialized_notification' panicked at 'assertion failed: `(left == right)`
[01:19:16]   left: `6`,
[01:19:16]  right: `7`', tools/rls/src/test/harness.rs:210:5
[01:19:16] 
[01:19:16] ---- test::lens::test_lens_run stdout ----
[01:19:16]   results: [
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_5\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_5\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_5\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_4\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]             "progress"
[01:19:16]         ]
[01:19:16]     }
[01:19:16] ]
[01:19:16] thread 'test::lens::test_lens_run' panicked at 'assertion failed: `(left == right)`
[01:19:16]  right: `7`', tools/rls/src/test/harness.rs:210:5
[01:19:16] 
[01:19:16] ---- test::test_all_features stdout ----
[01:19:16] expect_messages:
[01:19:16] expect_messages:
[01:19:16]   results: [
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_7\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_7\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_6\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_9\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_9\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_8\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]             "bin_lib/tests/tests.rs",
[01:19:16]             "unused variable: `unused_var`"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
---
[01:19:16] error: test failed, to rerun pass '--bin rls'
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_11\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_11\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_10\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]             "bin_lib/tests/tests.rs",
[01:19:16]             "unused variable: `unused_var`"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_15\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_15\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_14\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]             "\"message\":\"cannot borrow `x` as mutable more than once at a time"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_13\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_13\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_12\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]             "progress",
[01:19:16]             "deglob"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]         contains: [
[01:19:16]             "progress",
[01:19:16]             "deglob"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_17\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_17\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_16\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]             "\"message\":\"cannot find struct, variant or union type `Bar` in this scope"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_19\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_19\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_18\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]             "\"done\":true"
[01:19:16]         ]
[01:19:16]     }
[01:19:16] ]
[01:19:16] thread 'test::test_find_all_refs' panicked at 'Could not find `completion` in `{"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_19","title":"Building"}}`', libcore/option.rs:989:5
[01:19:16] ---- test::test_find_all_refs_no_cfg_test stdout ----
[01:19:16] expect_messages:
[01:19:16]   results: [
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_21\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_21\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_20\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]             "\"done\":true"
[01:19:16]         ]
[01:19:16]     }
[01:19:16] ]
[01:19:16] thread 'test::test_find_all_refs_no_cfg_test' panicked at 'Could not find `find_all_refs_no_cfg_test` in `{"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_21","title":"Building"}}`', libcore/option.rs:989:5
[01:19:16] ---- test::test_find_impls stdout ----
[01:19:16] expect_messages:
[01:19:16]   results: [
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_23\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_23\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_22\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_25\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_25\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_24\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_27\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_27\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_26\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_29\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_29\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_28\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_31\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_31\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_30\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_33\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_33\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_32\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]             "progress",
[01:19:16]             "infer_bin"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]         contains: [
[01:19:16]             "progress",
[01:19:16]             "infer_bin"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_35\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_35\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_34\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]             "struct is never used: `UnusedCustomBin`"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_37\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_37\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_36\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]             "progress",
[01:19:16]             "infer_lib"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]         contains: [
[01:19:16]             "progress",
[01:19:16]             "infer_lib"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_41\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_41\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_40\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]             "\"message\":\"cannot find struct, variant or union type `Baz` in this scope"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_39\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_39\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_38\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]             "unused variable: `bin_name"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
[01:19:16]         contains: [
[01:19:16]             "unused variable: `bin_name"
[01:19:16]     },
[01:19:16]     ExpectedMessage {
[01:19:16]         id: None,
[01:19:16]         contains: [
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_45\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_45\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_44\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_43\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_43\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_42\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_47\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_47\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_46\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_49\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_49\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_48\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_51\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_51\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_50\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]             "\"done\":true"
[01:19:16]         ]
[01:19:16]     }
[01:19:16] ]
[01:19:16] thread 'test::test_workspace_symbol' panicked at 'Could not find `workspace_symbol` in `{"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_51","title":"Building"}}`', libcore/option.rs:989:5
[01:19:16] ---- test::test_workspace_symbol_duplicates stdout ----
[01:19:16] expect_messages:
[01:19:16]   results: [
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"implementationProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"codeLensProvider\":{\"resolveProvider\":false},\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion-31593\",\"rls.deglobImports-31593\"]}}}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_53\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"done\":true,\"id\":\"progress_53\",\"title\":\"Building\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/progress\",\"params\":{\"id\":\"progress_52\",\"title\":\"Indexing\"}}",
[01:19:16]     "{\"jsonrpc\":\"2.0\",\"method\":\"window/showMessage\",\"params\":{\"message\":\"Cargo failed: Error compiling dependent crate\",\"type\":1}}",
[01:19:16] ],
[01:19:16]   expected: [
[01:19:16]     ExpectedMessage {
[01:19:16]         id: Some(
---
[01:19:16]             "\"done\":true"
[01:19:16]         ]
[01:19:16]     }
[01:19:16] ]
[01:19:16] thread 'test::test_workspace_symbol_duplicates' panicked at 'Could not find `workspace_symbol` in `{"jsonrpc":"2.0","method":"window/progress","params":{"done":true,"id":"progress_53","title":"Building"}}`', libcore/option.rs:989:5
[01:19:16] 
[01:19:16] failures:
[01:19:16]     test::fail_uninitialized_request
[01:19:16]     test::ignore_uninitialized_notification
[01:19:16]     test::ignore_uninitialized_notification
[01:19:16]     test::lens::test_lens_run
[01:19:16]     test::test_all_targets
[01:19:16]     test::test_bin_lib_project
[01:19:16]     test::test_borrow_error
[01:19:16]     test::test_deglob
---
[01:24:32] test shape::test::shape_visual_indent ... ok
[01:24:32] test string::test::nothing_to_break ... ok
[01:24:32] test string::test::big_whitespace ... ok
[01:24:32] test string::test::issue343 ... ok
[01:24:32] test string::test::should_break_forward ... ok
[01:24:32] test string::test::should_break_on_punctuation ... ok
[01:24:32] test string::test::significant_whitespaces ... ok
[01:24:32] test string::test::should_break_on_whitespace ... ok
[01:24:32] test test::format_lines_errors_are_reported ... ok
[01:24:32] test test::format_lines_errors_are_reported_with_tabs ... ok
[01:24:32] test test::modified_test ... ok
[01:24:32] test test::idempotence_tests ... ok
---
[01:25:38] test [ui] ui/wrong_self_convention.rs ... ok
5:38] error: redundant field names in struct initialization
[01:25:38]   --> $DIR/redundant_field_names.rs:35:9
[01:25:38]    |
[01:25:38] 35 |         age: age,
[01:25:38]    |         ^^^^^^^^ help: replace it with: `age`
[01:25:38] error: redundant field names in struct initialization
[01:25:38]   --> $DIR/redundant_field_names.rs:53:25
[01:25:38]    |
[01:25:38]    |
[01:25:38] 53 |     let _ = RangeFrom { start: start };
[01:25:38]    |                         ^^^^^^^^^^^^ help: replace it with: `start`
[01:25:38] error: redundant field names in struct initialization
[01:25:38]   --> $DIR/redundant_field_names.rs:54:23
[01:25:38]    |
[01:25:38]    |
[01:25:38] 54 |     let _ = RangeTo { end: end };
[01:25:38]    |                       ^^^^^^^^ help: replace it with: `end`
[01:25:38] error: redundant field names in struct initialization
[01:25:38]   --> $DIR/redundant_field_names.rs:55:21
[01:25:38]    |
[01:25:38]    |
[01:25:38] 55 |     let _ = Range { start: start, end: end };
[01:25:38]    |                     ^^^^^^^^^^^^ help: replace it with: `start`
[01:25:38] error: redundant field names in struct initialization
[01:25:38]   --> $DIR/redundant_field_names.rs:55:35
[01:25:38]    |
[01:25:38]    |
[01:25:38] 55 |     let _ = Range { start: start, end: end };
[01:25:38]    |                                   ^^^^^^^^ help: replace it with: `end`
[01:25:38] error: redundant field names in struct initialization
[01:25:38]   --> $DIR/redundant_field_names.rs:57:32
[01:25:38]    |
[01:25:38]    |
[01:25:38] 57 |     let _ = RangeToInclusive { end: end };
[01:25:38]    |                                ^^^^^^^^ help: replace it with: `end`
[01:25:38] error: aborting due to 7 previous errors
[01:25:38] 
[01:25:38] 
[01:25:38] 
[01:25:38] 
[01:25:38] diff of stderr:
[01:25:38] 
[01:25:38] -error: redundant field names in struct initialization
[01:25:38] -  --> $DIR/redundant_field_names.rs:34:9
[01:25:38] -   |
[01:25:38] -34 |         gender: gender,
[01:25:38] -   |         ^^^^^^^^^^^^^^ help: replace it with: `gender`
[01:25:38] -   = note: `-D redundant-field-names` implied by `-D warnings`
[01:25:38] -   = note: `-D redundant-field-names` implied by `-D warnings`
[01:25:38] +error[E0464]: multiple matching crates for `derive_new`
[01:25:38] + --> $DIR/redundant_field_names.rs:6:1
[01:25:38] +  |
[01:25:38] +6 | extern crate derive_new;
[01:25:38] +  | ^^^^^^^^^^^^^^^^^^^^^^^^
[01:25:38] +  |
[01:25:38] +  = note: candidates:
[01:25:38] +          crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-24d68d6f9f8be909.so
[01:25:38] +          crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-cc2a0602cb5c0153.so
[01:25:38] -error: redundant field names in struct initialization
[01:25:38] -  --> $DIR/redundant_field_names.rs:35:9
[01:25:38] -   |
[01:25:38] -   |
[01:25:38] -35 |         age: age,
[01:25:38] -   |         ^^^^^^^^ help: replace it with: `age`
[01:25:38] +error[E0463]: can't find crate for `derive_new`
[01:25:38] + --> $DIR/redundant_field_names.rs:6:1
[01:25:38] +  |
[01:25:38] +6 | extern crate derive_new;
[01:25:38] +  | ^^^^^^^^^^^^^^^^^^^^^^^^ can't find crate
[01:25:38] -error: redundant field names in struct initialization
[01:25:38] -  --> $DIR/redundant_field_names.rs:53:25
[01:25:38] -   |
[01:25:38] -   |
[01:25:38] -53 |     let _ = RangeFrom { start: start };
[01:25:38] -   |                         ^^^^^^^^^^^^ help: replace it with: `start`
[01:25:38]  
[01:25:38] -error: redundant field names in struct initialization
[01:25:38] -  --> $DIR/redundant_field_names.rs:54:23
[01:25:38] -   |
[01:25:38] -   |
[01:25:38] -54 |     let _ = RangeTo { end: end };
[01:25:38] -   |                       ^^^^^^^^ help: replace it with: `end`
[01:25:38] -error: redundant field names in struct initialization
[01:25:38] -  --> $DIR/redundant_field_names.rs:55:21
[01:25:38] -   |
[01:25:38] -   |
[01:25:38] -55 |     let _ = Range { start: start, end: end };
[01:25:38] -   |                     ^^^^^^^^^^^^ help: replace it with: `start`
[01:25:38] -error: redundant field names in struct initialization
[01:25:38] -  --> $DIR/redundant_field_names.rs:55:35
[01:25:38] -   |
[01:25:38] -   |
[01:25:38] -55 |     let _ = Range { start: start, end: end };
[01:25:38] -   |                                   ^^^^^^^^ help: replace it with: `end`
[01:25:38] -error: redundant field names in struct initialization
[01:25:38] -  --> $DIR/redundant_field_names.rs:57:32
[01:25:38] -   |
[01:25:38] -   |
[01:25:38] -57 |     let _ = RangeToInclusive { end: end };
[01:25:38] -   |                                ^^^^^^^^ help: replace it with: `end`
[01:25:38] -error: aborting due to 7 previous errors
[01:25:38] -
[01:25:38] +Some errors occurred: E0463, E0464.
[01:25:38] +For more information about an error, try `rustc --explain E0463`.
---
[01:25:38] 
[01:25:38] ------------------------------------------
[01:25:38] stderr:
[01:25:38] ------------------------------------------
[01:25:38] error[E0464]: multiple matching crates for `derive_new`
[01:25:38]   |
[01:25:38] 6 | extern crate derive_new;
[01:25:38]   | ^^^^^^^^^^^^^^^^^^^^^^^^
[01:25:38]   |
[01:25:38]   |
[01:25:38]   = note: candidates:
[01:25:38]           crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-24d68d6f9f8be909.so
[01:25:38]           crate `derive_new`: /checkout/obj/build/x86_64-unknown-linux-gnu/stage2-tools/release/deps/libderive_new-cc2a0602cb5c0153.so
[01:25:38] error[E0463]: can't find crate for `derive_new`
[01:25:38]  --> tests/ui/redundant_field_names.rs:6:1
[01:25:38]   |
[01:25:38] 6 | extern crate derive_new;
---
[01:25:38] Verifying status of rust-by-example...
[01:25:38] Verifying status of rls...
[01:25:38] This PR updated 'src/tools/rls', verifying if status is 'test-pass'...
[01:25:38] 
[01:25:38] ⚠️ We detected that this PR updated 'rls', but its tests failed.
[01:25:38] 
[01:25:38] If you do intend to update 'rls', please check the error messages above and
[01:25:38] commit another update.
[01:25:38] 
[01:25:38] If you do NOT intend to update 'rls', please ensure you did not accidentally
[01:25:38] change the submodule at 'src/tools/rls'. You may ask your reviewer for the
[01:25:38] proper steps.

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 3.
travis_time:start:0a81d0a2
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
