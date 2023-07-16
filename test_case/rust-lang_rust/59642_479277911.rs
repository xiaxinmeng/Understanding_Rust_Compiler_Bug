plain
[00:03:29] Questions should be asked at https://answers.launchpad.net/gcc-arm-embedded
[00:03:29] 
[00:03:29] Bug can be filed at https://bugs.launchpad.net/gcc-arm-embedded/+filebug. It is highly encouraged to ask question first before filing a bug.
[00:03:29]  More info: https://launchpad.net/~team-gcc-arm-embedded/+archive/ubuntu/ppa
[00:03:30] gpg: keyring `/tmp/tmpyxxphw6m/secring.gpg' created
[00:03:30] gpg: keyring `/tmp/tmpyxxphw6m/pubring.gpg' created
[00:03:30] gpg: requesting key F64D33B0 from hkp server keyserver.ubuntu.com
[00:03:30] gpg: /tmp/tmpyxxphw6m/trustdb.gpg: trustdb created
[00:03:30] gpg: no ultimately trusted keys found
[00:03:30] gpg: Total number processed: 1
[00:03:30] gpg:               imported: 1  (RSA: 1)
[00:03:30] OK
---
[00:09:13] Step 7/52 : COPY dist-various-1/install-x86_64-redox.sh /build
[00:09:13]  ---> 64988100452e
[00:09:13] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:09:13]  ---> Running in 7caf3a174b3f
[00:09:14] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:09:14] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:09:14] + tar --extract --gzip --directory /usr/local
[00:09:14] gzip: stdin: unexpected end of file
[00:09:14] tar: Child returned status 1
[00:09:14] tar: Error is not recoverable: exiting now
[00:09:14] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:09:15]  ---> Using cache
[00:09:15]  ---> 64988100452e
[00:09:15] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:09:15]  ---> Running in a11ff6bd0064
[00:09:16] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:09:16] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:09:16] + tar --extract --gzip --directory /usr/local
[00:09:16] gzip: stdin: unexpected end of file
[00:09:16] tar: Child returned status 1
[00:09:16] tar: Error is not recoverable: exiting now
[00:09:16] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:09:18]  ---> Using cache
[00:09:18]  ---> 64988100452e
[00:09:18] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:09:18]  ---> Running in 53aff84e2819
[00:09:18] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:09:18] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:09:18] + tar --extract --gzip --directory /usr/local
[00:09:18] gzip: stdin: unexpected end of file
[00:09:18] tar: Child returned status 1
[00:09:18] tar: Error is not recoverable: exiting now
[00:09:19] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:09:22]  ---> Using cache
[00:09:22]  ---> 64988100452e
[00:09:22] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:09:22]  ---> Running in 009af8d64262
[00:09:22] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:09:22] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:09:22] + tar --extract --gzip --directory /usr/local
[00:09:22] gzip: stdin: unexpected end of file
[00:09:22] tar: Child returned status 1
[00:09:22] tar: Error is not recoverable: exiting now
[00:09:23] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:09:27]  ---> Using cache
[00:09:27]  ---> 64988100452e
[00:09:27] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:09:27]  ---> Running in 25117cf7725b
[00:09:27] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:09:27] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:09:27] + tar --extract --gzip --directory /usr/local
[00:09:27] gzip: stdin: unexpected end of file
[00:09:27] tar: Child returned status 1
[00:09:27] tar: Error is not recoverable: exiting now
[00:09:28] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
travis_time:end:2623a4bc:start=1554252884802889357,finish=1554252884808625428,duration=5736071
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0923fdac
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0d34cca2
travis_time:start:0d34cca2
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:1a9f8aac
$ dmesg | grep -i kill
