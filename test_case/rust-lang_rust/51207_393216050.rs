plain
    apt:
      update: true
travis_fold:start:git.checkout
travis_time:start:0770d300
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:00:23] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
[00:00:23] curl: (22) The requested URL returned error: 404 Not Found
[00:00:23] The command has failed after 5 attempts.
[00:00:23] open /tmp/rustci_docker_cache: no such file or directory
[00:00:23] Attempting with retry: docker build --rm -t rust-ci -f /home/travis/build/rust-lang/rust/src/ci/docker/dist-x86_64-linux/Dockerfile /home/travis/build/rust-lang/rust/src/ci/docker
[00:00:24] Sending build context to Docker daemon  501.8kB
[00:00:24] Step 1/38 : FROM centos:5
[00:00:24] 5: Pulling from library/centos
---
[00:00:48] 
[00:00:48] Total download size: 4.9 M
[00:00:48] Downloading Packages:
[00:00:56] --------------------------------------------------------------------------------
[00:00:56] Total                                           570 kB/s | 4.9 MB     00:08     
[00:00:56] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:00:56] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:00:56] Running Transaction Test
[00:00:57] Finished Transaction Test
[00:00:57] Transaction Test Succeeded
[00:00:57] Running Transaction
---
[00:00:59] --> Processing Dependency: libc.so.6(GLIBC_2.2) for package: curl
[00:00:59] --> Processing Dependency: libidn.so.11 for package: curl
[00:00:59] --> Processing Dependency: libz.so.1 for package: curl
[00:00:59] --> Processing Dependency: libc.so.6 for package: curl
[00:00:59] --> Processing Dependency: libdl.so.2 for package: curl
[00:00:59] --> Processing Dependency: libgssapi_krb5.so.2(gssapi_krb5_2_MIT) for package: curl
[00:00:59] --> Processing Dependency: libc.so.6(GLIBC_2.3.4) for package: curl
[00:00:59] --> Processing Dependency: libk5crypto.so.3 for package: curl
[00:00:59] --> Processing Dependency: libc.so.6(GLIBC_2.1) for package: curl
[00:00:59] --> Processing Dependency: libssl.so.6 for package: curl
---
[00:00:59] --> Processing Dependency: glibc-headers = 2.5-123.el5_11.3 for package: glibc-devel
[00:00:59] --> Processing Dependency: glibc-headers for package: glibc-devel
[00:00:59] ---> Package glibc-devel.x86_64 0:2.5-123.el5_11.3 set to be updated
[00:00:59] ---> Package make.x86_64 1:3.81-3.el5 set to be updated
[00:00:59] ---> Package perl.i386 4:5.8.8-43.el5_11 set to be updated
[00:00:59] --> Processing Dependency: libgdbm.so.2 for package: perl
[00:00:59] --> Processing Dependency: libdb-4.3.so for package: perl
[00:00:59] ---> Package perl.x86_64 4:5.8.8-43.el5_11 set to be updated
[00:00:59] ---> Package pkgconfig.x86_64 1:0.21-2.el5 set to be updated
[00:00:59] ---> Package which.x86_64 0:2.16-7 set to be updated
[00:00:59] ---> Package xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
[00:00:59] --> Processing Dependency: xz-libs = 4.999.9-0.3.beta.20091007git.el5 for package: xz
[00:00:59] --> Processing Dependency: xz-libs = 4.999.9-0.3.beta.20091007git.el5 for package: xz
[00:00:59] --> Processing Dependency: liblzma.so.0()(64bit) for package: xz
[00:00:59] ---> Package zlib-devel.x86_64 0:1.2.3-7.el5 set to be updated
[00:00:59] --> Running transaction check
[00:00:59] ---> Package cpp.x86_64 0:4.1.2-55.el5 set to be updated
[00:00:59] ---> Package db4.i386 0:4.3.29-10.el5_5.2 set to be updated
[00:00:59] ---> Package db4.i386 0:4.3.29-10.el5_5.2 set to be updated
[00:00:59] --> Processing Dependency: libstdc++.so.6(GLIBCXX_3.4) for package: db4
[00:00:59] --> Processing Dependency: libstdc++.so.6(CXXABI_1.3) for package: db4
[00:00:59] --> Processing Dependency: libstdc++.so.6 for package: db4
[00:00:59] ---> Package e2fsprogs-libs.i386 0:1.39-37.el5 set to be updated
[00:00:59] --> Processing Dependency: libdevmapper.so.1.02 for package: e2fsprogs-libs
[00:00:59] ---> Package gdbm.i386 0:1.8.0-28.el5 set to be updated
[00:00:59] ---> Package glibc.i686 0:2.5-123.el5_11.3 set to be updated
[00:00:59] --> Processing Dependency: kernel-headers >= 2.2.1 for package: glibc-headers
[00:00:59] --> Processing Dependency: kernel-headers for package: glibc-headers
[00:00:59] ---> Package imake.x86_64 0:1.0.2-3 set to be updated
[00:00:59] ---> Package krb5-libs.i386 0:1.6.1-80.el5_11 set to be updated
---
[00:03:37] + hide_output make install
[00:03:37] + set +x
[00:03:54] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:54] + cd ..
[00:03:54] + rm -rf openssl-1.0.2k
[00:03:55] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:03:55] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:03:55] Removing intermediate container 88344f84846b
[00:03:55] Step 14/38 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:03:55]  ---> 7f5d6ea08cfb
[00:03:55] Step 15/38 : RUN ./build-curl.sh
[00:03:55] Step 15/38 : RUN ./build-curl.sh
[00:03:55]  ---> Running in f7592835002d
[00:03:55] + source shared.sh
[00:03:55] + VERSION=7.51.0
[00:03:55] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:03:57]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:03:57]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:58] 
  0 2509k    0  9871    0     0   5874      0  0:07:17  0:00:01  0:07:16  5874
  0 2509k    0  9871    0     0   5874      0  0:07:17  0:00:01  0:07:16  5874
  1 2509k    1 30599    0     0  16877      0  0:02:32  0:00:01  0:02:31  152k
 91 2509k   91 2290k    0     0   817k      0  0:00:03  0:00:02  0:00:01 2029k
100 2509k  100 2509k    0     0   882k      0  0:00:02  0:00:02 --:--:-- 2147k
[00:03:58] + mkdir curl-build
[00:03:58] + cd curl-build
[00:03:58] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:18] + hide_output make -j10
[00:04:18] + set +x
[00:04:29] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:29] + hide_output make install
---
[00:05:26]  ---> 33cf7782fef7
[00:05:26] Step 19/38 : RUN ./build-cmake.sh
[00:05:26]  ---> Running in 2d6b19edda0a
[00:05:27] + source shared.sh
[00:05:27] + curl https://cmake.org/files/v3.6/cmake-3.6.3.tar.gz
[00:05:27]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:05:27]                                  Dload  Upload   Total   Spent    Left  Speed
[00:05:28] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
 93 82.1M   93 76.7M    0     0  4032k      0  0:00:20  0:00:19  0:00:01 3525k
 97 82.1M   97 80.4M    0     0  4018k      0  0:00:20  0:00:20 --:--:-- 3512k
100 82.1M  100 82.1M    0     0  3990k      0  0:00:21  0:00:21 --:--:-- 3431k
[00:08:12] + cd gcc-4.8.5
[00:08:12] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:12] --2018-05-30 13:05:23--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:12] Resolving gcc.gnu.org... 209.132.180.131
[00:08:12] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:12] HTTP request sent, awaiting response... 200 OK
---
[00:08:16]   1850K ...                                                   100% 5743G=1.3s
[00:08:16] 
[00:08:16] 2018-05-30 13:05:27 (1.44 MB/s) - `gmp-4.3.2.tar.bz2' saved [1897483/1897483]
[00:08:16] 
[00:08:16] --2018-05-30 13:05:27--  http://gcc.gnu.org/pub/gcc/infrastructure/mpc-0.8.1.tar.gz
[00:08:16] Resolving gcc.gnu.org... 209.132.180.131
[00:08:16] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:17] HTTP request sent, awaiting response... 200 OK
[00:08:17] Length: 544950 (532K) [application/x-gzip]
[00:08:17] Saving to: `mpc-0.8.1.tar.gz'
[00:08:17]      0K .......... .......... .......... .......... ..........  9%  289K 2s
[00:08:17]     50K .......... .......... .......... .......... .......... 18%  434K 1s
[00:08:17]    100K .......... .......... .......... .......... .......... 28%  853K 1s
[00:08:17]    150K .......... .......... .......... .......... .......... 37%  818K 1s
---
[00:40:55]  ---> 68953dc8006f
[00:40:55] Step 23/38 : RUN ./build-python.sh
[00:40:55]  ---> Running in cbe86ac7632a
[00:40:55] + source shared.sh
[00:40:55] + curl https://www.python.org/ftp/python/2.7.12/Python-2.7.12.tgz
[00:40:55]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:40:55]                                  Dload  Upload   Total   Spent    Left  Speed
[00:40:57] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 15 16.1M   15 2492k    0     0  3927k      0  0:00:04 --:--:--  0:00:04 4019k
100 16.1M  100 16.1M    0     0  13.6M      0  0:00:01  0:00:01 --:--:-- 13.8M
[00:40:57] + mkdir python-build
[00:40:57] + cd python-build
[00:40:57] + CFLAGS='-I /rustroot/include'
[00:40:57] + LDFLAGS='-L /rustroot/lib -L /rustroot/lib64'
[00:40:57] + hide_output ../Python-2.7.12/configure --prefix=/rustroot
[00:41:09] + hide_output make -j10
[00:41:09] + set +x
[00:41:39] Wed May 30 13:38:50 UTC 2018 - building ...
[00:42:09] Wed May 30 13:39:20 UTC 2018 - building ...
---
[00:42:21]  ---> 9774a5de1e3f
[00:42:21] Step 25/38 : RUN ./build-clang.sh
[00:42:21]  ---> Running in 7ff1ce1592dd
[00:42:21] + source shared.sh
[00:42:21] + LLVM=6.0.0
[00:42:21] + mkdir clang
[00:42:21] + cd clang
[00:42:21] + curl https://releases.llvm.org/6.0.0/llvm-6.0.0.src.tar.xz
[00:42:21] + xz -d
[00:42:21] + tar xf -
[00:42:21]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:24] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 16 24.1M   16 4192k    0     0  4355k      0  0:00:05 --:--:--  0:00:05 5734k
 16 24.1M   16 4192k    0     0  4355k      0  0:00:05 --:--:--  0:00:05 5734k
 58 24.1M   58 14.1M    0     0  7397k      0  0:00:03  0:00:01  0:00:02 8388k
 94 24.1M   94 22.9M    0     0  7917k      0  0:00:03  0:00:02  0:00:01 8588k
100 24.1M  100 24.1M    0     0  8024k      0  0:00:03  0:00:03 --:--:-- 8676k
[00:42:24] + cd llvm-6.0.0.src
[00:42:24] + mkdir -p tools/clang
[00:42:24] + curl https://releases.llvm.org/6.0.0/cfe-6.0.0.src.tar.xz
[00:42:24] + tar xf - -C tools/clang --strip-components=1
[00:42:24] + xz -d
[00:42:24]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:26] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 56 11.4M   56 6545k    0     0  7510k      0  0:00:01 --:--:--  0:00:01 7523k
 56 11.4M   56 6545k    0     0  7510k      0  0:00:01 --:--:--  0:00:01 7523k
100 11.4M  100 11.4M    0     0  7723k      0  0:00:01  0:00:01 --:--:-- 7734k
[00:42:26] + mkdir -p tools/lld
[00:42:26] + curl https://releases.llvm.org/6.0.0/lld-6.0.0.src.tar.xz
[00:42:26] + xz -d
[00:42:26] + tar xf - -C tools/lld --strip-components=1
[00:42:26]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:26] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100  772k  100  772k    0     0  2509k      0 --:--:-- --:--:-- --:--:-- 2531k
100  772k  100  772k    0     0  2509k      0 --:--:-- --:--:-- --:--:-- 2531k
[00:42:26] + mkdir ../clang-build
[00:42:26] + cd ../clang-build
[00:42:26] + INC=/rustroot/include
[00:42:26] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:42:26] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:26] + hide_output cmake ../llvm-6.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:55] + hide_output make -j10
[00:42:55] + set +x
[00:43:25] Wed May 30 13:40:36 UTC 2018 - building ...
[00:43:55] Wed May 30 13:41:06 UTC 2018 - building ...
---
[01:36:26] + hide_output make install
[01:36:26] + set +x
[01:36:38] shared.sh: line 11:  1363 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:36:38] + cd ../..
[01:36:38] + rm -rf clang
[01:36:40] ./build-clang.sh: line 70: 15247 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/clang/clang-build)
[01:36:46] Removing intermediate container 7ff1ce1592dd
[01:36:46] Step 26/38 : ENV CC clang CXX clang++
[01:36:46]  ---> Running in 89ac307f1a53
[01:36:46]  ---> 503510e13433
[01:36:46]  ---> 503510e13433
[01:36:46] Removing intermediate container 89ac307f1a53
[01:36:46] Step 27/38 : COPY dist-x86_64-linux/build-git.sh /tmp/
[01:36:46]  ---> 5edc6f11cedf
[01:36:46] Step 28/38 : RUN ./build-git.sh
[01:36:46]  ---> Running in d4a51426bda2
[01:36:47] + source shared.sh
[01:36:47] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:36:47]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:36:47]                                  Dload  Upload   Total   Spent    Left  Speed
[01:36:47] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
100 62.6M  100 62.6M    0     0  5027k      0  0:00:12  0:00:12 --:--:-- 10.5M
[01:37:51] + cd linux-3.2.84
[01:37:51] + hide_output make mrproper
[01:37:51] + set +x
[01:37:52] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:37:55] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:37:55] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:37:55] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:37:55] + yes
[01:37:55] + yes
[01:37:55] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:37:55] + rm -rf linux-3.2.84
[01:37:56]  ---> 109e75a5971e
[01:37:56] Removing intermediate container 2980fe4ca40b
[01:37:56] Step 31/38 : COPY scripts/sccache.sh /scripts/
---
[02:57:00]    Compiling crates-io v0.17.0 (file:///checkout/src/tools/cargo/src/crates-io)
[02:57:04]    Compiling git2 v0.7.1
[02:57:13]    Compiling git2-curl v0.8.1
[02:57:14]    Compiling cargo v0.29.0 (file:///checkout/src/tools/cargo)
The job exceeded the maximum time limit for jobs, and has been terminated.
