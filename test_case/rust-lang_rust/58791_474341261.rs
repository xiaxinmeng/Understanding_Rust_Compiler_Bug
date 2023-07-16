plain
[00:02:26] Questions should be asked at https://answers.launchpad.net/gcc-arm-embedded
[00:02:26] 
[00:02:26] Bug can be filed at https://bugs.launchpad.net/gcc-arm-embedded/+filebug. It is highly encouraged to ask question first before filing a bug.
[00:02:26]  More info: https://launchpad.net/~team-gcc-arm-embedded/+archive/ubuntu/ppa
[00:02:26] gpg: keyring `/tmp/tmpz71y8vzt/secring.gpg' created
[00:02:26] gpg: keyring `/tmp/tmpz71y8vzt/pubring.gpg' created
[00:02:26] gpg: requesting key F64D33B0 from hkp server keyserver.ubuntu.com
[00:02:27] gpg: /tmp/tmpz71y8vzt/trustdb.gpg: trustdb created
[00:02:27] gpg: no ultimately trusted keys found
[00:02:27] gpg: Total number processed: 1
[00:02:27] gpg:               imported: 1  (RSA: 1)
[00:02:27] OK
---
travis_time:end:02590ac0:start=1552998402468279247,finish=1552998402478208338,duration=9929091
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:2798a654
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:08e5b8e0
travis_time:start:08e5b8e0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:0ff062a4
$ dmesg | grep -i kill
