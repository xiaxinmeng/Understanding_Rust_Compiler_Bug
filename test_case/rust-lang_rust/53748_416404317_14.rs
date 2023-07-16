\n"},"level":"error","spans":[{"file_name":"/checkout/src/test/ui/error-codes/E0405.rs","byte_start":485,"byte_end":494,"line_start":13,"line_end":13,"column_start":6,"column_end":15,"is_primary":true,"text":[{"text":"impl SomeTrait for Foo {} //~ ERROR E0405","highlight_start":6,"highlight_end":15}],"label":"not found in this scope","suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":"error[E0405]: cannot find trait `SomeTrait` in this scope\n  --> /checkout/src/test/ui/error-codes/E0405.rs:13:6\n   |\nLL | impl SomeTrait for Foo {} //~ ERROR E0405\n   |      ^^^^^^^^^ not found in this scope\n\n"}
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
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] thread '[ui] ui/error-codes/E0405.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:57] thread '[ui] ui/error-codes/E0405.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3196:9
[00:47:57] 
[00:47:57] ---- [ui] ui/extern/extern-with-type-bounds.rs stdout ----
[00:47:57] 
[00:47:57] error: Error: expected failure status (Some(1)) but received status Some(101).
[00:47:57] status: exit code: 101
[00:47:57] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/extern/extern-with-type-bounds.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-with-type-bounds/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/extern/extern-with-type-bounds/auxiliary" "-A" "unused"
[00:47:57] ------------------------------------------
[00:47:57] 
[00:47:57] ------------------------------------------
[00:47:57] stderr:
[00:47:57] stderr:
[00:47:57] ------------------------------------------
[00:47:57] {"message":"cannot find trait `NoSuchTrait` in this scope","code":{"code":"E0405","explanation":"\nThe code refers to a trait that is not in scope.\n\nErroneous code example:\n\n