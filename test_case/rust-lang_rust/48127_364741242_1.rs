
[01:51:01] failures:
[01:51:01] 
[01:51:01] ---- [compile-fail] compile-fail/rfc-2126-extern-in-paths/single-segment.rs stdout ----
[01:51:01] 	
[01:51:01] error: compiler encountered internal error
[01:51:01] status: exit code: 101
[01:51:01] command: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/travis/build/rust-lang/rust/src/test/compile-fail/rfc-2126-extern-in-paths/single-segment.rs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/compile-fail" "--target=x86_64-apple-darwin" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/compile-fail/rfc-2126-extern-in-paths/single-segment.stage2-x86_64-apple-darwin" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/compile-fail/rfc-2126-extern-in-paths/single-segment.stage2-x86_64-apple-darwin.aux" "-A" "unused"
[01:51:01] stdout:
[01:51:01] ------------------------------------------
[01:51:01] 
[01:51:01] ------------------------------------------
[01:51:01] stderr:
[01:51:01] ------------------------------------------
[01:51:01] {"message":"unresolved import `extern::*`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n