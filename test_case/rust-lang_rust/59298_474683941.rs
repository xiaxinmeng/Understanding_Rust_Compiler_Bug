plain
[00:02:26] Questions should be asked at https://answers.launchpad.net/gcc-arm-embedded
[00:02:26] 
[00:02:26] Bug can be filed at https://bugs.launchpad.net/gcc-arm-embedded/+filebug. It is highly encouraged to ask question first before filing a bug.
[00:02:26]  More info: https://launchpad.net/~team-gcc-arm-embedded/+archive/ubuntu/ppa
[00:02:26] gpg: keyring `/tmp/tmpgahunmk4/secring.gpg' created
[00:02:26] gpg: keyring `/tmp/tmpgahunmk4/pubring.gpg' created
[00:02:26] gpg: requesting key F64D33B0 from hkp server keyserver.ubuntu.com
[00:02:27] gpg: /tmp/tmpgahunmk4/trustdb.gpg: trustdb created
[00:02:27] gpg: no ultimately trusted keys found
[00:02:27] gpg: Total number processed: 1
[00:02:27] gpg:               imported: 1  (RSA: 1)
[00:02:27] OK
---
[00:09:10] Step 7/52 : COPY dist-various-1/install-x86_64-redox.sh /build
[00:09:11]  ---> 56c8b34365e4
[00:09:11] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:09:11]  ---> Running in 21be1df818e9
[00:09:11] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:09:11] + tar --extract --gzip --directory /usr/local
[00:09:11] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:09:11] gzip: stdin: unexpected end of file
[00:09:11] tar: Child returned status 1
[00:09:11] tar: Error is not recoverable: exiting now
[00:09:11] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:09:13]  ---> Using cache
[00:09:13]  ---> 56c8b34365e4
[00:09:13] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:09:13]  ---> Running in 3d4a45cef748
[00:09:13] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:09:13] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:09:13] + tar --extract --gzip --directory /usr/local
[00:09:13] gzip: stdin: unexpected end of file
[00:09:13] tar: Child returned status 1
[00:09:13] tar: Error is not recoverable: exiting now
[00:09:13] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:09:16]  ---> Using cache
[00:09:16]  ---> 56c8b34365e4
[00:09:16] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:09:16]  ---> Running in a435dd9dbf66
[00:09:16] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:09:16] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:09:16] + tar --extract --gzip --directory /usr/local
[00:09:16] gzip: stdin: unexpected end of file
[00:09:16] tar: Child returned status 1
[00:09:16] tar: Error is not recoverable: exiting now
[00:09:16] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:09:20]  ---> Using cache
[00:09:20]  ---> 56c8b34365e4
[00:09:20] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:09:20]  ---> Running in 4e733c464d9b
[00:09:20] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:09:20] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:09:20] + tar --extract --gzip --directory /usr/local
[00:09:20] gzip: stdin: unexpected end of file
[00:09:20] tar: Child returned status 1
[00:09:20] tar: Error is not recoverable: exiting now
[00:09:20] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
[00:09:25]  ---> Using cache
[00:09:25]  ---> 56c8b34365e4
[00:09:25] Step 8/52 : RUN ./install-x86_64-redox.sh
[00:09:25]  ---> Running in d8fa2c6140bf
[00:09:25] + wget -O - https://static.redox-os.org/toolchain/x86_64-unknown-redox/relibc-install.tar.gz
[00:09:25] ./install-x86_64-redox.sh: line 6: wget: command not found
[00:09:25] + tar --extract --gzip --directory /usr/local
[00:09:25] gzip: stdin: unexpected end of file
[00:09:25] tar: Child returned status 1
[00:09:25] tar: Error is not recoverable: exiting now
[00:09:25] The command '/bin/sh -c ./install-x86_64-redox.sh' returned a non-zero code: 2
---
travis_time:end:1681692b:start=1553057414425747872,finish=1553057414435417846,duration=9669974
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26effe94
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:087fbab0
travis_time:start:087fbab0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:00c41f10
$ dmesg | grep -i kill
