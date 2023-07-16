plain

Network availability confirmed.
travis_fold:start:git.checkout
travis_time:start:172a8f1c
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:00:23] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
[00:00:23] curl: (22) The requested URL returned error: 404 Not Found
[00:00:23] The command has failed after 5 attempts.
[00:00:23] open /tmp/rustci_docker_cache: no such file or directory
[00:00:23] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/dist-x86_64-linux/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:00:23] Sending build context to Docker daemon  501.8kB
[00:00:23] Step 1/38 : FROM centos:5
[00:00:24] 5: Pulling from library/centos
---
[00:00:57] 
[00:00:57] Total download size: 4.9 M
[00:00:57] Downloading Packages:
[00:01:06] --------------------------------------------------------------------------------
[00:01:06] Total                                           550 kB/s | 4.9 MB     00:09     
[00:01:06] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:06] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:06] Running Transaction Test
[00:01:06] Finished Transaction Test
[00:01:06] Transaction Test Succeeded
[00:01:06] Running Transaction
---
[00:01:09] --> Processing Dependency: libc.so.6(GLIBC_2.2) for package: curl
[00:01:09] --> Processing Dependency: libidn.so.11 for package: curl
[00:01:09] --> Processing Dependency: libz.so.1 for package: curl
[00:01:09] --> Processing Dependency: libc.so.6 for package: curl
[00:01:09] --> Processing Dependency: libdl.so.2 for package: curl
[00:01:09] --> Processing Dependency: libgssapi_krb5.so.2(gssapi_krb5_2_MIT) for package: curl
[00:01:09] --> Processing Dependency: libc.so.6(GLIBC_2.3.4) for package: curl
[00:01:09] --> Processing Dependency: libk5crypto.so.3 for package: curl
[00:01:09] --> Processing Dependency: libc.so.6(GLIBC_2.1) for package: curl
[00:01:09] --> Processing Dependency: libssl.so.6 for package: curl
---
[00:01:09] --> Processing Dependency: glibc-headers = 2.5-123.el5_11.3 for package: glibc-devel
[00:01:09] --> Processing Dependency: glibc-headers for package: glibc-devel
[00:01:09] ---> Package glibc-devel.x86_64 0:2.5-123.el5_11.3 set to be updated
[00:01:09] ---> Package make.x86_64 1:3.81-3.el5 set to be updated
[00:01:09] ---> Package perl.i386 4:5.8.8-43.el5_11 set to be updated
[00:01:09] --> Processing Dependency: libgdbm.so.2 for package: perl
[00:01:09] --> Processing Dependency: libdb-4.3.so for package: perl
[00:01:09] ---> Package perl.x86_64 4:5.8.8-43.el5_11 set to be updated
[00:01:09] ---> Package pkgconfig.x86_64 1:0.21-2.el5 set to be updated
[00:01:09] ---> Package which.x86_64 0:2.16-7 set to be updated
[00:01:09] ---> Package xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
[00:01:09] --> Processing Dependency: xz-libs = 4.999.9-0.3.beta.20091007git.el5 for package: xz
[00:01:09] --> Processing Dependency: xz-libs = 4.999.9-0.3.beta.20091007git.el5 for package: xz
[00:01:09] --> Processing Dependency: liblzma.so.0()(64bit) for package: xz
[00:01:09] ---> Package zlib-devel.x86_64 0:1.2.3-7.el5 set to be updated
[00:01:09] --> Running transaction check
[00:01:09] ---> Package cpp.x86_64 0:4.1.2-55.el5 set to be updated
[00:01:09] ---> Package db4.i386 0:4.3.29-10.el5_5.2 set to be updated
[00:01:09] ---> Package db4.i386 0:4.3.29-10.el5_5.2 set to be updated
[00:01:09] --> Processing Dependency: libstdc++.so.6(GLIBCXX_3.4) for package: db4
[00:01:09] --> Processing Dependency: libstdc++.so.6(CXXABI_1.3) for package: db4
[00:01:09] --> Processing Dependency: libstdc++.so.6 for package: db4
[00:01:09] ---> Package e2fsprogs-libs.i386 0:1.39-37.el5 set to be updated
[00:01:09] --> Processing Dependency: libdevmapper.so.1.02 for package: e2fsprogs-libs
[00:01:09] ---> Package gdbm.i386 0:1.8.0-28.el5 set to be updated
[00:01:09] ---> Package glibc.i686 0:2.5-123.el5_11.3 set to be updated
[00:01:09] --> Processing Dependency: kernel-headers >= 2.2.1 for package: glibc-headers
[00:01:09] --> Processing Dependency: kernel-headers for package: glibc-headers
[00:01:09] ---> Package imake.x86_64 0:1.0.2-3 set to be updated
[00:01:09] ---> Package krb5-libs.i386 0:1.6.1-80.el5_11 set to be updated
---
[00:03:51] + hide_output make install
[00:03:51] + set +x
[00:04:10] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:10] + cd ..
[00:04:10] + rm -rf openssl-1.0.2k
[00:04:10] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:10] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:11] Removing intermediate container fe919c67050b
[00:04:11] Step 14/38 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:11]  ---> 72d15cf587db
[00:04:11] Step 15/38 : RUN ./build-curl.sh
[00:04:11] Step 15/38 : RUN ./build-curl.sh
[00:04:11]  ---> Running in c336f946efd1
[00:04:11] + source shared.sh
[00:04:11] + VERSION=7.51.0
[00:04:11] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:13]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:13]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:14] 
  0 2509k    0  2631    0     0   1707      0  0:25:05  0:00:01  0:25:04  1707
  0 2509k    0  2631    0     0   1707      0  0:25:05  0:00:01  0:25:04  1707
  8 2509k    8  220k    0     0   106k      0  0:00:23  0:00:02  0:00:21  414k
100 2509k  100 2509k    0     0   926k      0  0:00:02  0:00:02 --:--:-- 2146k
[00:04:14] + mkdir curl-build
[00:04:14] + cd curl-build
[00:04:14] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:34] + hide_output make -j10
[00:04:34] + set +x
[00:04:47] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:47] + hide_output make install
---
[00:05:42]  ---> 644049aff1f3
[00:05:42] Step 19/38 : RUN ./build-cmake.sh
[00:05:42]  ---> Running in 3164894cfdb8
[00:05:43] + source shared.sh
[00:05:43] + curl https://cmake.org/files/v3.6/cmake-3.6.3.tar.gz
[00:05:43]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:05:43]                                  Dload  Upload   Total   Spent    Left  Speed
[00:05:54] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
 93 82.1M   93 76.8M    0     0  3820k      0  0:00:22  0:00:20  0:00:02 3427k
 98 82.1M   98 80.6M    0     0  3826k      0  0:00:21  0:00:21 --:--:-- 3535k
100 82.1M  100 82.1M    0     0  3825k      0  0:00:21  0:00:21 --:--:-- 3621k
[00:08:50] + cd gcc-4.8.5
[00:08:50] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:50] --2018-05-30 15:14:31--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:50] Resolving gcc.gnu.org... 209.132.180.131
[00:08:50] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:50] HTTP request sent, awaiting response... 200 OK
---
[00:08:53]   1850K ...                                                   100% 5743G=1.4s
[00:08:53] 
[00:08:53] 2018-05-30 15:14:35 (1.30 MB/s) - `gmp-4.3.2.tar.bz2' saved [1897483/1897483]
[00:08:53] 
[00:08:54] --2018-05-30 15:14:35--  http://gcc.gnu.org/pub/gcc/infrastructure/mpc-0.8.1.tar.gz
[00:08:54] Resolving gcc.gnu.org... 209.132.180.131
[00:08:54] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:54] HTTP request sent, awaiting response... 200 OK
[00:08:54] Length: 544950 (532K) [application/x-gzip]
[00:08:54] Saving to: `mpc-0.8.1.tar.gz'
[00:08:54]      0K .......... .......... .......... .......... ..........  9%  289K 2s
[00:08:54]     50K .......... .......... .......... .......... .......... 18%  436K 1s
[00:08:54]    100K .......... .......... .......... .......... .......... 28%  857K 1s
[00:08:54]    150K .......... .......... .......... .......... .......... 37%  448K 1s
---
[00:43:30] Step 23/38 : RUN ./build-python.sh
[00:43:30]  ---> Running in 408e3273bab2
[00:43:31] + source shared.sh
[00:43:31] + tar xzf -
[00:43:31] + curl https://www.python.org/ftp/python/2.7.12/Python-2.7.12.tgz
[00:43:31]                                  Dload  Upload   Total   Spent    Left  Speed
[00:43:32] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 37 16.1M   37 6254k    0     0  8974k      0  0:00:01 --:--:--  0:00:01 9184k
 37 16.1M   37 6254k    0     0  8974k      0  0:00:01 --:--:--  0:00:01 9184k
100 16.1M  100 16.1M    0     0  15.3M      0  0:00:01  0:00:01 --:--:-- 15.5M
[00:43:32] + mkdir python-build
[00:43:32] + cd python-build
[00:43:32] + CFLAGS='-I /rustroot/include'
[00:43:32] + LDFLAGS='-L /rustroot/lib -L /rustroot/lib64'
[00:43:32] + hide_output ../Python-2.7.12/configure --prefix=/rustroot
[00:43:45] + hide_output make -j10
[00:43:45] + set +x
[00:44:15] Wed May 30 15:49:57 UTC 2018 - building ...
[00:44:45] Wed May 30 15:50:27 UTC 2018 - building ...
---
[00:45:02]  ---> a91726edc50d
[00:45:02] Step 25/38 : RUN ./build-clang.sh
[00:45:02]  ---> Running in b7ce580df3f7
[00:45:02] + source shared.sh
[00:45:02] + LLVM=6.0.0
[00:45:02] + mkdir clang
[00:45:02] + cd clang
[00:45:02] + curl https://releases.llvm.org/6.0.0/llvm-6.0.0.src.tar.xz
[00:45:02] + xz -d
[00:45:02] + tar xf -
[00:45:02]                                  Dload  Upload   Total   Spent    Left  Speed
[00:45:05] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  3 24.1M    3  897k    0     0  1650k      0  0:00:14 --:--:--  0:00:14 3127k
  3 24.1M    3  897k    0     0  1650k      0  0:00:14 --:--:--  0:00:14 3127k
 41 24.1M   41 10.0M    0     0  6693k      0  0:00:03  0:00:01  0:00:02 8026k
 76 24.1M   76 18.3M    0     0  7395k      0  0:00:03  0:00:02  0:00:01 8227k
100 24.1M  100 24.1M    0     0  7693k      0  0:00:03  0:00:03 --:--:-- 8362k
[00:45:05] + cd llvm-6.0.0.src
[00:45:05] + mkdir -p tools/clang
[00:45:05] + curl https://releases.llvm.org/6.0.0/cfe-6.0.0.src.tar.xz
[00:45:05] + xz -d
[00:45:05] + tar xf - -C tools/clang --strip-components=1
[00:45:05]                                  Dload  Upload   Total   Spent    Left  Speed
[00:45:13] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  5 11.4M    5  591k    0     0  1854k      0  0:00:06 --:--:--  0:00:06 1860k
---
 90 11.4M   90 10.3M    0     0  1862k      0  0:00:06  0:00:05  0:00:01 1863k
 90 11.4M   90 10.3M    0     0  1594k      0  0:00:07  0:00:06  0:00:01 1442k
 90 11.4M   90 10.3M    0     0  1391k      0  0:00:08  0:00:07  0:00:01  671k
100 11.4M  100 11.4M    0     0  1453k      0  0:00:08  0:00:08 --:--:--  270k
[00:45:13] + mkdir -p tools/lld
[00:45:13] + curl https://releases.llvm.org/6.0.0/lld-6.0.0.src.tar.xz
[00:45:13] + xz -d
[00:45:13] + tar xf - -C tools/lld --strip-components=1
[00:45:13]                                  Dload  Upload   Total   Spent    Left  Speed
[00:45:14] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 20  772k   20  159k    0     0   587k      0  0:00:01 --:--:--  0:00:01  592k
 20  772k   20  159k    0     0   587k      0  0:00:01 --:--:--  0:00:01  592k
100  772k  100  772k    0     0  1901k      0 --:--:-- --:--:-- --:--:-- 1910k
[00:45:14] + mkdir ../clang-build
[00:45:14] + cd ../clang-build
[00:45:14] + INC=/rustroot/include
[00:45:14] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:45:14] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:45:14] + hide_output cmake ../llvm-6.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:45:44] Wed May 30 15:51:25 UTC 2018 - building ...
[00:45:46] + hide_output make -j10
[00:45:46] + set +x
[00:46:16] Wed May 30 15:51:57 UTC 2018 - building ...
---
[01:44:54] + hide_output make install
[01:44:54] + set +x
[01:45:08] shared.sh: line 11:  1366 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:45:08] + cd ../..
[01:45:08] + rm -rf clang
[01:45:10] ./build-clang.sh: line 70: 15287 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/clang/clang-build)
[01:45:30] Removing intermediate container b7ce580df3f7
[01:45:30] Step 26/38 : ENV CC clang CXX clang++
[01:45:30]  ---> Running in 14d865313565
[01:45:30]  ---> add45439e2ef
[01:45:30]  ---> add45439e2ef
[01:45:30] Removing intermediate container 14d865313565
[01:45:30] Step 27/38 : COPY dist-x86_64-linux/build-git.sh /tmp/
[01:45:30]  ---> 140014964a37
[01:45:30] Step 28/38 : RUN ./build-git.sh
[01:45:30]  ---> Running in 986741d494ed
[01:45:31] + source shared.sh
[01:45:31] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:45:31]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:45:31]                                  Dload  Upload   Total   Spent    Left  Speed
[01:45:31] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
100 62.6M  100 62.6M    0     0  9415k      0  0:00:06  0:00:06 --:--:-- 10.1M
[01:46:35] + cd linux-3.2.84
[01:46:35] + hide_output make mrproper
[01:46:35] + set +x
[01:46:36] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:46:40] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:46:40] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:46:40] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:46:40] + yes
[01:46:40] + yes
[01:46:40] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:46:40] + rm -rf linux-3.2.84
[01:46:41]  ---> b6c48e39883d
[01:46:41] Removing intermediate container fdc425225aa2
[01:46:41] Step 31/38 : COPY scripts/sccache.sh /scripts/
---
[01:46:44] Removing intermediate container d230bf8a89ef
[01:46:44] Step 33/38 : ENV HOSTS x86_64-unknown-linux-gnu
[01:46:44]  ---> Running in 2adf5abda795
[01:46:44]  ---> 3da298381dab
[01:46:44] Removing intermediate container 2adf5abda795
[01:46:44] Step 34/38 : ENV RUST_CONFIGURE_ARGS --enable-full-tools       --enable-sanitizers       --enable-profiler       --enable-compiler-docs       --set target.x86_64-unknown-linux-gnu.linker=clang       --set target.x86_64-unknown-linux-gnu.ar=/rustroot/bin/llvm-ar       --set target.x86_64-unknown-linux-gnu.ranlib=/rustroot/bin/llvm-ranlib
[01:46:44]  ---> Running in 23bb40f8177a
[01:46:44] Removing intermediate container 23bb40f8177a
[01:46:44] Step 35/38 : ENV SCRIPT python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[01:46:44]  ---> Running in c381dfa5fff4
[01:46:44]  ---> 5c6b813ff3b2
