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
travis_time:start:0a06fdd8
$ travis_apt_get_update
travis_time:end:0a06fdd8:start=1538159523378727802,finish=1538159528333874680,duration=4955146878
---
travis_time:end:354e0a50:start=1538160937446623206,finish=1538160937451140669,duration=4517463
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0e80f7ce
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0dae1878
travis_time:start:0dae1878
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:2443d70d
$ dmesg | grep -i kill
