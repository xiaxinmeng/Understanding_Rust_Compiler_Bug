plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:03206532
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:00:55] Reading package lists...
[00:00:55] Building dependency tree...
[00:00:55] Reading state information...
[00:00:55] The following additional packages will be installed:
[00:00:55]   binutils cmake-data cpp cpp-5 g++-5 gcc gcc-5 git-man icu-devtools
[00:00:55]   libbabeltrace1 libc-dev-bin libc6-dev libcc1-0 libcilkrts5 libcurl3
[00:00:55]   libcurl3-gnutls liberror-perl libexpat1 libexpat1-dev libffi6 libgcc-5-dev
[00:00:55]   libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1 libgssapi-krb5-2
[00:00:55]   libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
---
[00:00:55]   libgcc1-dbg libgomp1-dbg libitm1-dbg libatomic1-dbg libasan2-dbg
[00:00:55]   liblsan0-dbg libtsan0-dbg libubsan0-dbg libcilkrts5-dbg libmpx0-dbg
[00:00:55]   libquadmath0-dbg gdb-doc gettext-base git-daemon-run | git-daemon-sysvinit
[00:00:55]   git-doc git-el git-email git-gui gitk gitweb git-arch git-cvs git-mediawiki
[00:00:55]   git-svn lrzip glibc-doc gnutls-bin krb5-doc krb5-user icu-doc ncurses-doc
[00:00:55]   libstdc++-5-doc pkg-config make-doc perl-doc libterm-readline-gnu-perl
[00:00:55] Recommended packages:
[00:00:55]   libc-dbg gdbserver patch less rsync ssh-client manpages manpages-dev
[00:00:55]   libglib2.0-data shared-mime-info xdg-user-dirs krb5-locales libsasl2-modules
[00:00:55]   xml-core netbase rename
[00:00:55]   xml-core netbase rename
[00:00:55] The following NEW packages will be installed:
[00:00:55]   binutils ca-certificates cmake cmake-data cpp cpp-5 curl file g++ g++-5 gcc
[00:00:55]   gcc-5 gdb git git-man icu-devtools libarchive13 libasan2 libasn1-8-heimdal
[00:00:55]   libcilkrts5 libcurl3 libcurl3-gnutls liberror-perl libexpat1 libexpat1-dev
[00:00:55]   libffi6 libgcc-5-dev libgdbm3 libglib2.0-0 libgmp10 libgnutls30 libgomp1
[00:00:55]   libgssapi-krb5-2 libgssapi3-heimdal libhcrypto4-heimdal libheimbase1-heimdal
[00:00:55]   libheimntlm0-heimdal libhogweed4 libhx509-5-heimdal libicu-dev libicu55
---
[00:01:02] Get:85 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 gdb amd64 7.11.1-0ubuntu1~16.5 [2526 kB]
[00:01:02] Get:86 http://archive.ubuntu.com/ubuntu xenial/main amd64 liberror-perl all 0.17-1.2 [19.6 kB]
[00:01:02] Get:87 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 git-man all 1:2.7.4-0ubuntu1.6 [736 kB]
[00:01:02] Get:88 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 git amd64 1:2.7.4-0ubuntu1.6 [3176 kB]
[00:01:02] Get:89 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 icu-devtools amd64 55.1-7ubuntu0.4 [166 kB]
[00:01:02] Get:91 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libicu-dev amd64 55.1-7ubuntu0.4 [8566 kB]
[00:01:03] Get:92 http://archive.ubuntu.com/ubuntu xenial/main amd64 libtinfo-dev amd64 6.0+20160213-1ubuntu1 [77.4 kB]
[00:01:03] Get:93 http://archive.ubuntu.com/ubuntu xenial/main amd64 libncurses5-dev amd64 6.0+20160213-1ubuntu1 [175 kB]
[00:01:03] Get:94 http://archive.ubuntu.com/ubuntu xenial-updates/main amd64 libpython2.7-stdlib amd64 2.7.12-1ubuntu0~16.04.4 [1880 kB]
---
[00:01:16] Unpacking git-man (1:2.7.4-0ubuntu1.6) ...
[00:01:16] Selecting previously unselected package git.
[00:01:16] Preparing to unpack .../git_1%3a2.7.4-0ubuntu1.6_amd64.deb ...
[00:01:16] Unpacking git (1:2.7.4-0ubuntu1.6) ...
[00:01:16] Selecting previously unselected package icu-devtools.
[00:01:16] Preparing to unpack .../icu-devtools_55.1-7ubuntu0.4_amd64.deb ...
[00:01:16] Unpacking icu-devtools (55.1-7ubuntu0.4) ...
[00:01:16] Preparing to unpack .../libexpat1-dev_2.1.0-7ubuntu0.16.04.3_amd64.deb ...
[00:01:16] Unpacking libexpat1-dev:amd64 (2.1.0-7ubuntu0.16.04.3) ...
[00:01:16] Selecting previously unselected package libicu-dev:amd64.
[00:01:16] Preparing to unpack .../libicu-dev_55.1-7ubuntu0.4_amd64.deb ...
---
[00:01:23] Setting up gdb (7.11.1-0ubuntu1~16.5) ...
[00:01:23] Setting up liberror-perl (0.17-1.2) ...
[00:01:23] Setting up git-man (1:2.7.4-0ubuntu1.6) ...
[00:01:23] Setting up git (1:2.7.4-0ubuntu1.6) ...
[00:01:23] Setting up icu-devtools (55.1-7ubuntu0.4) ...
[00:01:23] Setting up libicu-dev:amd64 (55.1-7ubuntu0.4) ...
[00:01:23] Setting up libtinfo-dev:amd64 (6.0+20160213-1ubuntu1) ...
[00:01:23] Setting up libncurses5-dev:amd64 (6.0+20160213-1ubuntu1) ...
[00:01:23] Setting up libpython2.7-stdlib:amd64 (2.7.12-1ubuntu0~16.04.4) ...
---
[00:01:36] Step 7/8 : ENV RUST_CONFIGURE_ARGS       --build=x86_64-unknown-linux-gnu       --enable-debug       --enable-lld       --enable-lldb       --enable-optimize
[00:01:37]  ---> Running in 71d8f485d501
[00:01:37] Removing intermediate container 71d8f485d501
[00:01:37]  ---> 175c4f85f08b
[00:01:37] Step 8/8 : ENV SCRIPT   python2.7 ../x.py build &&   python2.7 ../x.py test src/test/run-make-fulldeps --test-args clang
[00:01:37] Removing intermediate container d4768a658b6e
[00:01:37]  ---> 2a9769a26165
[00:01:37] Successfully built 2a9769a26165
[00:01:37] Successfully tagged rust-ci:latest
---
[00:35:02] -- Looking for pthread_create in pthreads - not found
[00:35:02] -- Looking for pthread_create in pthread
[00:35:02] -- Looking for pthread_create in pthread - found
[00:35:02] -- Found Threads: TRUE  
[00:35:02] -- Found LibXml2: /usr/lib/x86_64-linux-gnu/libxml2.so (found version "2.9.3") 
[00:35:02] -- Looking for xar_open in xar - not found
[00:35:02] -- Looking for arc4random
[00:35:02] -- Looking for arc4random - not found
[00:35:02] -- Looking for backtrace
---
[00:35:08] -- Looking for sys/resource.h - found
[00:35:08] -- Clang version: 8.0.0
[00:35:08] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG
[00:35:08] -- Performing Test CXX_SUPPORTS_NO_NESTED_ANON_TYPES_FLAG - Failed
[00:35:08] -- Could NOT find Z3 (missing:  Z3_LIBRARIES Z3_INCLUDE_DIR) (Required is exact version "4.7.1")
[00:35:08] -- Found PythonLibs: /usr/lib/x86_64-linux-gnu/libpython2.7.so (found version "2.7.12") 
[00:35:08] -- Performing Test CXX_SUPPORTS_NO_DEPRECATED_DECLARATIONS - Success
[00:35:08] -- Performing Test CXX_SUPPORTS_NO_UNKNOWN_PRAGMAS
[00:35:08] -- Performing Test CXX_SUPPORTS_NO_UNKNOWN_PRAGMAS - Success
[00:35:08] -- Performing Test CXX_SUPPORTS_NO_STRICT_ALIASING
---
[00:35:09] -- LLDB version: 8.0.0
[00:35:09] -- Found Curses: /usr/lib/x86_64-linux-gnu/libcurses.so  
[00:35:09] -- Looking for __GLIBCXX__
[00:35:09] -- Looking for __GLIBCXX__ - found
[00:35:09] -- Performing Test LLDB_USING_LIBSTDCXX_4_9
[00:35:09] -- Performing Test LLDB_USING_LIBSTDCXX_4_9 - Success
[00:35:09] -- Looking for ppoll - found
[00:35:09] -- Looking for sigaction
[00:35:09] -- Looking for sigaction - found
[00:35:09] -- Looking for accept4
---
[00:35:10] -- Looking for stddef.h
[00:35:10] -- Looking for stddef.h - found
[00:35:10] -- Check size of el_rfunc_t
[00:35:10] -- Check size of el_rfunc_t - failed
[00:35:10] -- Could NOT find libedit (missing:  libedit_INCLUDE_DIRS libedit_LIBRARIES) 
[00:35:10] -- Could NOT find Doxygen (missing:  DOXYGEN_EXECUTABLE) 
[00:35:10] CMake Error at /usr/share/cmake-3.5/Modules/FindPackageHandleStandardArgs.cmake:148 (message):
[00:35:10]   Could NOT find SWIG (missing: SWIG_EXECUTABLE SWIG_DIR)
[00:35:10] Call Stack (most recent call first):
[00:35:10]   /usr/share/cmake-3.5/Modules/FindPackageHandleStandardArgs.cmake:388 (_FPHSA_FAILURE_MESSAGE)
[00:35:10]   /usr/share/cmake-3.5/Modules/FindSWIG.cmake:75 (FIND_PACKAGE_HANDLE_STANDARD_ARGS)
[00:35:10]   /checkout/src/tools/lldb/scripts/CMakeLists.txt:35 (find_package)
[00:35:10] 
[00:35:10] -- Configuring incomplete, errors occurred!
[00:35:10] -- Configuring incomplete, errors occurred!
[00:35:10] See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeOutput.log".
[00:35:10] See also "/checkout/obj/build/x86_64-unknown-linux-gnu/llvm/build/CMakeFiles/CMakeError.log".
[00:35:10] command did not execute successfully, got: exit code: 1
[00:35:10] 
[00:35:10] 
[00:35:10] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.33/src/lib.rs:773:5
[00:35:10]  finished in 11.361
[00:35:10] travis_fold:end:llvm

[00:35:10] travis_time:end:llvm:start=1548337714249159054,finish=1548337725610426284,duration=11361267230
---
travis_time:end:009876c0:start=1548337726897889062,finish=1548337726903895883,duration=6006821
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:26bc37d0
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:1806ace4
travis_time:start:1806ace4
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:15ea5870
$ dmesg | grep -i kill
