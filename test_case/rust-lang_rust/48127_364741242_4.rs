\n"},"level":"error","spans":[{"file_name":"/Users/travis/build/rust-lang/rust/src/test/compile-fail/rfc-2126-extern-in-paths/single-segment.rs","byte_start":620,"byte_end":629,"line_start":17,"line_end":17,"column_start":5,"column_end":14,"is_primary":true,"text":[{"text":"use extern::*; //~ ERROR unresolved import `extern::*`","highlight_start":5,"highlight_end":14}],"label":"cannot glob-import all possible crates","suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0432]: unresolved import `extern::*`\n  --> /Users/travis/build/rust-lang/rust/src/test/compile-fail/rfc-2126-extern-in-paths/single-segment.rs:17:5\n   |\n17 | use extern::*; //~ ERROR unresolved import `extern::*`\n   |     ^^^^^^^^^ cannot glob-import all possible crates\n\n"}
[01:51:01] thread 'rustc' panicked at 'librustc_resolve/resolve_imports.rs:1040: invalid name `::extern` at Span { lo: BytePos(525), hi: BytePos(531), ctxt: #0 }; global = false, names = [], subclass = SingleImport { target: extern#0, source: extern#0, result: PerNS { value_ns: Cell { value: Err(Determined) }, type_ns: Cell { value: Err(Determined) }, macro_ns: None }, type_ns_only: false }', librustc/session/mod.rs:1180:26
[01:51:01] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[01:51:01] 
[01:51:01] error: internal compiler error: unexpected panic
[01:51:01] 
[01:51:01] note: the compiler unexpectedly panicked. this is a bug.
[01:51:01] 
[01:51:01] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[01:51:01] 
[01:51:01] note: rustc 1.25.0-dev running on x86_64-apple-darwin
[01:51:01] 
[01:51:01] 
[01:51:01] ------------------------------------------
[01:51:01] 
[01:51:01] thread '[compile-fail] compile-fail/rfc-2126-extern-in-paths/single-segment.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2883:9
[01:51:01] 
[01:51:01] ---- [compile-fail] compile-fail/use-keyword.rs stdout ----
[01:51:01] 	
[01:51:01] error: compiler encountered internal error
[01:51:01] status: exit code: 101
[01:51:01] command: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/travis/build/rust-lang/rust/src/test/compile-fail/use-keyword.rs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/compile-fail" "--target=x86_64-apple-darwin" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/compile-fail/use-keyword.stage2-x86_64-apple-darwin" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/compile-fail/use-keyword.stage2-x86_64-apple-darwin.aux" "-A" "unused"
[01:51:01] stdout:
[01:51:01] ------------------------------------------
[01:51:01] 
[01:51:01] ------------------------------------------
[01:51:01] stderr:
[01:51:01] ------------------------------------------
[01:51:01] {"message":"`self` imports are only allowed within a { } list","code":{"code":"E0429","explanation":"\nThe `self` keyword cannot appear alone as the last segment in a `use`\ndeclaration.\n\nErroneous code example:\n\n