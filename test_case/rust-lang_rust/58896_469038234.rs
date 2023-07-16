plain
travis_time:end:29e504c2:start=1551629806245673383,finish=1551629879696548710,duration=73450875327
$ git checkout -qf FETCH_HEAD
travis_fold:end:git.checkout

Encrypted environment variables have been removed for security reasons.
See https://docs.travis-ci.com/user/pull-requests/#pull-requests-and-security-restrictions
$ export SCCACHE_BUCKET=rust-lang-ci-sccache2
$ export SCCACHE_REGION=us-west-1
Setting environment variables from .travis.yml
$ export IMAGE=x86_64-gnu-llvm-6.0
---
[00:02:40] curl: (22) The requested URL returned error: 404 Not Found
[00:02:40] 
[00:02:40] spurious failure, trying again
[00:02:40] curl: (22) The requested URL returned error: 404 Not Found
[00:02:40] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmp994C7I.sha256 https://static.rust-lang.org/dist/2019-02-27/rust-std-1.33.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:02:40] Makefile:70: recipe for target 'prepare' failed
[00:02:40] make: *** [prepare] Error 1
[00:02:41] Command failed. Attempt 2/5:
[00:02:41] curl: (22) The requested URL returned error: 404 Not Found
---
[00:02:42] curl: (22) The requested URL returned error: 404 Not Found
[00:02:42] 
[00:02:42] spurious failure, trying again
[00:02:42] curl: (22) The requested URL returned error: 404 Not Found
[00:02:42] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpD8OAQe.sha256 https://static.rust-lang.org/dist/2019-02-27/rust-std-1.33.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:02:42] make: *** [prepare] Error 1
[00:02:42] Makefile:70: recipe for target 'prepare' failed
[00:02:44] Command failed. Attempt 3/5:
[00:02:44] curl: (22) The requested URL returned error: 404 Not Found
---
[00:02:45] curl: (22) The requested URL returned error: 404 Not Found
[00:02:45] 
[00:02:45] spurious failure, trying again
[00:02:45] curl: (22) The requested URL returned error: 404 Not Found
[00:02:45] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmp9M8HSm.sha256 https://static.rust-lang.org/dist/2019-02-27/rust-std-1.33.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:02:45] Makefile:70: recipe for target 'prepare' failed
[00:02:45] make: *** [prepare] Error 1
[00:02:48] Command failed. Attempt 4/5:
[00:02:48] curl: (22) The requested URL returned error: 404 Not Found
---
[00:02:49] curl: (22) The requested URL returned error: 404 Not Found
[00:02:49] 
[00:02:49] spurious failure, trying again
[00:02:49] curl: (22) The requested URL returned error: 404 Not Found
[00:02:49] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmp85obX9.sha256 https://static.rust-lang.org/dist/2019-02-27/rust-std-1.33.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:02:49] Makefile:70: recipe for target 'prepare' failed
[00:02:49] make: *** [prepare] Error 1
[00:02:53] Command failed. Attempt 5/5:
[00:02:53] curl: (22) The requested URL returned error: 404 Not Found
---
[00:02:54] curl: (22) The requested URL returned error: 404 Not Found
[00:02:54] 
[00:02:54] spurious failure, trying again
[00:02:54] curl: (22) The requested URL returned error: 404 Not Found
[00:02:54] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpT3XFQ6.sha256 https://static.rust-lang.org/dist/2019-02-27/rust-std-1.33.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:02:54] make: *** [prepare] Error 1
[00:02:54] Makefile:70: recipe for target 'prepare' failed
[00:02:54] The command has failed after 5 attempts.
travis_time:end:280fbba6:start=1551629887968608457,finish=1551630062641375593,duration=174672767136
---
travis_time:end:2ed4d7e0:start=1551630063401369796,finish=1551630063406609370,duration=5239574
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0024f680
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1f7b9584
travis_time:start:1f7b9584
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:05432fb0
$ dmesg | grep -i kill
