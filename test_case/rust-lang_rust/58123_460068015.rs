plain
travis_time:end:15a31ec2:start=1549212458035764223,finish=1549212458959316648,duration=923552425
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:04:38]    Compiling rustc-demangle v0.1.10
[00:04:42] error[E0133]: call to unsafe function is unsafe and requires unsafe function or block
[00:04:42]    --> src/liballoc/collections/binary_heap.rs:262:11
[00:04:42]     |
[00:04:42] 262 |         { self.heap.data.get_unchecked_mut(0) }
[00:04:42]     |           ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^ call to unsafe function
[00:04:42]     |
[00:04:42]     = note: consult the function's documentation for information on how to avoid undefined behavior
[00:04:42] error: aborting due to previous error
[00:04:42] 
[00:04:42] For more information about this error, try `rustc --explain E0133`.
[00:04:42] error: Could not compile `alloc`.
[00:04:42] error: Could not compile `alloc`.
[00:04:42] 
[00:04:42] To learn more, run the command again with --verbose.
[00:04:42] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:04:42] expected success, got: exit code: 101
[00:04:42] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:04:42] Build completed unsuccessfully in 0:00:42
[00:04:42] Makefile:18: recipe for target 'all' failed
[00:04:42] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0f08d30b
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb  3 16:52:32 UTC 2019
