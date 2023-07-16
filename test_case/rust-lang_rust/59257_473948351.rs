plain
[00:03:55] Questions should be asked at https://answers.launchpad.net/gcc-arm-embedded
[00:03:55] 
[00:03:55] Bug can be filed at https://bugs.launchpad.net/gcc-arm-embedded/+filebug. It is highly encouraged to ask question first before filing a bug.
[00:03:55]  More info: https://launchpad.net/~team-gcc-arm-embedded/+archive/ubuntu/ppa
[00:03:55] gpg: keyring `/tmp/tmpqi4j9i0n/secring.gpg' created
[00:03:55] gpg: keyring `/tmp/tmpqi4j9i0n/pubring.gpg' created
[00:03:55] gpg: requesting key F64D33B0 from hkp server keyserver.ubuntu.com
[00:03:56] gpg: /tmp/tmpqi4j9i0n/trustdb.gpg: trustdb created
[00:03:56] gpg: no ultimately trusted keys found
[00:03:56] gpg: Total number processed: 1
[00:03:56] gpg:               imported: 1  (RSA: 1)
[00:03:56] OK
---
[00:14:46] Step 7/52 : COPY dist-various-1/install-x86_64-redox.sh /build
[00:14:46]  ---> 03e5b6f01e44
[00:14:46] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:14:46]  ---> Running in 0fc3f63efca3
[00:14:46] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:14:46] + tar --extract --gzip --directory /usr/local
[00:14:46] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:14:46] gzip: stdin: unexpected end of file
[00:14:46] tar: Child returned status 1
[00:14:46] tar: Error is not recoverable: exiting now
[00:14:47] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:14:48]  ---> Using cache
[00:14:48]  ---> 03e5b6f01e44
[00:14:48] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:14:48]  ---> Running in ad5191b1b18e
[00:14:48] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:14:48] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:14:48] + tar --extract --gzip --directory /usr/local
[00:14:48] gzip: stdin: unexpected end of file
[00:14:48] tar: Child returned status 1
[00:14:48] tar: Error is not recoverable: exiting now
[00:14:48] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:14:51]  ---> Using cache
[00:14:51]  ---> 03e5b6f01e44
[00:14:51] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:14:51]  ---> Running in 1252197825d2
[00:14:51] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:14:51] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:14:51] + tar --extract --gzip --directory /usr/local
[00:14:51] gzip: stdin: unexpected end of file
[00:14:51] tar: Child returned status 1
[00:14:51] tar: Error is not recoverable: exiting now
[00:14:51] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:14:55]  ---> Using cache
[00:14:55]  ---> 03e5b6f01e44
[00:14:55] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:14:55]  ---> Running in 7f53e310f5e9
[00:14:55] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:14:55] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:14:55] + tar --extract --gzip --directory /usr/local
[00:14:55] gzip: stdin: unexpected end of file
[00:14:55] tar: Child returned status 1
[00:14:55] tar: Error is not recoverable: exiting now
[00:14:55] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:14:59]  ---> Using cache
[00:14:59]  ---> 03e5b6f01e44
[00:14:59] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:15:00]  ---> Running in 10ef79472919
[00:15:00] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:15:00] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:15:00] + tar --extract --gzip --directory /usr/local
[00:15:00] gzip: stdin: unexpected end of file
[00:15:00] tar: Child returned status 1
[00:15:00] tar: Error is not recoverable: exiting now
[00:15:00] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
travis_time:end:24610a42:start=1552921331327448396,finish=1552921331333126091,duration=5677695
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:23274fca
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:25793f8c
travis_time:start:25793f8c
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:01fee961
$ dmesg | grep -i kill
