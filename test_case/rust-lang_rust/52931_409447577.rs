
[01:13:25] ---- test::test_infer_bin stdout ----
...
[01:13:25]     ExpectedMessage {
[01:13:25]         id: None,
[01:13:25]         contains: [
[01:13:25]             "struct is never used: `UnusedBin`"
[01:13:25]         ]
[01:13:25]     },
...
[01:13:25] thread 'test::test_infer_bin' panicked at 'Could not find `struct is never used: `UnusedBin`` in `{"jsonrpc":"2.0","method":"textDocument/publishDiagnostics","params":{"diagnostics":[{"code":"dead_code","message":"struct is never constructed: `UnusedBin`\n\nnote: #[warn(dead_code)] on by default","range":{"end":{"character":17,"line":0},"start":{"character":0,"line":0}},"severity":2,"source":"rustc"}],"uri":"file:///C:/projects/rust/build/rls-test-data/test_data/infer_bin/src/main.rs"}}`', libcore\option.rs:1000:5
...
[01:13:25] failures:
[01:13:25]     test::test_infer_bin
[01:13:25]     test::test_infer_custom_bin
[01:13:25]     test::test_infer_lib
[01:13:25] 
[01:13:25] test result: FAILED. 66 passed; 3 failed; 0 ignored; 0 measured; 0 filtered out
