plain
ruby 2.4.1p111 (2017-03-22 revision 58053) [x86_64-linux]
travis_fold:end:system_info

Network availability confirmed.
Setting APT mirror in /etc/apt/sources.list: http://us-east-1.ec2.archive.ubuntu.com/ubuntu/
Installing APT Packages
travis_time:start:14d5591c
$ travis_apt_get_update
travis_time:end:14d5591c:start=1539096993510226658,finish=1539097001420973563,duration=7910746905
---
  gdb
0 upgraded, 1 newly installed, 0 to remove and 190 not upgraded.
Need to get 2,199 kB of archives.
After this operation, 6,474 kB of additional disk space will be used.
Get:1 http://us-east-1.ec2.archive.ubuntu.com/ubuntu trusty-updates/main amd64 gdb amd64 7.7.1-0ubuntu5~14.04.3 [2,199 kB]
Selecting previously unselected package gdb.
(Reading database ... 
(Reading database ... 5%
(Reading database ... 10%
---

[00:04:46] travis_fold:start:tidy
travis_time:start:tidy
tidy check
[00:04:46] tidy error: /checkout/src/libsyntax/attr/builtin.rs:393: line longer than 100 chars
[00:04:47] some tidy checks failed
[00:04:47] 
[00:04:47] 
[00:04:47] command did not execute successfully: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools-bin/tidy" "/checkout/src" "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "--no-vendor" "--quiet"
[00:04:47] 
[00:04:47] 
[00:04:47] failed to run: /checkout/obj/build/bootstrap/debug/bootstrap test src/tools/tidy
[00:04:47] Build completed unsuccessfully in 0:00:51
[00:04:47] Build completed unsuccessfully in 0:00:51
[00:04:47] Makefile:79: recipe for target 'tidy' failed
[00:04:47] make: *** [tidy] Error 1

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 2.
travis_time:start:203dd2fc
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:03719bf4:start=1539097314389436580,finish=1539097314393175377,duration=3738797
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0cdaac30
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:06981cde
travis_time:start:06981cde
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:19418300
$ dmesg | grep -i kill
