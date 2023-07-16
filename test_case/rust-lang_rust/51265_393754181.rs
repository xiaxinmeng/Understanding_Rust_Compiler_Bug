plain
[00:51:54] ..........................................................................i.........................
[00:51:59] ....................................................................................................
[00:52:05] ....................................................................................................
[00:52:11] ....................................................................................................
[00:52:16] ......i.................iiiiiiiii...................................................
[00:52:16] 
[00:52:16] travis_fold:start:test_ui_nll
travis_time:start:test_ui_nll
Check compiletest suite=ui mode=ui compare_mode=nll (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
---
[00:53:08] ..........................................................................i.........................
[00:53:13] ....................................................................................................
[00:53:18] ....................................................................................................
[00:53:23] ....................................................................................................
[00:53:28] ......i.................iiiiiiiii...................................................
[00:53:28] 
[00:53:28]  finished in 72.166
[00:53:28] travis_fold:end:test_ui_nll

---
[01:32:35] travis_fold:start:test_stage1-syntax
travis_time:start:test_stage1-syntax
Testing syntax stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:32:35]    Compiling syntax v0.0.0 (file:///checkout/src/libsyntax)
[01:32:36] error[E0603]: function `is_pattern_whitespace` is private
[01:32:36]    --> libsyntax/util/parser_testing.rs:131:11
[01:32:36]     |
[01:32:36] 131 |     while lexer::is_pattern_whitespace(iter.peek().cloned()) {
[01:32:36] 
[01:32:36] 
[01:32:36] error[E0603]: function `is_pattern_whitespace` is private
[01:32:36]    --> libsyntax/util/parser_testing.rs:137:5
[01:32:36]     |
[01:32:36] 137 |     lexer::is_pattern_whitespace(Some(c))
[01:32:36] 
[01:32:48] error: aborting due to 2 previous errors
[01:32:48] 
[01:32:48] For more information about this error, try `rustc --explain E0603`.
[01:32:48] For more information about this error, try `rustc --explain E0603`.
[01:32:48] error: Could not compile `syntax`.
[01:32:48] 
[01:32:48] To learn more, run the command again with --verbose.
[01:32:48] 
[01:32:48] 
[01:32:48] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "-p" "syntax" "--" "--quiet"
[01:32:48] 
[01:32:48] 
[01:32:48] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:32:48] Build completed unsuccessfully in 0:43:27
[01:32:48] Build completed unsuccessfully in 0:43:27
[01:32:48] Makefile:58: recipe for target 'check' failed
[01:32:48] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0e8eff70
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
