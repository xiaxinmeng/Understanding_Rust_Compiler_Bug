plain
[00:12:23] curl: (6) Could not resolve host: static.rust-lang.org
[00:12:23] 
[00:12:23] spurious failure, trying again
[00:13:52] curl: (6) Could not resolve host: static.rust-lang.org
[00:13:52] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpx1SQbS.sha256 https://static.rust-lang.org/dist/2018-10-30/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:13:52] Makefile:81: recipe for target 'prepare' failed
[00:13:52] make: *** [prepare] Error 1
[00:13:53] Command failed. Attempt 2/5:
[00:15:22] curl: (6) Could not resolve host: static.rust-lang.org
---
[00:19:49] curl: (6) Could not resolve host: static.rust-lang.org
[00:19:49] 
[00:19:49] spurious failure, trying again
[00:21:18] curl: (6) Could not resolve host: static.rust-lang.org
[00:21:18] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmp4wnb8c.sha256 https://static.rust-lang.org/dist/2018-10-30/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:21:18] make: *** [prepare] Error 1
[00:21:18] Makefile:81: recipe for target 'prepare' failed
[00:21:20] Command failed. Attempt 3/5:
[00:22:50] curl: (6) Could not resolve host: static.rust-lang.org
---
[00:27:17] curl: (6) Could not resolve host: static.rust-lang.org
[00:27:17] 
[00:27:17] spurious failure, trying again
[00:28:46] curl: (6) Could not resolve host: static.rust-lang.org
[00:28:46] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmp0euVYZ.sha256 https://static.rust-lang.org/dist/2018-10-30/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:28:46] make: *** [prepare] Error 1
[00:28:46] Makefile:81: recipe for target 'prepare' failed
[00:28:49] Command failed. Attempt 4/5:
[00:30:18] curl: (6) Could not resolve host: static.rust-lang.org
---
[00:34:46] curl: (6) Could not resolve host: static.rust-lang.org
[00:34:46] 
[00:34:46] spurious failure, trying again
[00:36:15] curl: (6) Could not resolve host: static.rust-lang.org
[00:36:15] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpGuqCEl.sha256 https://static.rust-lang.org/dist/2018-10-30/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:36:15] make: *** [prepare] Error 1
[00:36:15] Makefile:81: recipe for target 'prepare' failed
[00:36:19] Command failed. Attempt 5/5:
[00:37:48] curl: (6) Could not resolve host: static.rust-lang.org
---
[00:42:15] curl: (6) Could not resolve host: static.rust-lang.org
[00:42:15] 
[00:42:15] spurious failure, trying again
[00:43:45] curl: (6) Could not resolve host: static.rust-lang.org
[00:43:45] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpoVryFu.sha256 https://static.rust-lang.org/dist/2018-10-30/rust-std-beta-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:43:45] Makefile:81: recipe for target 'prepare' failed
[00:43:45] make: *** [prepare] Error 1
[00:43:45] The command has failed after 5 attempts.
travis_time:end:08db2dd8:start=1543818272790084160,finish=1543820898121292130,duration=2625331207970
---
travis_time:end:0ccdfb80:start=1543820899189282557,finish=1543820899197623496,duration=8340939
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1ee31c9a
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0aef848b
travis_time:start:0aef848b
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:02bf1976
$ dmesg | grep -i kill
