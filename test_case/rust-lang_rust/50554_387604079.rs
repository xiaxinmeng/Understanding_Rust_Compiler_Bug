plain
[01:11:39]    Compiling core v0.0.0 (file:///checkout/src/libcore)
[01:11:53] error[E0282]: type annotations needed
[01:11:53]    --> libcore/../libcore/tests/result.rs:223:19
[01:11:53]     |
[01:11:53] 223 |         let val = Ok(1)?;
[01:11:53]     |                   ^^^^^^ cannot infer type for `_`
[01:11:54] error: aborting due to previous error
[01:11:54] 
[01:11:54] For more information about this error, try `rustc --explain E0282`.
[01:11:54] error: Could not compile `core`.
[01:11:54] error: Could not compile `core`.
[01:11:54] 
[01:11:54] To learn more, run the command again with --verbose.
[01:11:54] 
[01:11:54] 
[01:11:54] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "test" "--target" "x86_64-unknown-linux-gnu" "--release" "--locked" "--color" "always" "--features" "panic-unwind jemalloc backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "-p" "core" "--" "--quiet"
[01:11:54] 
[01:11:54] 
[01:11:54] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test
[01:11:54] Build completed unsuccessfully in 0:28:38
[01:11:54] Build completed unsuccessfully in 0:28:38
[01:11:54] Makefile:58: recipe for target 'check' failed
[01:11:54] make: *** [check] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0ebdfa38
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
