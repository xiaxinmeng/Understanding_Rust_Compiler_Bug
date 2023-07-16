\n"},"level":"error","spans":[{"file_name":"/Users/travis/build/rust-lang/rust/src/test/compile-fail/use-keyword.rs","byte_start":634,"byte_end":643,"line_start":16,"line_end":16,"column_start":13,"column_end":22,"is_primary":true,"text":[{"text":"        use self as A;","highlight_start":13,"highlight_end":22}],"label":null,"suggested_replacement":null,"expansion":null}],"children":[],"rendered":"error[E0429]: `self` imports are only allowed within a { } list\n  --> /Users/travis/build/rust-lang/rust/src/test/compile-fail/use-keyword.rs:16:13\n   |\n16 |         use self as A;\n   |             ^^^^^^^^^\n\n"}
[01:51:01] thread 'rustc' panicked at 'librustc_resolve/resolve_imports.rs:1040: invalid name `::super` at Span { lo: BytePos(726), hi: BytePos(736), ctxt: #0 }; global = false, names = [], subclass = SingleImport { target: B#0, source: super#0, result: PerNS { value_ns: Cell { value: Err(Determined) }, type_ns: Cell { value: Err(Determined) }, macro_ns: None }, type_ns_only: false }', librustc/session/mod.rs:1180:26
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
[01:51:01] thread '[compile-fail] compile-fail/use-keyword.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2883:9
[01:51:01] 
[01:51:01] ---- [compile-fail] compile-fail/use-mod-2.rs stdout ----
[01:51:01] 	
[01:51:01] error: compiler encountered internal error
[01:51:01] status: exit code: 101
[01:51:01] command: "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/stage2/bin/rustc" "/Users/travis/build/rust-lang/rust/src/test/compile-fail/use-mod-2.rs" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/compile-fail" "--target=x86_64-apple-darwin" "--error-format" "json" "-C" "prefer-dynamic" "-o" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/compile-fail/use-mod-2.stage2-x86_64-apple-darwin" "-Crpath" "-O" "-Zmiri" "-Zunstable-options" "-Lnative=/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/native/rust-test-helpers" "-L" "/Users/travis/build/rust-lang/rust/build/x86_64-apple-darwin/test/compile-fail/use-mod-2.stage2-x86_64-apple-darwin.aux" "-A" "unused"
[01:51:01] stdout:
[01:51:01] ------------------------------------------
[01:51:01] 
[01:51:01] ------------------------------------------
[01:51:01] stderr:
[01:51:01] ------------------------------------------
[01:51:01] thread 'rustc' panicked at 'librustc_resolve/resolve_imports.rs:1040: invalid name `::self` at Span { lo: BytePos(492), hi: BytePos(496), ctxt: #0 }; global = false, names = [], subclass = SingleImport { target: self#0, source: self#0, result: PerNS { value_ns: Cell { value: Err(Undetermined) }, type_ns: Cell { value: Err(Determined) }, macro_ns: None }, type_ns_only: true }', librustc/session/mod.rs:1180:26
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
[01:51:01] thread '[compile-fail] compile-fail/use-mod-2.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:2883:9
[01:51:01] 
[01:51:01] 
[01:51:01] failures:
[01:51:01]     [compile-fail] compile-fail/rfc-2126-extern-in-paths/single-segment.rs
[01:51:01]     [compile-fail] compile-fail/use-keyword.rs
[01:51:01]     [compile-fail] compile-fail/use-mod-2.rs
