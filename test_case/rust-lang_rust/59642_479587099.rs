plain
[00:04:19]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:20] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   162  100   162    0     0    261      0 --:--:-- --:--:-- --:--:--   261
[00:04:20] xz: (stdin): File format not recognized
[00:04:20] tar: Error is not recoverable: exiting now
[00:04:21] The command '/bin/sh -c /tmp/freebsd-toolchain.sh i686' returned a non-zero code: 2
[00:04:22] Command failed. Attempt 2/5:
[00:04:22] Sending build context to Docker daemon  504.8kB
---
[00:05:40]                                  Dload  Upload   Total   Spent    Left  Speed
[00:05:40] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   162  100   162    0     0   1285      0 --:--:-- --:--:-- --:--:--  1285
[00:05:40] xz: (stdin): File format not recognized
[00:05:40] tar: Error is not recoverable: exiting now
[00:05:40] The command '/bin/sh -c /tmp/freebsd-toolchain.sh i686' returned a non-zero code: 2
[00:05:42] Command failed. Attempt 3/5:
[00:05:43] Sending build context to Docker daemon  504.8kB
---
[00:07:02] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   162  100   162    0     0    259      0 --:--:-- --:--:-- --:--:--   258
[00:07:02] xz: (stdin): File format not recognized
[00:07:02] tar: Error is not recoverable: exiting now
[00:07:02] The command '/bin/sh -c /tmp/freebsd-toolchain.sh i686' returned a non-zero code: 2
[00:07:05] Command failed. Attempt 4/5:
[00:07:06] Sending build context to Docker daemon  504.8kB
---
[00:08:25]                                  Dload  Upload   Total   Spent    Left  Speed
[00:08:25] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   162  100   162    0     0   1246      0 --:--:-- --:--:-- --:--:--  1246
[00:08:25] xz: (stdin): File format not recognized
[00:08:25] tar: Error is not recoverable: exiting now
[00:08:26] The command '/bin/sh -c /tmp/freebsd-toolchain.sh i686' returned a non-zero code: 2
[00:08:30] Command failed. Attempt 5/5:
[00:08:30] Sending build context to Docker daemon  504.8kB
---
[00:09:48]                                  Dload  Upload   Total   Spent    Left  Speed
[00:09:49] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   162  100   162    0     0    925      0 --:--:-- --:--:-- --:--:--   925
[00:09:49] xz: (stdin): File format not recognized
[00:09:49] tar: Error is not recoverable: exiting now
[00:09:49] The command '/bin/sh -c /tmp/freebsd-toolchain.sh i686' returned a non-zero code: 2
[00:09:49] The command has failed after 5 attempts.
travis_time:end:21d1f5b0:start=1554312307079683909,finish=1554312896806699246,duration=589727015337
---
travis_time:end:18af502c:start=1554312897967478491,finish=1554312897974419545,duration=6941054
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:08c581b0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:163ed7b2
travis_time:start:163ed7b2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0b00e2d6
$ dmesg | grep -i kill
