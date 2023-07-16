\n\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/codemap_tests/two_files.rs","byte_start":520,"byte_end":523,"line_start":15,"line_end":15,"column_start":6,"column_end":9,"is_primary":true,"text":[{"text":"impl Bar for Baz { } //~ ERROR expected trait, found type alias","highlight_start":6,"highlight_end":9}],"label":"type aliases cannot be used for traits","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0404]: expected trait, found type alias `Bar`\n  --> /checkout/src/test/ui/codemap_tests/two_files.rs:15:6\n   |\nLL | impl Bar for Baz { } //~ ERROR expected trait, found type alias\n   |      ^^^ type aliases cannot be used for traits\n\n"}
[00:47:57] thread 'main' panicked at 'internal error: entered unreachable code', librustc_typeck/astconv.rs:696:18
[00:47:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:57] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:57] {"message":"For more information about this error, try `rustc --explain E0404`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0404`.\n"}
[00:47:57] error: internal compiler error: unexpected panic
[00:47:57] 
[00:47:57] note: the compiler unexpectedly panicked. this is a bug.
[00:47:57] 
[00:47:57] 
[00:47:57] note: we would appreciate a bug report: https://github.com/rust-lang/rust/blob/master/CONTRIBUTING.md#bug-reports
[00:47:57] 
[00:47:57] note: rustc 1.30.0-dev running on x86_64-unknown-linux-gnu
[00:47:57] 
[00:47:57] note: compiler flags: -Z ui-testing -Z unstable-options -C prefer-dynamic -C rpath
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] thread '[ui] ui/codemap_tests/two_files.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:57] thread '[ui] ui/codemap_tests/two_files.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:57] 
[00:47:57] ---- [ui] ui/error-codes/E0404.rs stdout ----
[00:47:57] 
[00:47:57] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:57] status: exit code: 101
[00:47:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/error-codes/E0404.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0404/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/error-codes/E0404/auxiliary" "-A" "unused"
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] stderr:
[00:47:57] stderr:
[00:47:57] ------------------------------------------
[00:47:57] {"message":"expected trait, found struct `Foo`","code":{"code":"E0404","explanation":"\nYou tried to use something which is not a trait in a trait position, such as\na bound or `impl`.\n\nErroneous code example:\n\n