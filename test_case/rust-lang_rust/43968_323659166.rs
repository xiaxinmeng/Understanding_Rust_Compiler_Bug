
[01:42:00] failures:
[01:42:00] 
[01:42:00] ---- test::test_find_all_refs_no_cfg_test stdout ----
[01:42:00] 	expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion\"]}}}}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsBegin\",\"params\":null}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\",\"params\":null}"],
[01:42:00] expected: [ExpectedMessage { id: Some(0), contains: ["capabilities"] }, ExpectedMessage { id: None, contains: ["diagnosticsBegin"] }, ExpectedMessage { id: None, contains: ["diagnosticsEnd"] }]
[01:42:00] expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":42,\"result\":[]}"],
[01:42:00] expected: [ExpectedMessage { id: Some(42), contains: ["{\"start\":{\"line\":9,\"character\":7},\"end\":{\"line\":9,\"character\":10}}", "{\"start\":{\"line\":22,\"character\":15},\"end\":{\"line\":22,\"character\":18}}"] }]
[01:42:00] thread 'test::test_find_all_refs_no_cfg_test' panicked at 'Could not find `{"start":{"line":9,"character":7},"end":{"line":9,"character":10}}` in `{"jsonrpc":"2.0","id":42,"result":[]}`', /checkout/src/libcore/option.rs:819:4
[01:42:00] 
[01:42:00] ---- test::test_goto_def stdout ----
[01:42:00] 	expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion\"]}}}}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsBegin\",\"params\":null}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\",\"params\":null}"],
[01:42:00] expected: [ExpectedMessage { id: Some(0), contains: ["capabilities"] }, ExpectedMessage { id: None, contains: ["diagnosticsBegin"] }, ExpectedMessage { id: None, contains: ["diagnosticsEnd"] }]
[01:42:00] expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":11,\"result\":[]}"],
[01:42:00] expected: [ExpectedMessage { id: Some(11), contains: ["\"start\":{\"line\":20,\"character\":8}"] }]
[01:42:00] thread 'test::test_goto_def' panicked at 'Could not find `"start":{"line":20,"character":8}` in `{"jsonrpc":"2.0","id":11,"result":[]}`', /checkout/src/libcore/option.rs:819:4
[01:42:00] 
[01:42:00] ---- test::test_find_impls stdout ----
[01:42:00] 	expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion\"]}}}}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsBegin\",\"params\":null}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\",\"params\":null}"],
[01:42:00] expected: [ExpectedMessage { id: Some(0), contains: ["capabilities"] }, ExpectedMessage { id: None, contains: ["diagnosticsBegin"] }, ExpectedMessage { id: None, contains: ["diagnosticsEnd"] }]
[01:42:00] thread 'test::test_find_impls' panicked at 'Analysis: Getting typeid from span: Unclassified', /checkout/src/libcore/result.rs:906:4
[01:42:00] 
[01:42:00] ---- test::test_highlight stdout ----
[01:42:00] 	expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion\"]}}}}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsBegin\",\"params\":null}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\",\"params\":null}"],
[01:42:00] expected: [ExpectedMessage { id: Some(0), contains: ["capabilities"] }, ExpectedMessage { id: None, contains: ["diagnosticsBegin"] }, ExpectedMessage { id: None, contains: ["diagnosticsEnd"] }]
[01:42:00] expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":42,\"result\":[]}"],
[01:42:00] expected: [ExpectedMessage { id: Some(42), contains: ["{\"start\":{\"line\":20,\"character\":8},\"end\":{\"line\":20,\"character\":13}}", "{\"start\":{\"line\":21,\"character\":27},\"end\":{\"line\":21,\"character\":32}}"] }]
[01:42:00] thread 'test::test_highlight' panicked at 'Could not find `{"start":{"line":20,"character":8},"end":{"line":20,"character":13}}` in `{"jsonrpc":"2.0","id":42,"result":[]}`', /checkout/src/libcore/option.rs:819:4
[01:42:00] 
[01:42:00] ---- test::test_hover stdout ----
[01:42:00] 	expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion\"]}}}}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsBegin\",\"params\":null}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\",\"params\":null}"],
[01:42:00] expected: [ExpectedMessage { id: Some(0), contains: ["capabilities"] }, ExpectedMessage { id: None, contains: ["diagnosticsBegin"] }, ExpectedMessage { id: None, contains: ["diagnosticsEnd"] }]
[01:42:00] expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":11,\"result\":{\"contents\":[],\"range\":null}}"],
[01:42:00] expected: [ExpectedMessage { id: Some(11), contains: ["[{\"language\":\"rust\",\"value\":\"&str\"}]"] }]
[01:42:00] thread 'test::test_hover' panicked at 'Could not find `[{"language":"rust","value":"&str"}]` in `{"jsonrpc":"2.0","id":11,"result":{"contents":[],"range":null}}`', /checkout/src/libcore/option.rs:819:4
[01:42:00] 
[01:42:00] ---- test::test_find_all_refs stdout ----
[01:42:00] 	thread 'test::test_find_all_refs' panicked at 'Hit timeout', /checkout/src/tools/rls/src/test/harness.rs:142:12
[01:42:00] 
[01:42:00] ---- test::test_infer_bin stdout ----
[01:42:00] 	thread 'test::test_infer_bin' panicked at 'Hit timeout', /checkout/src/tools/rls/src/test/harness.rs:142:12
[01:42:00] 
[01:42:00] ---- test::test_infer_custom_bin stdout ----
[01:42:00] 	thread 'test::test_infer_custom_bin' panicked at 'Hit timeout', /checkout/src/tools/rls/src/test/harness.rs:142:12
[01:42:00] 
[01:42:00] ---- test::test_infer_lib stdout ----
[01:42:00] 	thread 'test::test_infer_lib' panicked at 'Hit timeout', /checkout/src/tools/rls/src/test/harness.rs:142:12
[01:42:00] 
[01:42:00] ---- test::test_rename stdout ----
[01:42:00] 	expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion\"]}}}}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsBegin\",\"params\":null}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\",\"params\":null}"],
[01:42:00] expected: [ExpectedMessage { id: Some(0), contains: ["capabilities"] }, ExpectedMessage { id: None, contains: ["diagnosticsBegin"] }, ExpectedMessage { id: None, contains: ["diagnosticsEnd"] }]
[01:42:00] expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":42,\"result\":{\"changes\":{}}}"],
[01:42:00] expected: [ExpectedMessage { id: Some(42), contains: ["{\"start\":{\"line\":20,\"character\":8},\"end\":{\"line\":20,\"character\":13}}", "{\"start\":{\"line\":21,\"character\":27},\"end\":{\"line\":21,\"character\":32}}", "{\"changes\""] }]
[01:42:00] thread 'test::test_rename' panicked at 'Could not find `{"start":{"line":20,"character":8},"end":{"line":20,"character":13}}` in `{"jsonrpc":"2.0","id":42,"result":{"changes":{}}}`', /checkout/src/libcore/option.rs:819:4
[01:42:00] 
[01:42:00] ---- test::test_reformat_with_range stdout ----
[01:42:00] 	expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":0,\"result\":{\"capabilities\":{\"textDocumentSync\":2,\"hoverProvider\":true,\"completionProvider\":{\"resolveProvider\":true,\"triggerCharacters\":[\".\",\":\"]},\"definitionProvider\":true,\"referencesProvider\":true,\"documentHighlightProvider\":true,\"documentSymbolProvider\":true,\"workspaceSymbolProvider\":true,\"codeActionProvider\":true,\"documentFormattingProvider\":true,\"documentRangeFormattingProvider\":false,\"renameProvider\":true,\"executeCommandProvider\":{\"commands\":[\"rls.applySuggestion\"]}}}}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsBegin\",\"params\":null}", "{\"jsonrpc\":\"2.0\",\"method\":\"rustDocument/diagnosticsEnd\",\"params\":null}"],
[01:42:00] expected: [ExpectedMessage { id: Some(0), contains: ["capabilities"] }, ExpectedMessage { id: None, contains: ["diagnosticsBegin"] }, ExpectedMessage { id: None, contains: ["diagnosticsEnd"] }]
[01:42:00] expect_messages: results: ["{\"jsonrpc\":\"2.0\",\"id\":42,\"result\":[{\"range\":{\"start\":{\"line\":0,\"character\":0},\"end\":{\"line\":15,\"character\":5}},\"newText\":\"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT\\n// file at the top-level directory of this distribution and at\\n// http://rust-lang.org/COPYRIGHT.\\n//\\n// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or\\n// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license\\n// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your\\n// option. This file may not be copied, modified, or distributed\\n// except according to those terms.\\n\\npub fn main() {\\n    let world1 = \\\"world\\\";\\n    println!(\\\"Hello, {}!\\\", world1);\\n    let world2 = \\\"world\\\";\\n    println!(\\\"Hello, {}!\\\", world2);\\n    let world3 = \\\"world\\\";\\n    println!(\\\"Hello, {}!\\\", world3);\\n}\\n\"}]}"],
[01:42:00] expected: [ExpectedMessage { id: Some(42), contains: ["{\"start\":{\"line\":0,\"character\":0},\"end\":{\"line\":15,\"character\":5}}", "newText\":\"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT\\n// file at the top-level directory of this distribution and at\\n// http://rust-lang.org/COPYRIGHT.\\n//\\n// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or\\n// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license\\n// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your\\n// option. This file may not be copied, modified, or distributed\\n// except according to those terms.\\n\\npub fn main() {\\n    let world1 = \\\"world\\\";\\n    println!(\\\"Hello, {}!\\\", world1);\\n    let world2 = \\\"world\\\";\\n    println!(\\\"Hello, {}!\\\", world2);\\nlet world3 = \\\"world\\\"; println!(\\\"Hello, {}!\\\", world3);\\n}\\n"] }]
[01:42:00] thread 'test::test_reformat_with_range' panicked at 'Could not find `newText":"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT\n// file at the top-level directory of this distribution and at\n// http://rust-lang.org/COPYRIGHT.\n//\n// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or\n// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license\n// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your\n// option. This file may not be copied, modified, or distributed\n// except according to those terms.\n\npub fn main() {\n    let world1 = \"world\";\n    println!(\"Hello, {}!\", world1);\n    let world2 = \"world\";\n    println!(\"Hello, {}!\", world2);\nlet world3 = \"world\"; println!(\"Hello, {}!\", world3);\n}\n` in `{"jsonrpc":"2.0","id":42,"result":[{"range":{"start":{"line":0,"character":0},"end":{"line":15,"character":5}},"newText":"// Copyright 2017 The Rust Project Developers. See the COPYRIGHT\n// file at the top-level directory of this distribution and at\n// http://rust-lang.org/COPYRIGHT.\n//\n// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or\n// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license\n// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your\n// option. This file may not be copied, modified, or distributed\n// except according to those terms.\n\npub fn main() {\n    let world1 = \"world\";\n    println!(\"Hello, {}!\", world1);\n    let world2 = \"world\";\n    println!(\"Hello, {}!\", world2);\n    let world3 = \"world\";\n    println!(\"Hello, {}!\", world3);\n}\n"}]}`', /checkout/src/libcore/option.rs:819:4
[01:42:00] 
[01:42:00] ---- test::test_multiple_binaries stdout ----
[01:42:00] 	thread 'test::test_multiple_binaries' panicked at 'Hit timeout', /checkout/src/tools/rls/src/test/harness.rs:142:12
[01:42:00] 
[01:42:00] ---- test::test_simple_workspace stdout ----
[01:42:00] 	thread 'test::test_simple_workspace' panicked at 'Hit timeout', /checkout/src/tools/rls/src/test/harness.rs:142:12
[01:42:00] 
[01:42:00] 
[01:42:00] failures:
[01:42:00]     test::test_find_all_refs
[01:42:00]     test::test_find_all_refs_no_cfg_test
[01:42:00]     test::test_find_impls
[01:42:00]     test::test_goto_def
[01:42:00]     test::test_highlight
[01:42:00]     test::test_hover
[01:42:00]     test::test_infer_bin
[01:42:00]     test::test_infer_custom_bin
[01:42:00]     test::test_infer_lib
[01:42:00]     test::test_multiple_binaries
[01:42:00]     test::test_reformat_with_range
[01:42:00]     test::test_rename
[01:42:00]     test::test_simple_workspace
[01:42:00] 
[01:42:00] test result: FAILED. 10 passed; 13 failed; 0 ignored; 0 measured; 0 filtered out
