plain
travis_time:end:25e1a07c:start=1555995655459651856,finish=1555995740167855959,duration=84708204103
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export AWS_ACCESS_KEY_ID=AKIA46X5W6CZEJZ6XT55
---
[01:04:07] 34 fn main() {}
[01:04:07] + 
[01:04:07] 
[01:04:07] 
[01:04:07] The actual fixed differed from the expected fixed.
[01:04:07] Actual fixed saved to /checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt/expr-as-stmt.fixed
[01:04:07] To update references, rerun the tests and pass the `--bless` flag
[01:04:07] To only update this specific test, also pass `--test-args parser/expr-as-stmt.rs`
[01:04:07] error: 1 errors occurred comparing output.
[01:04:07] status: exit code: 1
[01:04:07] status: exit code: 1
[01:04:07] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/ui/parser/expr-as-stmt.rs" "-Zthreads=1" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui/parser/expr-as-stmt/auxiliary" "-A" "unused"
[01:04:07] ------------------------------------------
[01:04:07] 
[01:04:07] ------------------------------------------
[01:04:07] stderr:
[01:04:07] stderr:
[01:04:07] ------------------------------------------
[01:04:07] error: expected expression, found `+`
[01:04:07]   --> /checkout/src/test/ui/parser/expr-as-stmt.rs:7:9
[01:04:07]    |
[01:04:07] LL |     {2} + {2} //~ ERROR expected expression, found `+`
[01:04:07]    |     --- ^ expected expression
[01:04:07]    |     |
[01:04:07]    |     help: parenthesis are required to parse this as an expression: `({2})`
[01:04:07] error: expected expression, found `+`
[01:04:07]   --> /checkout/src/test/ui/parser/expr-as-stmt.rs:12:9
[01:04:07]    |
[01:04:07]    |
[01:04:07] LL |     {2} + 2 //~ ERROR expected expression, found `+`
[01:04:07]    |     --- ^ expected expression
[01:04:07]    |     |
[01:04:07]    |     help: parenthesis are required to parse this as an expression: `({2})`
[01:04:07] error: expected expression, found `+`
[01:04:07]   --> /checkout/src/test/ui/parser/expr-as-stmt.rs:18:12
[01:04:07]    |
[01:04:07]    |
[01:04:07] LL |     { 42 } + foo; //~ ERROR expected expression, found `+`
[01:04:07]    |     ------ ^ expected expression
[01:04:07]    |     |
[01:04:07]    |     help: parenthesis are required to parse this as an expression: `({ 42 })`
[01:04:07] error: ambiguous parse
[01:04:07]   --> /checkout/src/test/ui/parser/expr-as-stmt.rs:30:5
[01:04:07]    |
[01:04:07] LL |     if let Some(x) = a { true } else { false }
[01:04:07] LL |     if let Some(x) = a { true } else { false }
[01:04:07]    |     ------------------------------------------ help: parenthesis are required to parse this as an expression: `(if let Some(x) = a { true } else { false })`
[01:04:07] LL |     && //~ ERROR ambiguous parse
[01:04:07] 
[01:04:07] error[E0308]: mismatched types
[01:04:07]   --> /checkout/src/test/ui/parser/expr-as-stmt.rs:7:6
[01:04:07]    |
[01:04:07]    |
[01:04:07] LL |     {2} + {2} //~ ERROR expected expression, found `+`
[01:04:07]    |      ^ expected (), found integer
[01:04:07]    = note: expected type `()`
[01:04:07]               found type `{integer}`
[01:04:07] 
[01:04:07] error[E0308]: mismatched types
[01:04:07] error[E0308]: mismatched types
[01:04:07]   --> /checkout/src/test/ui/parser/expr-as-stmt.rs:12:6
[01:04:07]    |
[01:04:07] LL |     {2} + 2 //~ ERROR expected expression, found `+`
[01:04:07]    |      ^ expected (), found integer
[01:04:07]    = note: expected type `()`
[01:04:07]               found type `{integer}`
[01:04:07] 
[01:04:07] error[E0308]: mismatched types
[01:04:07] error[E0308]: mismatched types
[01:04:07]   --> /checkout/src/test/ui/parser/expr-as-stmt.rs:18:7
[01:04:07]    |
[01:04:07] LL |     { 42 } + foo; //~ ERROR expected expression, found `+`
[01:04:07]    |       ^^ expected (), found integer
[01:04:07]    = note: expected type `()`
[01:04:07]               found type `{integer}`
[01:04:07] 
[01:04:07] error[E0308]: mismatched types
[01:04:07] error[E0308]: mismatched types
[01:04:07]   --> /checkout/src/test/ui/parser/expr-as-stmt.rs:24:7
[01:04:07]    |
[01:04:07] LL |     { 3 } * 3 //~ ERROR type `{integer}` cannot be dereferenced
[01:04:07]    |       ^ expected (), found integer
[01:04:07]    = note: expected type `()`
[01:04:07]               found type `{integer}`
[01:04:07] 
[01:04:07] error[E0614]: type `{integer}` cannot be dereferenced
[01:04:07] error[E0614]: type `{integer}` cannot be dereferenced
[01:04:07]   --> /checkout/src/test/ui/parser/expr-as-stmt.rs:24:11
[01:04:07]    |
[01:04:07] LL |     { 3 } * 3 //~ ERROR type `{integer}` cannot be dereferenced
[01:04:07]    |     ----- ^^^
[01:04:07]    |     |
[01:04:07]    |     help: parenthesis are required to parse this as an expression: `({ 3 })`
[01:04:07] error: aborting due to 9 previous errors
[01:04:07] 
[01:04:07] Some errors have detailed explanations: E0308, E0614.
[01:04:07] For more information about an error, try `rustc --explain E0308`.
---
[01:04:07] thread 'main' panicked at 'Some tests failed', src/tools/compiletest/src/main.rs:517:22
[01:04:07] note: Run with `RUST_BACKTRACE=1` environment variable to display a backtrace.
[01:04:07] 
[01:04:07] 
[01:04:07] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/compiletest" "--compile-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib" "--run-lib-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/lib/rustlib/x86_64-unknown-linux-gnu/lib" "--rustc-path" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "--src-base" "/checkout/src/test/ui" "--build-base" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/ui" "--stage-id" "stage2-x86_64-unknown-linux-gnu" "--mode" "ui" "--target" "x86_64-unknown-linux-gnu" "--host" "x86_64-unknown-linux-gnu" "--llvm-filecheck" "/usr/lib/llvm-6.0/bin/FileCheck" "--host-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--target-rustcflags" "-Crpath -O -Zunstable-options  -Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "--docck-python" "/usr/bin/python2.7" "--lldb-python" "/usr/bin/python2.7" "--gdb" "/usr/bin/gdb" "--quiet" "--llvm-version" "6.0.0\n" "--system-llvm" "--cc" "" "--cxx" "" "--cflags" "" "--llvm-components" "" "--llvm-cxxflags" "" "--adb-path" "adb" "--adb-test-dir" "/data/tmp/work" "--android-cross-path" "" "--color" "always"
[01:04:07] 
[01:04:07] 
[01:04:07] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:04:07] Build completed unsuccessfully in 0:04:10
[01:04:07] Build completed unsuccessfully in 0:04:10
[01:04:07] make: *** [check] Error 1
[01:04:07] Makefile:48: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:1307d309
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Apr 23 06:06:36 UTC 2019
