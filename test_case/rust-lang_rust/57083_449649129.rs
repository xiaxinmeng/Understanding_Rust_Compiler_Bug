plain
[00:04:49] Removing intermediate container a4b5ab65b455
[00:04:49] Step 5/31 : RUN apt-key adv --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
[00:04:49]  ---> Running in 16a7d6c1a87d
[00:04:50] Warning: apt-key output should not be parsed (stdout is not a terminal)
[00:04:50] Executing: /tmp/apt-key-gpghome.gB7PXfYMce/gpg.1.sh --batch --yes --keyserver keyserver.ubuntu.com --recv-keys 74DA7924C5513486
[00:04:52] gpg: Total number processed: 1
[00:04:52] gpg:               imported: 1
[00:04:53]  ---> ed16a194dfc7
[00:04:53] Removing intermediate container 16a7d6c1a87d
---
travis_time:end:3a92f7c2:start=1545583542084928642,finish=1545583542093269878,duration=8341236
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:09dd1a91
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:27125d0f
travis_time:start:27125d0f
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0bcc7d2d
$ dmesg | grep -i kill
