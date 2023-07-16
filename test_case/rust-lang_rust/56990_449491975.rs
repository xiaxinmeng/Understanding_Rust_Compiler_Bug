plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:052e652b
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:00:00] Attempting with retry: sh -c rm -f download-src-tools-lld.tar.gz &&         curl -sSL -o download-src-tools-lld.tar.gz https://github.com/rust-lang/lld/archive/1928c5eeb613a4c6d232fc47ae91914bbfd92a79.tar.gz
[00:00:00] rm 'src/tools/lldb'
[00:00:00] Attempting with retry: sh -c rm -f download-src-tools-lldb.tar.gz &&         curl -sSL -o download-src-tools-lldb.tar.gz https://github.com/rust-lang-nursery/lldb/archive/8ad0817ce45b0eef9d374691b23f2bd69c164254.tar.gz
[00:00:00] rm 'src/tools/clang'
[00:00:00] Attempting with retry: sh -c git submodule deinit -f  src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd src/libbacktrace src/tools/gdb src/doc/rustc-guide src/doc/edition-guide &&     git submodule sync &&     git submodule update -j 16 --init --recursive  src/tools/rust-installer src/doc/nomicon src/tools/cargo src/doc/reference src/tools/rls src/tools/clippy src/tools/rustfmt src/tools/miri src/stdsimd src/libbacktrace src/tools/gdb src/doc/rustc-guide src/doc/edition-guide
[00:00:00] Cleared directory 'src/doc/edition-guide'
[00:00:00] Cleared directory 'src/doc/nomicon'
[00:00:00] Cleared directory 'src/doc/reference'
[00:00:00] Cleared directory 'src/doc/rustc-guide'
---
[00:00:00] Submodule 'src/libbacktrace' (https://github.com/rust-lang-nursery/libbacktrace.git) registered for path 'src/libbacktrace'
[00:00:00] Submodule 'src/stdsimd' (https://github.com/rust-lang-nursery/stdsimd.git) registered for path 'src/stdsimd'
[00:00:00] Submodule 'src/tools/cargo' (https://github.com/rust-lang/cargo.git) registered for path 'src/tools/cargo'
[00:00:00] Submodule 'src/tools/clippy' (https://github.com/rust-lang-nursery/rust-clippy.git) registered for path 'src/tools/clippy'
[00:00:00] Submodule 'src/tools/gdb' (https://github.com/rust-dev-tools/gdb.git) registered for path 'src/tools/gdb'
[00:00:00] Submodule 'src/tools/rls' (https://github.com/rust-lang-nursery/rls.git) registered for path 'src/tools/rls'
[00:00:00] Submodule 'src/rust-installer' (https://github.com/rust-lang/rust-installer.git) registered for path 'src/tools/rust-installer'
[00:00:00] Submodule 'src/tools/rustfmt' (https://github.com/rust-lang-nursery/rustfmt.git) registered for path 'src/tools/rustfmt'
[00:00:00] Cloning into '/home/travis/build/rust-lang/rust/src/doc/edition-guide'...
---
[00:01:09] Submodule path 'src/libbacktrace': checked out 'f4d02bbdbf8a2c5a31f0801dfef597a86caad9e3'
[00:01:10] Submodule path 'src/stdsimd': checked out '3c0503db8439928e42c1175f0009c506fc874ae9'
[00:01:10] Submodule path 'src/tools/cargo': checked out '2cf1f5dda2f7ed84e94c4d32f643e0f1f15352f0'
[00:01:10] Submodule path 'src/tools/clippy': checked out 'a416c5e0f7c4c9473069a58410d3ec3e86b1ac0d'
[00:01:12] Submodule path 'src/tools/gdb': checked out 'f1f53755e84e32511afd7cd38ee9d71987e2662c'
[00:01:12] Submodule path 'src/tools/rls': checked out 'bd5b899afb05e14d33e210ede3da241ca1ca088f'
[00:01:12] Submodule path 'src/tools/rust-installer': checked out '27dec6cae3a8132d8a073aad6775425c85095c99'
[00:01:12] Submodule path 'src/tools/rustfmt': checked out 'be135599ef5e54b5219f9adec68e1ee267ea0584'
[00:01:12] travis_fold:end:init_repo
---
[00:01:33] Step 5/41 : RUN sed -i 's|#\(baseurl.*\)mirror.centos.org/centos/$releasever|\1vault.centos.org/5.11|' /etc/yum.repos.d/*.repo
[00:01:33]  ---> Running in 3b47225008f3
[00:01:33]  ---> ff9bcb7ec3f0
[00:01:33] Removing intermediate container 3b47225008f3
[00:01:33] Step 6/41 : RUN yum upgrade -y && yum install -y       curl       bzip2       gcc       gcc-c++       make       glibc-devel       perl       zlib-devel       file       xz       which       pkgconfig       wget       autoconf       gettext       xz-static       xz-devel       expat-static       expat-devel       ncurses-static       ncurses-devel
[00:01:38] Reducing CentOS-5 - libselinux to included packages only
[00:01:38] Finished
[00:01:38] Setting up Upgrade Process
[00:01:38] Resolving Dependencies
---
[00:01:38] 
[00:01:38] Total download size: 4.9 M
[00:01:38] Downloading Packages:
[00:01:41] --------------------------------------------------------------------------------
[00:01:41] Total                                           1.7 MB/s | 4.9 MB     00:02     
[00:01:41] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:41] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:41] Running Transaction Test
[00:01:41] Finished Transaction Test
[00:01:41] Transaction Test Succeeded
[00:01:41] Running Transaction
---
[00:01:42] Complete!
[00:01:43] Reducing CentOS-5 - libselinux to included packages only
[00:01:43] Finished
[00:01:43] Setting up Install Process
[00:01:44] No package xz-static available.
[00:01:44] No package expat-static available.
[00:01:44] No package ncurses-static available.
[00:01:44] --> Running transaction check
[00:01:44] ---> Package autoconf.noarch 0:2.59-12 set to be updated
[00:01:44] --> Processing Dependency: imake for package: autoconf
[00:01:44] --> Processing Dependency: m4 for package: autoconf
---
[00:01:45] --> Processing Dependency: glibc-headers = 2.5-123.el5_11.3 for package: glibc-devel
[00:01:45] --> Processing Dependency: glibc-headers for package: glibc-devel
[00:01:45] ---> Package glibc-devel.x86_64 0:2.5-123.el5_11.3 set to be updated
[00:01:45] ---> Package make.x86_64 1:3.81-3.el5 set to be updated
[00:01:45] ---> Package ncurses-devel.i386 0:5.5-24.20060715 set to be updated
[00:01:45] --> Processing Dependency: libmenu.so.5 for package: ncurses-devel
[00:01:45] --> Processing Dependency: libpanelw.so.5 for package: ncurses-devel
[00:01:45] --> Processing Dependency: libncursesw.so.5 for package: ncurses-devel
[00:01:45] --> Processing Dependency: libncurses.so.5 for package: ncurses-devel
[00:01:45] --> Processing Dependency: libform.so.5 for package: ncurses-devel
[00:01:45] --> Processing Dependency: libpanel.so.5 for package: ncurses-devel
[00:01:45] --> Processing Dependency: libformw.so.5 for package: ncurses-devel
[00:01:45] --> Processing Dependency: libmenuw.so.5 for package: ncurses-devel
[00:01:45] ---> Package ncurses-devel.x86_64 0:5.5-24.20060715 set to be updated
[00:01:45] --> Processing Dependency: libgdbm.so.2 for package: perl
[00:01:45] --> Processing Dependency: libdb-4.3.so for package: perl
[00:01:45] ---> Package perl.x86_64 4:5.8.8-43.el5_11 set to be updated
[00:01:45] ---> Package pkgconfig.x86_64 1:0.21-2.el5 set to be updated
---
[00:01:45] ---> Package libgomp.x86_64 0:4.4.7-1.el5 set to be updated
[00:01:45] ---> Package libidn.i386 0:0.6.5-1.1 set to be updated
[00:01:45] ---> Package libidn.x86_64 0:0.6.5-1.1 set to be updated
[00:01:45] ---> Package libstdc++-devel.x86_64 0:4.1.2-55.el5 set to be updated
[00:01:57] ---> Package m4.x86_64 0:1.4.5-3.el5.1 set to be updated
[00:01:57] ---> Package ncurses.i386 0:5.5-24.20060715 set to be updated
[00:01:57] ---> Package xz-libs.i386 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
[00:01:57] ---> Package xz-libs.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
[00:01:57] ---> Package zlib.i386 0:1.2.3-7.el5 set to be updated
[00:01:57] --> Running transaction check
---
[00:04:09] + hide_output make install
[00:04:09] + set +x
[00:04:28] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:28] + cd ..
[00:04:28] + rm -rf openssl-1.0.2k
[00:04:28] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:28] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:29] Removing intermediate container 818188802daa
[00:04:29] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:29]  ---> 85f8dc9de8dd
[00:04:29] Step 15/41 : RUN ./build-curl.sh
[00:04:29] Step 15/41 : RUN ./build-curl.sh
[00:04:29]  ---> Running in cd0efc2ddecf
[00:04:29] + source shared.sh
[00:04:29] + VERSION=7.51.0
[00:04:29] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:31]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:31]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:32] 
  0 2509k    0  6975    0     0   4286      0  0:09:59  0:00:01  0:09:58  4286
  0 2509k    0  6975    0     0   4286      0  0:09:59  0:00:01  0:09:58  4286
  8 2509k    8  220k    0     0   103k      0  0:00:24  0:00:02  0:00:22  428k
100 2509k  100 2509k    0     0   904k      0  0:00:02  0:00:02 --:--:-- 2180k
[00:04:32] + mkdir curl-build
[00:04:32] + cd curl-build
[00:04:32] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:54] + hide_output make -j10
[00:04:54] + set +x
[00:05:07] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:07] + hide_output make install
---
 95 82.1M   95 78.5M    0     0  3746k      0  0:00:22  0:00:21  0:00:01 3222k
 99 82.1M   99 81.8M    0     0  3727k      0  0:00:22  0:00:22 --:--:-- 3341k
100 82.1M  100 82.1M    0     0  3728k      0  0:00:22  0:00:22 --:--:-- 3483k
[00:08:53] + cd gcc-4.8.5
[00:08:53] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:53] --2018-12-21 17:44:30--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:53] Resolving gcc.gnu.org... 209.132.180.131
[00:08:53] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:53] HTTP request sent, awaiting response... 200 OK
---
100  894k  100  894k    0     0  2010k      0 --:--:-- --:--:-- --:--:-- 2018k
100  894k  100  894k    0     0  2010k      0 --:--:-- --:--:-- --:--:-- 2014k
[00:43:07] + mkdir ../clang-build
[00:43:07] + cd ../clang-build
[00:43:07] + INC=/rustroot/include
[00:43:07] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:43:07] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:43:07] + hide_output cmake ../llvm-7.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:43:37] Fri Dec 21 18:19:15 UTC 2018 - building ...
[00:43:40] + hide_output make -j10
[00:43:40] + set +x
[00:44:10] Fri Dec 21 18:19:47 UTC 2018 - building ...
---
[01:42:12] Step 32/41 : RUN ./build-perl.sh
[01:42:12]  ---> Running in 94762fa77748
[01:42:12] + source shared.sh
[01:42:12] + tar xzf -
[01:42:12] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:42:12]                                  Dload  Upload   Total   Spent    Left  Speed
[01:42:13] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 13 17.0M   13 2416k    0     0  5319k      0  0:00:03 --:--:--  0:00:03 5835k
---
[01:46:48] configure: rust.llvm-tools      := True
[01:46:48] configure: build.extended       := True
[01:46:48] configure: build.cargo-native-static := True
[01:46:48] configure: dist.missing-tools   := True
[01:46:48] configure: build.configure-args := ['--enable-full-tools', '--enable-gdb', '--ena ...
[01:46:48] configure: writing `config.toml` in current directory
[01:46:48] configure: 
[01:46:48] configure: run `python /checkout/x.py --help`
[01:46:48] configure: 
---
[02:59:00] [ 98%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/coff2yaml.cpp.o
[02:59:01] [ 98%] Building CXX object tools/llvm-xray/CMakeFiles/llvm-xray.dir/xray-account.cpp.o
[02:59:02] [ 98%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/dwarf2yaml.cpp.o
[02:59:05] [ 98%] Building CXX object tools/obj2yaml/CMakeFiles/obj2yaml.dir/elf2yaml.cpp.o
The job exceeded the maximum time limit for jobs, and has been terminated.
