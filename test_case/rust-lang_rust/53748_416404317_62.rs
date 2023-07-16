\n# #[cfg(for_demonstration_only)]\n// solutionSome errors occurred: E0405, E0412.\n"}
[00:47:57] {"message":"For more information about an error, try `rustc --explain E0405`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about an error, try `rustc --explain E0405`.\n"}
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
[00:47:57] thread '[ui] ui/resolve/issue-21221-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:57] thread '[ui] ui/resolve/issue-21221-1.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:57] 
[00:47:57] ---- [ui] ui/resolve/issue-21221-3.rs stdout ----
[00:47:57] 
[00:47:57] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:57] status: exit code: 101
[00:47:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-21221-3.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-21221-3/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-":"struct Foo;","highlight_start":1,"highlight_end":1}],"label":null,"suggested_replacement":"use issue_21221_3::outer::OuterTrait;\n\n","suggestion_applicability":"Unspecified","expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0405]: cannot find trait `OuterTrait` in this scope\n  --> /checkout/src/test/ui/resolve/issue-21221-3.rs:25:6\n   |\nLL | impl OuterTrait for Foo {}\n   |      ^^^^^^^^^^ not found in this scope\nhelp: possible candidate is found in another module, you can import it into scope\n   |\nLL | use issue_21221_3::outer::OuterTrait;\n   |\n\n"}
[00:47:57] thread 'main' panicked at 'internal error: entered unreachable code', librustc_typeck/astconv.rs:696:18
[00:47:57] note: Run with `RUST_BACKTRACE=1` for a backtrace.
[00:47:57] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:57] {"message":"For more information about this error, try `rustc --explain E0405`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0405`.\n"}
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
[00:47:,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[00:47:57] {"message":"For more information about this error, try `rustc --explain E0405`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0405`.\n"}
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
[00:47:57] thread '[ui] ui/resolve/issue-21221-4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:57] thread '[ui] ui/resolve/issue-21221-4.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:57] 
[00:47:57] ---- [ui] ui/resolve/issue-3907.rs stdout ----
[00:47:57] 
[00:47:57] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:57] status: exit code: 101
[00:47:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/resolve/issue-3907.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-3907/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/resolve/issue-3907/auxiliary" "-A" "unused"
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] stderr:
[00:47:57] stderr:
[00:47:57] ------------------------------------------
[00:47:57] {"message":"expected trait, found type alias `Foo`","code":{"code":"E0404","explanation":"\nYou tried to use something which is not a trait in a trait position, such as\na bound or `impl`.\n\nErroneous code example:\n\n