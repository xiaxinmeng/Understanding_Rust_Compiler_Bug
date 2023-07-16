plain
travis_time:end:006642a6:start=1546315616515119080,finish=1546315617518097538,duration=1002978458
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:03:16]    Compiling rustc_lsan v0.0.0 (/checkout/src/librustc_lsan)
[00:03:17]    Compiling rustc_msan v0.0.0 (/checkout/src/librustc_msan)
[00:03:17]    Compiling rustc_asan v0.0.0 (/checkout/src/librustc_asan)
[00:03:17]    Compiling rustc_tsan v0.0.0 (/checkout/src/librustc_tsan)
[00:03:19] error[E0276]: impl has stricter requirements than trait
[00:03:19]      |
[00:03:19]      |
[00:03:19] 458  | /     fn rposition<P>(&mut self, predicate: P) -> Option<usize> where
[00:03:19] 459  | |         P: FnMut(<I as Iterator>::Item) -> bool
[00:03:19] 460  | |     {
[00:03:19] 461  | |         self.iter.position(predicate)
[00:03:19] 462  | |     }
[00:03:19]      | |_____^ impl has extra requirement `P: ops::function::FnMut<(<I as iter::iterator::Iterator>::Item,)>`
[00:03:19]     ::: src/libcore/iter/iterator.rs:1978:5
[00:03:19]      |
[00:03:19]      |
[00:03:19] 1978 | /     fn rposition<P>(&mut self, mut predicate: P) -> Option<usize> where
[00:03:19] 1979 | |         P: FnMut(Self::Item) -> bool,
[00:03:19] 1980 | |         Self: Sized + ExactSizeIterator + DoubleEndedIterator
[00:03:19] ...    |
[00:03:19] ...    |
[00:03:19] 1989 | |         }).break_value()
[00:03:19] 1990 | |     }
[00:03:19]      | |_____- definition of `rposition` from trait
[00:03:19] error: aborting due to previous error
[00:03:19] 
[00:03:19] For more information about this error, try `rustc --explain E0276`.
[00:03:19] error: Could not compile `core`.
[00:03:19] error: Could not compile `core`.
[00:03:19] warning: build failed, waiting for other jobs to finish...
[00:03:21] error: build failed
[00:03:21] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" "panic-unwind backtrace" "--manifest-path" "/checkout/src/libstd/Cargo.toml" "--message-format" "json"
[00:03:21] expected success, got: exit code: 101
[00:03:21] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap build
[00:03:21] Build completed unsuccessfully in 0:00:15
[00:03:21] make: *** [all] Error 1
[00:03:21] Makefile:18: recipe for target 'all' failed
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:2ae5a54a
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
Tue Jan  1 04:10:27 UTC 2019
---
travis_time:end:02638dbe:start=1546315827766340379,finish=1546315827771644586,duration=5304207
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00963399
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1d965cc8
travis_time:start:1d965cc8
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02ee4b99
$ dmesg | grep -i kill
