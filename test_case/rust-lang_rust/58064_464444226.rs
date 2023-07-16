plain
travis_time:end:2552edef:start=1550402196864076492,finish=1550402267931572170,duration=71067495678
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:46]    Compiling rustc-demangle v0.1.10
[00:03:46] error: expected one of `.`, `;`, `?`, or an operator, found `let`
[00:03:46]     --> src/liballoc/collections/vec_deque.rs:2187:13
[00:03:46]      |
[00:03:46] 2186 |             let (front, back) = self.ring.split_at(self.tail)
[00:03:46]      |                                                              - expected one of `.`, `;`, `?`, or an operator here
[00:03:46] 2187 |             let mut back_iter = back.iter();
[00:03:46] 
[00:03:46] error: expected one of `.`, `;`, `?`, or an operator, found `let`
[00:03:46]     --> src/liballoc/collections/vec_deque.rs:2231:13
[00:03:46]      |
[00:03:46]      |
[00:03:46] 2230 |             let (front, back) = self.ring.split_at(self.tail)
[00:03:46]      |                                                              - expected one of `.`, `;`, `?`, or an operator here
[00:03:46] 2231 |             let mut front_iter = front[..self.head].iter();
[00:03:46] 
[00:03:46]    Compiling panic_abort v0.0.0 (/checkout/src/libpanic_abort)
[00:03:49] error: aborting due to 2 previous errors
[00:03:49] 
[00:03:49] 
[00:03:49] error: Could not compile `alloc`.
[00:03:49] 
[00:03:49] To learn more, run the command again with --verbose.
[00:03:49] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:49] expected success, got: exit code: 101
[00:03:49] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:49] Build completed unsuccessfully in 0:00:38
[00:03:49] make: *** [all] Error 1
[00:03:49] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:0062b25d
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Sun Feb 17 11:21:47 UTC 2019
