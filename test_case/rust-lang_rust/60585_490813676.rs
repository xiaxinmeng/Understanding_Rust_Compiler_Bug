plain
[00:01:05]   readline-common tex-common ucf
[00:01:05] Suggested packages:
[00:01:05]   autoconf-archive gnu-standards autoconf-doc gettext binutils-doc bison-doc
[00:01:05]   cmake-doc ninja-build cpp-doc gcc-7-locales debian-keyring build-essential
[00:01:05]   flex-doc g++-multilib g++-7-multilib gcc-7-doc libstdc++6-7-dbg gawk-doc
[00:01:05]   libitm1-dbg libatomic1-dbg libasan4-dbg liblsan0-dbg libtsan0-dbg
[00:01:05]   libubsan0-dbg libcilkrts5-dbg libmpx2-dbg libquadmath0-dbg gdb-doc
[00:01:05]   gettext-base git-daemon-run | git-daemon-sysvinit git-doc git-el git-email
[00:01:05]   git-gui gitk gitweb git-cvs git-mediawiki git-svn lrzip glibc-doc gnupg
---
[00:01:28] Setting up libmpc3:amd64 (1.1.0-1) ...
[00:01:28] Setting up libc-dev-bin (2.27-3ubuntu1) ...
[00:01:28] Setting up bison (2:3.0.4.dfsg-1build1) ...
[00:01:28] update-alternatives: using /usr/bin/bison.yacc to provide /usr/bin/yacc (yacc) in auto mode
[00:01:28] update-alternatives: warning: skip creation of /usr/share/man/man1/yacc.1.gz because associated file /usr/share/man/man1/bison.yacc.1.gz (of link group yacc) doesn't exist
[00:01:28] Setting up libkeyutils1:amd64 (1.5.9-9.2ubuntu2) ...
[00:01:28] Setting up libpython2.7-minimal:amd64 (2.7.15~rc1-1ubuntu0.1) ...
[00:01:28] Setting up ca-certificates (20180409) ...
[00:01:29] debconf: unable to initialize frontend: Dialog
---
[00:01:31] Setting up dpkg-dev (1.19.0.5ubuntu2.1) ...
[00:01:31] Setting up libldap-2.4-2:amd64 (2.4.45+dfsg-1ubuntu1.2) ...
[00:01:31] Setting up automake (1:1.15.1-3ubuntu2) ...
[00:01:31] update-alternatives: using /usr/bin/automake-1.15 to provide /usr/bin/automake (automake) in auto mode
[00:01:31] update-alternatives: warning: skip creation of /usr/share/man/man1/automake.1.gz because associated file /usr/share/man/man1/automake-1.15.1.gz (of link group automake) doesn't exist
[00:01:31] update-alternatives: warning: skip creation of /usr/share/man/man1/aclocal.1.gz because associated file /usr/share/man/man1/aclocal-1.15.1.gz (of link group automake) doesn't exist
[00:01:31] update-alternatives: using /usr/bin/g++ to provide /usr/bin/c++ (c++) in auto mode
[00:01:31] update-alternatives: warning: skip creation of /usr/share/man/man1/c++.1.gz because associated file /usr/share/man/man1/g++.1.gz (of link group c++) doesn't exist
[00:01:31] Setting up gdb (8.1-0ubuntu3) ...
[00:01:31] Setting up libxml-sax-perl (0.99+dfsg-2ubuntu1) ...
---
[00:01:40]  ---> ed72928b9c16
[00:01:40] Step 4/39 : RUN apt-get build-dep -y clang llvm && apt-get install -y --no-install-recommends   build-essential   gcc-multilib   libedit-dev   libgmp-dev   libisl-dev   libmpc-dev   libmpfr-dev   ninja-build   nodejs   python2.7-dev   software-properties-common   unzip
[00:01:40]  ---> Running in bf997ee48ce9
[00:01:41] Reading package lists...
[00:01:41] E: You must put some 'source' URIs in your sources.list
[00:01:41] The command '/bin/sh -c apt-get build-dep -y clang llvm && apt-get install -y --no-install-recommends   build-essential   gcc-multilib   libedit-dev   libgmp-dev   libisl-dev   libmpc-dev   libmpfr-dev   ninja-build   nodejs   python2.7-dev   software-properties-common   unzip' returned a non-zero code: 100
[00:01:43] Sending build context to Docker daemon  520.7kB
[00:01:43] Step 1/39 : FROM ubuntu:18.04
[00:01:43]  ---> d131e0fa2585
[00:01:43] Step 2/39 : COPY scripts/cross-apt-packages.sh /scripts/
---
[00:01:43]  ---> ed72928b9c16
[00:01:43] Step 4/39 : RUN apt-get build-dep -y clang llvm && apt-get install -y --no-install-recommends   build-essential   gcc-multilib   libedit-dev   libgmp-dev   libisl-dev   libmpc-dev   libmpfr-dev   ninja-build   nodejs   python2.7-dev   software-properties-common   unzip
[00:01:43]  ---> Running in 9fadf41f1ab8
[00:01:44] Reading package lists...
[00:01:44] E: You must put some 'source' URIs in your sources.list
[00:01:44] The command '/bin/sh -c apt-get build-dep -y clang llvm && apt-get install -y --no-install-recommends   build-essential   gcc-multilib   libedit-dev   libgmp-dev   libisl-dev   libmpc-dev   libmpfr-dev   ninja-build   nodejs   python2.7-dev   software-properties-common   unzip' returned a non-zero code: 100
[00:01:46] Sending build context to Docker daemon  520.7kB
[00:01:46] Step 1/39 : FROM ubuntu:18.04
[00:01:46]  ---> d131e0fa2585
[00:01:46] Step 2/39 : COPY scripts/cross-apt-packages.sh /scripts/
---
[00:01:46]  ---> ed72928b9c16
[00:01:46] Step 4/39 : RUN apt-get build-dep -y clang llvm && apt-get install -y --no-install-recommends   build-essential   gcc-multilib   libedit-dev   libgmp-dev   libisl-dev   libmpc-dev   libmpfr-dev   ninja-build   nodejs   python2.7-dev   software-properties-common   unzip
[00:01:47]  ---> Running in 5e8b452a134a
[00:01:48] Reading package lists...
[00:01:48] E: You must put some 'source' URIs in your sources.list
[00:01:48] The command '/bin/sh -c apt-get build-dep -y clang llvm && apt-get install -y --no-install-recommends   build-essential   gcc-multilib   libedit-dev   libgmp-dev   libisl-dev   libmpc-dev   libmpfr-dev   ninja-build   nodejs   python2.7-dev   software-properties-common   unzip' returned a non-zero code: 100
[00:01:51] Sending build context to Docker daemon  520.7kB
[00:01:51] Step 1/39 : FROM ubuntu:18.04
[00:01:51]  ---> d131e0fa2585
[00:01:51] Step 2/39 : COPY scripts/cross-apt-packages.sh /scripts/
---
[00:01:51]  ---> ed72928b9c16
[00:01:51] Step 4/39 : RUN apt-get build-dep -y clang llvm && apt-get install -y --no-install-recommends   build-essential   gcc-multilib   libedit-dev   libgmp-dev   libisl-dev   libmpc-dev   libmpfr-dev   ninja-build   nodejs   python2.7-dev   software-properties-common   unzip
[00:01:51]  ---> Running in 736088e22b09
[00:01:53] Reading package lists...
[00:01:53] E: You must put some 'source' URIs in your sources.list
[00:01:53] The command '/bin/sh -c apt-get build-dep -y clang llvm && apt-get install -y --no-install-recommends   build-essential   gcc-multilib   libedit-dev   libgmp-dev   libisl-dev   libmpc-dev   libmpfr-dev   ninja-build   nodejs   python2.7-dev   software-properties-common   unzip' returned a non-zero code: 100
[00:01:57] Sending build context to Docker daemon  520.7kB
[00:01:57] Step 1/39 : FROM ubuntu:18.04
[00:01:57]  ---> d131e0fa2585
[00:01:57] Step 2/39 : COPY scripts/cross-apt-packages.sh /scripts/
---
[00:01:57]  ---> ed72928b9c16
[00:01:57] Step 4/39 : RUN apt-get build-dep -y clang llvm && apt-get install -y --no-install-recommends   build-essential   gcc-multilib   libedit-dev   libgmp-dev   libisl-dev   libmpc-dev   libmpfr-dev   ninja-build   nodejs   python2.7-dev   software-properties-common   unzip
[00:01:57]  ---> Running in f0aa0b69667b
[00:01:59] Reading package lists...
[00:01:59] E: You must put some 'source' URIs in your sources.list
[00:01:59] The command '/bin/sh -c apt-get build-dep -y clang llvm && apt-get install -y --no-install-recommends   build-essential   gcc-multilib   libedit-dev   libgmp-dev   libisl-dev   libmpc-dev   libmpfr-dev   ninja-build   nodejs   python2.7-dev   software-properties-common   unzip' returned a non-zero code: 100
travis_time:end:066d3200:start=1557391174303231325,finish=1557391293981885728,duration=119678654403
The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:19819cc0
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
---
travis_time:end:0f4bead0:start=1557391294711557553,finish=1557391294717371365,duration=5813812
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:1bc2204e
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:0010fbc0
travis_time:start:0010fbc0
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:13cbb6c4
$ dmesg | grep -i kill
