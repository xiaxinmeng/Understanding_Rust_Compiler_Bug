plain
[00:02:25] Removing intermediate container b9872184686c
[00:02:25] Step 5/27 : RUN apt-key adv --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
[00:02:25]  ---> Running in 4ed48fc81ddc
[00:02:25] Warning: apt-key output should not be parsed (stdout is not a terminal)
[00:02:26] Executing: /tmp/apt-key-gpghome.NbDMdPQ4fx/gpg.1.sh --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
[00:02:28] gpg: Total number processed: 1
[00:02:28] gpg:               imported: 1
[00:02:28]  ---> f2a1d559c2c2
[00:02:28] Removing intermediate container 4ed48fc81ddc
---
travis_time:end:075ee977:start=1540614972451610527,finish=1540614972461279292,duration=9668765
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1d21293c
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:00594d36
travis_time:start:00594d36
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:19480533
$ dmesg | grep -i kill
