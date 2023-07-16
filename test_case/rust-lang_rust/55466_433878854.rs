plain
travis_time:end:16b8f0e8:start=1540808734905367668,finish=1540808735989862870,duration=1084495202
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#Pull-Requests-and-Security-Restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
---
[00:51:50] .................................................................................................... 2200/4977
[00:51:54] .................................................................................................... 2300/4977
[00:51:58] .................................................................................................... 2400/4977
[00:52:02] .................................................................................................... 2500/4977
[00:52:05] .............................................................iiiiiiiii.............................. 2600/4977
[00:52:12] ............ii...................................................................................... 2800/4977
[00:52:15] .................................................................................................... 2900/4977
[00:52:19] .................................................................................................... 3000/4977
[00:52:22] .......i............................................................................................ 3100/4977
---
travis_time:start:test_codegen
Check compiletest suite=codegen mode=codegen (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:05:35] 
[01:05:35] running 112 tests
[01:05:38] i..ii...iii.......i...i.........i..iii...........i.....i.....ii...i..i.ii..............i...ii..ii.i. 100/112
[01:05:38] test result: ok. 82 passed; 0 failed; 30 ignored; 0 measured; 0 filtered out
[01:05:38] 
[01:05:38]  finished in 3.648
[01:05:38] travis_fold:end:test_codegen
---
travis_time:start:test_run-pass-fulldeps
Check compiletest suite=run-pass-fulldeps mode=run-pass (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:07:00] 
[01:07:00] running 97 tests
[01:08:57] .F.........................................................test [run-pass] run-pass-fulldeps/myriad-closures.rs has been running for over 60 seconds
[01:11:12] thread 'main' panicked at 'Some tests failed', tools/compiletest/src/main.rs:503:22
[01:11:12] failures:
[01:11:12] 
[01:11:12] ---- [run-pass] run-pass-fulldeps/ast_stmt_expr_attr.rs stdout ----
[01:11:12] 
[01:11:12] 
[01:11:12] error: test compilation failed although it shouldn't!
[01:11:12] status: exit code: 1
[01:11:12] command: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc" "/checkout/src/test/run-pass-fulldeps/ast_stmt_expr_attr.rs" "--target=x86_64-unknown-linux-gnu" "--error-format" "json" "-Zui-testing" "-C" "prefer-dynamic" "-o" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/ast_stmt_expr_attr/a" "-Crpath" "-O" "-Zunstable-options" "-Lnative=/checkout/obj/build/x86_64-unknown-linux-gnu/native/rust-test-helpers" "-L" "/checkout/obj/build/x86_64-unknown-linux-gnu/test/run-pass-fulldeps/ast_stmt_expr_attr/auxiliary"
[01:11:12] ------------------------------------------
[01:11:12] 
[01:11:12] ------------------------------------------
[01:11:12] stderr:
[01:11:12] stderr:
[01:11:12] ------------------------------------------
[01:11:12] {"message":"unresolved import `syntax::str`","code":{"code":"E0432","explanation":"\nAn import was unresolved.\n\nErroneous code example:\n\n