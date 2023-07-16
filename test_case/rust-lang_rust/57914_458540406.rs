plain
[00:02:24]  ---> 51da13068bbc
[00:02:24] Step 5/35 : RUN apt-key adv --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
[00:02:24]  ---> Running in fb39503ebd30
[00:02:25] Warning: apt-key output should not be parsed (stdout is not a terminal)
[00:02:25] Executing: /tmp/apt-key-gpghome.VIFxyrk2HM/gpg.1.sh --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
[00:02:27] gpg: Total number processed: 1
[00:02:27] gpg:               imported: 1
[00:02:28] Removing intermediate container fb39503ebd30
[00:02:28]  ---> 2bfdd7d1926b
---
travis_time:end:0f7d3640:start=1548768709540306357,finish=1548768709547386648,duration=7080291
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1b26a900
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:03a538af
travis_time:start:03a538af
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0018e37c
$ dmesg | grep -i kill
