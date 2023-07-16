plain
travis_time:end:2de6f12d:start=1547184764427866001,finish=1547184767059270895,duration=2631404894
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:06:08] error: unreachable expression
[00:06:08]    --> src/libarena/lib.rs:214:9
[00:06:08]     |
[00:06:08] 214 | /         unsafe {
[00:06:08] 215 | |             let len = vec.len();
[00:06:08] 216 | |             let start_ptr = self.alloc_raw_slice(len);
[00:06:08] 217 | |             vec.as_ptr().copy_to_nonoverlapping(start_ptr, len);
[00:06:08] 221 | |             slice::from_raw_parts_mut(start_ptr, len)
[00:06:08] 222 | |         }
[00:06:08]     | |_________^
[00:06:08]     |
---
[00:06:31] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[00:06:31] expected success, got: exit code: 101
[00:06:31] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:06:31] Build completed unsuccessfully in 0:02:38
[00:06:31] Makefile:18: recipe for target 'all' failed
[00:06:31] make: *** [all] Error 1
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:001e7892
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Fri Jan 11 05:39:29 UTC 2019
