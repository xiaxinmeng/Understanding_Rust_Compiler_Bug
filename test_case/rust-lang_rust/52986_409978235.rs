plain
[00:02:03] curl: (22) The requested URL returned error: 404 Not Found
[00:02:03] 
[00:02:03] spurious failure, trying again
[00:02:04] curl: (22) The requested URL returned error: 404 Not Found
[00:02:04] failed to run: curl -s --retry 3 -Sf -o /tmp/tmpn0AoTj.sha256 https://static.rust-lang.org/dist/2018-07-31/rust-std-1.28.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:02:04] Makefile:81: recipe for target 'prepare' failed
[00:02:04] make: *** [prepare] Error 1
[00:02:05] Command failed. Attempt 2/5:
[00:02:05] curl: (22) The requested URL returned error: 404 Not Found
---
[00:02:06] curl: (22) The requested URL returned error: 404 Not Found
[00:02:06] 
[00:02:06] spurious failure, trying again
[00:02:06] curl: (22) The requested URL returned error: 404 Not Found
[00:02:06] failed to run: curl -s --retry 3 -Sf -o /tmp/tmp_GORxS.sha256 https://static.rust-lang.org/dist/2018-07-31/rust-std-1.28.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:02:06] Makefile:81: recipe for target 'prepare' failed
[00:02:06] make: *** [prepare] Error 1
[00:02:08] Command failed. Attempt 3/5:
[00:02:08] curl: (22) The requested URL returned error: 404 Not Found
---
[00:02:09] curl: (22) The requested URL returned error: 404 Not Found
[00:02:09] 
[00:02:09] spurious failure, trying again
[00:02:10] curl: (22) The requested URL returned error: 404 Not Found
[00:02:10] failed to run: curl -s --retry 3 -Sf -o /tmp/tmp4bxbVM.sha256 https://static.rust-lang.org/dist/2018-07-31/rust-std-1.28.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:02:10] make: *** [prepare] Error 1
[00:02:10] Makefile:81: recipe for target 'prepare' failed
[00:02:13] Command failed. Attempt 4/5:
[00:02:13] curl: (22) The requested URL returned error: 404 Not Found
---
[00:02:14] curl: (22) The requested URL returned error: 404 Not Found
[00:02:14] 
[00:02:14] spurious failure, trying again
[00:02:14] curl: (22) The requested URL returned error: 404 Not Found
[00:02:14] failed to run: curl -s --retry 3 -Sf -o /tmp/tmppIslLq.sha256 https://static.rust-lang.org/dist/2018-07-31/rust-std-1.28.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:02:14] make: *** [prepare] Error 1
[00:02:14] Makefile:81: recipe for target 'prepare' failed
[00:02:18] Command failed. Attempt 5/5:
[00:02:18] curl: (22) The requested URL returned error: 404 Not Found
---
[00:02:19] curl: (22) The requested URL returned error: 404 Not Found
[00:02:19] 
[00:02:19] spurious failure, trying again
[00:02:19] curl: (22) The requested URL returned error: 404 Not Found
[00:02:19] failed to run: curl -s --retry 3 -Sf -o /tmp/tmp4P27P_.sha256 https://static.rust-lang.org/dist/2018-07-31/rust-std-1.28.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:02:19] Makefile:81: recipe for target 'prepare' failed
[00:02:19] make: *** [prepare] Error 1
[00:02:19] The command has failed after 5 attempts.
travis_time:end:00f5bea8:start=1533225451359544624,finish=1533225591342041495,duration=139982496871
---
travis_time:end:012a7847:start=1533225591912061057,finish=1533225591920565109,duration=8504052
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09958864
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:01ddeb6c
travis_time:start:01ddeb6c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0f748bc0
$ dmesg | grep -i kill
