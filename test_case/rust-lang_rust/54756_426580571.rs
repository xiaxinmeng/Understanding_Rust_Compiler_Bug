plain
[00:07:46] 
[00:07:46] error[E0614]: type `syntax_pos::Span` cannot be dereferenced
[00:07:46]     --> librustc/middle/resolve_lifetime.rs:1109:22
[00:07:46]      |
[00:07:46] 1109 |         ).span_label(*explicit_span, "explicit lifetime definition here")
[00:07:46] 
[00:07:54] error: aborting due to 3 previous errors
[00:07:54] 
[00:07:54] For more information about this error, try `rustc --explain E0614`.
[00:07:54] For more information about this error, try `rustc --explain E0614`.
[00:07:54] error: Could not compile `rustc`.
[00:07:54] 
[00:07:54] To learn more, run the command again with --verbose.
[00:07:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:07:54] expected success, got: exit code: 101
[00:07:54] thread 'main' panicked at 'cargo must succeed', bootstrap/compile.rs:1112:9
[00:07:54] travis_fold:end:stage0-rustc

[00:07:54] travis_time:end:stage0-rustc:start=1538561089982951430,finish=1538561248173324841,duration=158190373411


[00:07:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:07:54] Build completed unsuccessfully in 0:03:31
[00:07:54] Makefile:28: recipe for target 'all' failed
[00:07:54] make: *** [all] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0b440822
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
