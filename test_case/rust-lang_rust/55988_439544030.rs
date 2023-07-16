plain
travis_time:end:01b07478:start=1542401858657554307,finish=1542401916210817723,duration=57553263416
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-5.0
---
[00:52:40] .................................................................................................... 100/5021
[00:52:43] .................................................................................................... 200/5021
[00:52:46] .............................ii............................................ii...................ii.. 300/5021
[00:52:48] ..............................................................................................iii... 400/5021
[00:52:51] .....iiiiiiii.iii............................iii...........................................i........ 500/5021
[00:52:58] .................................................................................................... 700/5021
[00:53:05] .................................................................................i...........i...... 800/5021
[00:53:08] .................................................................................................... 900/5021
[00:53:11] .iiiii.................ii.iiii...................................................................... 1000/5021
---
[00:53:48] .................................................................................................... 2200/5021
[00:53:52] .................................................................................................... 2300/5021
[00:53:56] .................................................................................................... 2400/5021
[00:54:00] .................................................................................................... 2500/5021
[00:54:03] ................................................................................iiiiiiiii........... 2600/5021
[00:54:10] ..............................................ii.................................................... 2800/5021
[00:54:13] .................................................................................................... 2900/5021
[00:54:16] .................................................................................................... 3000/5021
[00:54:20] ...........................................i........................................................ 3100/5021
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:16] 
[01:08:16] running 116 tests
[01:08:19] i..ii...iii..iiii.....i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..i 100/116
[01:08:19] i.i....iiii.....
[01:08:19] 
[01:08:19]  finished in 3.567
[01:08:19] travis_fold:end:test_codegen

---
travis_time:start:test_incremental
Check compiletest suite=incremental mode=incremental (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:08:21] 
[01:08:21] running 92 tests
[01:08:34] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:08:34] .................F..........................................................................
[01:08:34] 
[01:08:34] ---- [incremental] incremental/hashes/closure_expressions.rs stdout ----
[01:08:34] 
[01:08:34] 
[01:08:34] error in revision `cfail1`: test compilation failed although it shouldn't!
[01:08:34] status: exit code: 1
[01:08:34] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/incremental/hashes/closure_expressions.rs" "--target=x86_64-unknown-linux-gnu" "--cfg" "cfail1" "-C" "incremental=/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/closure_expressions.inc" "-Z" "incremental-verify-ich" "-Z" "incremental-queries" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-Z" "query-dep-graph" "-Zincremental-ignore-spans" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/incremental/hashes/closure_expressions/auxiliary"
[01:08:34] ------------------------------------------
[01:08:34] 
[01:08:34] ------------------------------------------
[01:08:34] stderr:
[01:08:34] stderr:
[01:08:34] ------------------------------------------
[01:08:34] {"message":"cannot infer an appropriate lifetime due to conflicting requirements","code":{"code":"E0495","explanation":null},"level":"error","spans":[{"file_name":"/checkout/src/test/incremental/hashes/closure_expressions.rs","byte_start":1810,"byte_end":1811,"line_start":63,"line_end":63,"column_start":23,"column_end":24,"is_primary":true,"text":[{"text":"    let _ = |x: &u32| x;","highlight_start":23,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[{"message":"first, the lifetime cannot outlive the anonymous lifetime #1 defined on the body at 63:13...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/incremental/hashes/closure_expressions.rs","byte_start":1800,"byte_end":1811,"line_start":63,"line_end":63,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"    let _ = |x: &u32| x;","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...so that the expression is assignable:\nexpected &u32\n   found &u32","code":null,"level":"note","spans":[],"children":[],"rendered":null},{"message":"but, the lifetime must be valid for the expression at 63:13...","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/incremental/hashes/closure_expressions.rs","byte_start":1800,"byte_end":1811,"line_start":63,"line_end":63,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"    let _ = |x: &u32| x;","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null},{"message":"...so type `[closure@/checkout/src/test/incremental/hashes/closure_expressions.rs:63:13: 63:24]` of expression is valid during the expression","code":null,"level":"note","spans":[{"file_name":"/checkout/src/test/incremental/hashes/closure_expressions.rs","byte_start":1800,"byte_end":1811,"line_start":63,"line_end":63,"column_start":13,"column_end":24,"is_primary":true,"text":[{"text":"    let _ = |x: &u32| x;","highlight_start":13,"highlight_end":24}],"label":null,"suggested_replacement":null,"suggestion_applicability":null,"expansion":null}],"children":[],"rendered":null}],"rendered":"error[E0495]: cannot infer an appropriate lifetime due to conflicting requirements\n  --> /checkout/src/test/incremental/hashes/closure_expressions.rs:63:23\n   |\nLL |     let _ = |x: &u32| x;\n   |                       ^\n   |\nnote: first, the lifetime cannot outlive the anonymous lifetime #1 defined on the body at 63:13...\n  --> /checkout/src/test/incremental/hashes/closure_expressions.rs:63:13\n   |\nLL |     let _ = |x: &u32| x;\n   |             ^^^^^^^^^^^\n   = note: ...so that the expression is assignable:\n           expected &u32\n              found &u32\nnote: but, the lifetime must be valid for the expression at 63:13...\n  --> /checkout/src/test/incremental/hashes/closure_expressions.rs:63:13\n   |\nLL |     let _ = |x: &u32| x;\n   |             ^^^^^^^^^^^\nnote: ...so type `[closure@/checkout/src/test/incremental/hashes/closure_expressions.rs:63:13: 63:24]` of expression is valid during the expression\n  --> /checkout/src/test/incremental/hashes/closure_expressions.rs:63:13\n   |\nLL |     let _ = |x: &u32| x;\n   |             ^^^^^^^^^^^\n\n"}
[01:08:34] {"message":"aborting due to previous error","code":null,"level":"error","spans":[],"children":[],"rendered":"error: aborting due to previous error\n\n"}
[01:08:34] {"message":"For more information about this error, try `rustc --explain E0495`.","code":null,"level":"","spans":[],"children":[],"rendered":"For more information about this error, try `rustc --explain E0495`.\n"}
[01:08:34] ------------------------------------------
[01:08:34] 
[01:08:34] thread '[incremental] incremental/hashes/closure_expressions.rs' panicked at 'explicit panic', tools/compiletest/src/runtest.rs:3282:9
[01:08:34] note: Run with `RUST_BACKTRACE=1` trap/debug/bootstrap test
[01:08:34] note: Run with `RUST_BACKTRACE=1` trap/debug/bootstrap test
[01:08:34] Build completed unsuccessfully in 0:19:45
[01:08:34] make: *** [check] Error 1
[01:08:34] Makefile:58: recipe for target 'check' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:17754cf3
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Nov 16 22:07:20 UTC 2018
