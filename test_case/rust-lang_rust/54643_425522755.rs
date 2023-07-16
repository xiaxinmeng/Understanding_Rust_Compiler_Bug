plain
travis_fold:end:system_info

Network availability confirmed.
Setting APT mirror in /etc/apt/sources.list: http://us-central1.gce.archive.ubuntu.com/ubuntu/
travis_fold:start:update_heroku
Updating Heroku
$ heroku version
heroku/7.16.0 linux-x64 node-v10.10.0
travis_fold:end:update_heroku
Installing APT Packages
travis_time:start:014b1f58
$ travis_apt_get_update
travis_time:end:014b1f58:start=1538157134823975382,finish=1538157139828327103,duration=5004351721
---
travis_time:end:271a0d12:start=1538158549999520696,finish=1538158550002889004,duration=3368308
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:00da5cf8
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:21427638
travis_time:start:21427638
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1941b517
$ dmesg | grep -i kill
