plain
[00:00:51] 
[00:00:51] Total download size: 4.9 M
[00:00:51] Downloading Packages:
[00:01:00] --------------------------------------------------------------------------------
[00:01:00] Total                                           565 kB/s | 4.9 MB     00:08     
[00:01:00] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:00] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:00] Running Transaction Test
[00:01:00] Finished Transaction Test
[00:01:00] Transaction Test Succeeded
[00:01:00] Running Transaction
---
[00:01:03] --> Processing Dependency: libc.so.6(GLIBC_2.2) for package: curl
[00:01:03] --> Processing Dependency: libidn.so.11 for package: curl
[00:01:03] --> Processing Dependency: libz.so.1 for package: curl
[00:01:03] --> Processing Dependency: libc.so.6 for package: curl
[00:01:03] --> Processing Dependency: libdl.so.2 for package: curl
[00:01:03] --> Processing Dependency: libgssapi_krb5.so.2(gssapi_krb5_2_MIT) for package: curl
[00:01:03] --> Processing Dependency: libc.so.6(GLIBC_2.3.4) for package: curl
[00:01:03] --> Processing Dependency: libk5crypto.so.3 for package: curl
[00:01:03] --> Processing Dependency: libc.so.6(GLIBC_2.1) for package: curl
[00:01:03] --> Processing Dependency: libssl.so.6 for package: curl
---
[00:01:03] --> Processing Dependency: glibc-headers = 2.5-123.el5_11.3 for package: glibc-devel
[00:01:03] --> Processing Dependency: glibc-headers for package: glibc-devel
[00:01:03] ---> Package glibc-devel.x86_64 0:2.5-123.el5_11.3 set to be updated
[00:01:03] ---> Package make.x86_64 1:3.81-3.el5 set to be updated
[00:01:03] ---> Package perl.i386 4:5.8.8-43.el5_11 set to be updated
[00:01:03] --> Processing Dependency: libgdbm.so.2 for package: perl
[00:01:03] --> Processing Dependency: libdb-4.3.so for package: perl
[00:01:03] ---> Package perl.x86_64 4:5.8.8-43.el5_11 set to be updated
[00:01:04] ---> Package pkgconfig.x86_64 1:0.21-2.el5 set to be updated
[00:01:04] ---> Package which.x86_64 0:2.16-7 set to be updated
[00:01:04] ---> Package xz.x86_64 0:4.999.9-0.3.beta.20091007git.el5 set to be updated
[00:01:04] --> Processing Dependency: xz-libs = 4.999.9-0.3.beta.20091007git.el5 for package: xz
[00:01:04] --> Processing Dependency: xz-libs = 4.999.9-0.3.beta.20091007git.el5 for package: xz
[00:01:04] --> Processing Dependency: liblzma.so.0()(64bit) for package: xz
[00:01:04] ---> Package zlib-devel.x86_64 0:1.2.3-7.el5 set to be updated
[00:01:04] --> Running transaction check
[00:01:04] ---> Package cpp.x86_64 0:4.1.2-55.el5 set to be updated
[00:01:04] ---> Package db4.i386 0:4.3.29-10.el5_5.2 set to be updated
[00:01:04] ---> Package db4.i386 0:4.3.29-10.el5_5.2 set to be updated
[00:01:04] --> Processing Dependency: libstdc++.so.6(GLIBCXX_3.4) for package: db4
[00:01:04] --> Processing Dependency: libstdc++.so.6(CXXABI_1.3) for package: db4
[00:01:04] --> Processing Dependency: libstdc++.so.6 for package: db4
[00:01:04] ---> Package e2fsprogs-libs.i386 0:1.39-37.el5 set to be updated
[00:01:04] --> Processing Dependency: libdevmapper.so.1.02 for package: e2fsprogs-libs
[00:01:04] ---> Package gdbm.i386 0:1.8.0-28.el5 set to be updated
[00:01:04] ---> Package glibc.i686 0:2.5-123.el5_11.3 set to be updated
[00:01:04] --> Processing Dependency: kernel-headers >= 2.2.1 for package: glibc-headers
[00:01:04] --> Processing Dependency: kernel-headers for package: glibc-headers
[00:01:04] ---> Package imake.x86_64 0:1.0.2-3 set to be updated
[00:01:04] ---> Package krb5-libs.i386 0:1.6.1-80.el5_11 set to be updated
---
[00:03:43] + hide_output make install
[00:03:43] + set +x
[00:04:02] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:02] + cd ..
[00:04:02] + rm -rf openssl-1.0.2k
[00:04:02] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:02] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:03] Removing intermediate container 8287466148b9
[00:04:03] Step 14/38 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:03]  ---> 4486358c5cce
[00:04:03] Step 15/38 : RUN ./build-curl.sh
[00:04:03] Step 15/38 : RUN ./build-curl.sh
[00:04:03]  ---> Running in 98a8e1f1fa9e
[00:04:03] + source shared.sh
[00:04:03] + VERSION=7.51.0
[00:04:03] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:05]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:05]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:06] 
  0 2509k    0 14215    0     0   9207      0  0:04:39  0:00:01  0:04:38  9207
  0 2509k    0 14215    0     0   9207      0  0:04:39  0:00:01  0:04:38  9207
 18 2509k   18  454k    0     0   206k      0  0:00:12  0:00:02  0:00:10  668k
100 2509k  100 2509k    0     0   899k      0  0:00:02  0:00:02 --:--:-- 2001k
[00:04:06] + mkdir curl-build
[00:04:06] + cd curl-build
[00:04:06] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:28] + hide_output make -j10
[00:04:28] + set +x
[00:04:40] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:40] + hide_output make install
---
[00:05:35]  ---> ebaa534bb196
[00:05:35] Step 19/38 : RUN ./build-cmake.sh
[00:05:35]  ---> Running in fb519c2dde88
[00:05:35] + source shared.sh
[00:05:35] + curl https://cmake.org/files/v3.6/cmake-3.6.3.tar.gz
[00:05:35]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:05:35]                                  Dload  Upload   Total   Spent    Left  Speed
[00:05:36] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
 92 82.1M   92 75.6M    0     0  3670k      0  0:00:22  0:00:21  0:00:01 3066k
 96 82.1M   96 79.1M    0     0  3664k      0  0:00:22  0:00:22 --:--:-- 3237k
100 82.1M  100 82.1M    0     0  3647k      0  0:00:23  0:00:23 --:--:-- 3318k
[00:08:26] + cd gcc-4.8.5
[00:08:26] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:26] --2018-07-07 11:18:58--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:26] Resolving gcc.gnu.org... 209.132.180.131
[00:08:26] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:26] HTTP request sent, awaiting response... 200 OK
---
[00:08:29]   1850K ...                                                   100% 5743G=1.6s
[00:08:29] 
[00:08:29] 2018-07-07 11:19:01 (1.16 MB/s) - `gmp-4.3.2.tar.bz2' saved [1897483/1897483]
[00:08:29] 
[00:08:29] --2018-07-07 11:19:01--  http://gcc.gnu.org/pub/gcc/infrastructure/mpc-0.8.1.tar.gz
[00:08:29] Resolving gcc.gnu.org... 209.132.180.131
[00:08:29] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:29] HTTP request sent, awaiting response... 200 OK
[00:08:29] Length: 544950 (532K) [application/x-gzip]
[00:08:29] Saving to: `mpc-0.8.1.tar.gz'
[00:08:30]      0K .......... .......... .......... .......... ..........  9%  292K 2s
[00:08:30]     50K .......... .......... .......... .......... .......... 18%  437K 1s
[00:08:30]    100K .......... .......... .......... .......... .......... 28%  872K 1s
[00:08:30]    150K .......... .......... .......... .......... .......... 37%  872K 1s
---
[00:41:16]  ---> e6a029c20a54
[00:41:16] Step 23/38 : RUN ./build-python.sh
[00:41:16]  ---> Running in 984155b92f14
[00:41:16] + source shared.sh
[00:41:16] + curl https://www.python.org/ftp/python/2.7.12/Python-2.7.12.tgz
[00:41:16]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:41:16]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:17] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 23 16.1M   23 3967k    0     0  5707k      0  0:00:02 --:--:--  0:00:02 5947k
100 16.1M  100 16.1M    0     0  13.7M      0  0:00:01  0:00:01 --:--:-- 14.1M
[00:41:17] + mkdir python-build
[00:41:17] + cd python-build
[00:41:17] + CFLAGS='-I /rustroot/include'
[00:41:17] + LDFLAGS='-L /rustroot/lib -L /rustroot/lib64'
[00:41:17] + hide_output ../Python-2.7.12/configure --prefix=/rustroot
[00:41:32] + hide_output make -j10
[00:41:32] + set +x
[00:42:02] Sat Jul 7 11:52:34 UTC 2018 - building ...
[00:42:32] Sat Jul 7 11:53:04 UTC 2018 - building ...
---
[00:42:48]  ---> 71a18d59b21a
[00:42:48] Step 25/38 : RUN ./build-clang.sh
[00:42:48]  ---> Running in fbc5e3546714
[00:42:49] + source shared.sh
[00:42:49] + LLVM=6.0.0
[00:42:49] + mkdir clang
[00:42:49] + cd clang
[00:42:49] + curl https://releases.llvm.org/6.0.0/llvm-6.0.0.src.tar.xz
[00:42:49] + xz -d
[00:42:49] + tar xf -
[00:42:49]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:52] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 25 24.1M   25 6329k    0     0  6375k      0  0:00:03 --:--:--  0:00:03 7580k
 25 24.1M   25 6329k    0     0  6375k      0  0:00:03 --:--:--  0:00:03 7580k
 63 24.1M   63 15.2M    0     0  7836k      0  0:00:03  0:00:01  0:00:02 8513k
 96 24.1M   96 23.3M    0     0  8001k      0  0:00:03  0:00:02  0:00:01 8447k
100 24.1M  100 24.1M    0     0  8048k      0  0:00:03  0:00:03 --:--:-- 8486k
[00:42:52] + cd llvm-6.0.0.src
[00:42:52] + mkdir -p tools/clang
[00:42:52] + curl https://releases.llvm.org/6.0.0/cfe-6.0.0.src.tar.xz
[00:42:52] + tar xf - -C tools/clang --strip-components=1
[00:42:52] + xz -d
[00:42:52]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:53] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 56 11.4M   56 6621k    0     0  7279k      0  0:00:01 --:--:--  0:00:01 7292k
 56 11.4M   56 6621k    0     0  7279k      0  0:00:01 --:--:--  0:00:01 7292k
100 11.4M  100 11.4M    0     0  7346k      0  0:00:01  0:00:01 --:--:-- 7359k
[00:42:53] + mkdir ../clang-build
[00:42:53] + cd ../clang-build
[00:42:53] + INC=/rustroot/include
[00:42:53] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:42:53] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:53] + hide_output cmake ../llvm-6.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:43:23] Sat Jul 7 11:53:55 UTC 2018 - building ...
[00:43:23] + hide_output make -j10
[00:43:23] + set +x
[00:43:53] Sat Jul 7 11:54:25 UTC 2018 - building ...
---
[01:33:58] + hide_output make install
[01:33:58] + set +x
[01:34:10] shared.sh: line 11:  1360 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:34:10] + cd ../..
[01:34:10] + rm -rf clang
[01:34:11] ./build-clang.sh: line 64: 14545 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/clang/clang-build)
[01:34:30] Removing intermediate container fbc5e3546714
[01:34:30] Step 26/38 : ENV CC clang CXX clang++
[01:34:33]  ---> Running in a5fc9048b713
[01:34:34]  ---> 1d29ecd19724
[01:34:34]  ---> 1d29ecd19724
[01:34:34] Removing intermediate container a5fc9048b713
[01:34:34] Step 27/38 : COPY dist-x86_64-linux/build-git.sh /tmp/
[01:34:34]  ---> 0de9f14be89b
[01:34:34] Step 28/38 : RUN ./build-git.sh
[01:34:34]  ---> Running in 8a6cb208c40e
[01:34:34] + source shared.sh
[01:34:34] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:34:34]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:34:34]                                  Dload  Upload   Total   Spent    Left  Speed
[01:34:34] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
100 62.6M  100 62.6M    0     0  8656k      0  0:00:07  0:00:07 --:--:-- 9337k
[01:35:31] + cd linux-3.2.84
[01:35:31] + hide_output make mrproper
[01:35:31] + set +x
[01:35:32] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:35:35] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:35:35] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:35:35] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:35:35] + yes
[01:35:35] + yes
[01:35:35] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:35:35] + rm -rf linux-3.2.84
[01:35:36]  ---> 5baa1f41369e
[01:35:36] Removing intermediate container f2d97c59ac44
[01:35:36] Step 31/38 : COPY scripts/sccache.sh /scripts/
---
[02:58:36]    Compiling rustc-ap-rustc_errors v182.0.0
[02:58:43] [RUSTC-TIMING] rls_analysis test:false 18.240
[02:58:47] [RUSTC-TIMING] rustc_errors test:false 11.726
[02:58:47]    Compiling rustc-ap-syntax v182.0.0
The job exceeded the maximum time limit for jobs, and has been terminated.
