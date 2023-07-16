plain
[00:06:17] curl: (22) The requested URL returned error: 404 Not Found
[00:06:17] 
[00:06:17] spurious failure, trying again
[00:06:17] curl: (22) The requested URL returned error: 404 Not Found
[00:06:17] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpMJENtS.sha256 https://static.rust-lang.org/dist/2019-04-08/rust-std-1.34.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:06:17] make: *** [prepare] Error 1
[00:06:17] Makefile:69: recipe for target 'prepare' failed
[00:06:18] Command failed. Attempt 2/5:
[00:06:18] curl: (22) The requested URL returned error: 404 Not Found
---
[00:06:19] curl: (22) The requested URL returned error: 404 Not Found
[00:06:19] 
[00:06:19] spurious failure, trying again
[00:06:19] curl: (22) The requested URL returned error: 404 Not Found
[00:06:19] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpT5_BSc.sha256 https://static.rust-lang.org/dist/2019-04-08/rust-std-1.34.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:06:19] make: *** [prepare] Error 1
[00:06:19] Makefile:69: recipe for target 'prepare' failed
[00:06:21] Command failed. Attempt 3/5:
[00:06:22] curl: (22) The requested URL returned error: 404 Not Found
---
[00:06:22] curl: (22) The requested URL returned error: 404 Not Found
[00:06:22] 
[00:06:22] spurious failure, trying again
[00:06:23] curl: (22) The requested URL returned error: 404 Not Found
[00:06:23] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpjZXOD_.sha256 https://static.rust-lang.org/dist/2019-04-08/rust-std-1.34.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:06:23] make: *** [prepare] Error 1
[00:06:23] Makefile:69: recipe for target 'prepare' failed
[00:06:26] Command failed. Attempt 4/5:
[00:06:26] curl: (22) The requested URL returned error: 404 Not Found
---
[00:06:26] curl: (22) The requested URL returned error: 404 Not Found
[00:06:26] 
[00:06:26] spurious failure, trying again
[00:06:26] curl: (22) The requested URL returned error: 404 Not Found
[00:06:26] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmp4kPptN.sha256 https://static.rust-lang.org/dist/2019-04-08/rust-std-1.34.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:06:26] make: *** [prepare] Error 1
[00:06:26] Makefile:69: recipe for target 'prepare' failed
[00:06:30] Command failed. Attempt 5/5:
[00:06:31] curl: (22) The requested URL returned error: 404 Not Found
---
[00:06:31] curl: (22) The requested URL returned error: 404 Not Found
[00:06:31] 
[00:06:31] spurious failure, trying again
[00:06:31] curl: (22) The requested URL returned error: 404 Not Found
[00:06:31] failed to run: curl -s -y 30 -Y 10 --connect-timeout 30 --retry 3 -Sf -o /tmp/tmpAwQaXJ.sha256 https://static.rust-lang.org/dist/2019-04-08/rust-std-1.34.0-x86_64-unknown-linux-gnu.tar.gz.sha256
[00:06:31] make: *** [prepare] Error 1
[00:06:31] Makefile:69: recipe for target 'prepare' failed
[00:06:31] The command has failed after 5 attempts.
travis_time:end:024fd660:start=1557964205440331231,finish=1557964597530787535,duration=392090456304
---
travis_time:end:0de1b1c8:start=1557964598512412440,finish=1557964598519530281,duration=7117841
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0d7df8ba
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0b3493eb
travis_time:start:0b3493eb
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00a766e2
$ dmesg | grep -i kill
