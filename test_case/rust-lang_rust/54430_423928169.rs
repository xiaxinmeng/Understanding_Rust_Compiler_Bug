plain
[00:01:23] 
[00:01:23] Total download size: 4.9 M
[00:01:23] Downloading Packages:
[00:01:23] --------------------------------------------------------------------------------
[00:01:23] Total                                            11 MB/s | 4.9 MB     00:00     
[00:01:23] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:23] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:23] Running Transaction Test
[00:01:23] Finished Transaction Test
[00:01:23] Transaction Test Succeeded
[00:01:23] Running Transaction
---
[00:02:57] + hide_output make install
[00:02:57] + set +x
[00:03:15] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:15] + cd ..
[00:03:15] + rm -rf openssl-1.0.2k
[00:03:15] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:03:15] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:03:15] Removing intermediate container a47e17a12f9d
[00:03:15] Step 14/38 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:03:16]  ---> 2985648128e3
[00:03:16] Step 15/38 : RUN ./build-curl.sh
[00:03:16] Step 15/38 : RUN ./build-curl.sh
[00:03:16]  ---> Running in a1416ae1077a
[00:03:16] + source shared.sh
[00:03:16] + VERSION=7.51.0
[00:03:16] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:03:17]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:03:17]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:19] 
  0 2509k    0 14215    0     0   9155      0  0:04:40  0:00:01  0:04:39  9155
  0 2509k    0 14215    0     0   9155      0  0:04:40  0:00:01  0:04:39  9155
  1 2509k    1 30599    0     0  18170      0  0:02:21  0:00:01  0:02:20  121k
 75 2509k   75 1906k    0     0   739k      0  0:00:03  0:00:02  0:00:01 1846k
100 2509k  100 2509k    0     0   922k      0  0:00:02  0:00:02 --:--:-- 2135k
[00:03:19] + mkdir curl-build
[00:03:19] + cd curl-build
[00:03:19] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:03:40] + hide_output make -j10
[00:03:40] + set +x
[00:03:52] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:52] + hide_output make install
---
 91 82.1M   91 75.1M    0     0  3053k      0  0:00:27  0:00:25  0:00:02 3609k
 96 82.1M   96 78.9M    0     0  3083k      0  0:00:27  0:00:26  0:00:01 3532k
100 82.1M  100 82.1M    0     0  3112k      0  0:00:27  0:00:27 --:--:-- 3694k
[00:07:37] + cd gcc-4.8.5
[00:07:37] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:07:37] --2018-09-24 07:15:56--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:07:37] Resolving gcc.gnu.org... 209.132.180.131
[00:07:37] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:07:37] HTTP request sent, awaiting response... 200 OK
---
  3 24.1M    3  943k    0     0  1948k      0  0:00:12 --:--:--  0:00:12 3867k
 43 24.1M   43 10.5M    0     0  7274k      0  0:00:03  0:00:01  0:00:02 8681k
 77 24.1M   77 18.6M    0     0  7685k      0  0:00:03  0:00:02  0:00:01 8505k
100 24.1M  100 24.1M    0     0  7888k      0  0:00:03  0:00:03 --:--:-- 8544k
[00:41:23] + cd llvm-6.0.0.src
[00:41:23] + mkdir -p tools/clang
[00:41:23] + curl https://releases.llvm.org/6.0.0/cfe-6.0.0.src.tar.xz
[00:41:23] + tar xf - -C tools/clang --strip-components=1
[00:41:23] + xz -d
[00:41:23]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:25] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 15 11.4M   15 1796k    0     0  5249k      0  0:00:02 --:--:--  0:00:02 5268k
 15 11.4M   15 1796k    0     0  5249k      0  0:00:02 --:--:--  0:00:02 5268k
 74 11.4M   74 8724k    0     0  6498k      0  0:00:01  0:00:01 --:--:-- 6501k
100 11.4M  100 11.4M    0     0  6914k      0  0:00:01  0:00:01 --:--:-- 6918k
[00:41:25] + mkdir -p tools/lld
[00:41:25] + curl https://releases.llvm.org/6.0.0/lld-6.0.0.src.tar.xz
[00:41:25] + xz -d
[00:41:25] + tar xf - -C tools/lld --strip-components=1
[00:41:25]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:25] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100  772k  100  772k    0     0  3337k      0 --:--:-- --:--:-- --:--:-- 3371k
100  772k  100  772k    0     0  3337k      0 --:--:-- --:--:-- --:--:-- 3371k
[00:41:25] + mkdir ../clang-build
[00:41:25] + cd ../clang-build
[00:41:25] + INC=/rustroot/include
[00:41:25] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:41:25] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:41:25] + hide_output cmake ../llvm-6.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:41:54] + hide_output make -j10
[00:41:54] + set +x
[00:42:24] Mon Sep 24 07:50:43 UTC 2018 - building ...
[00:42:54] Mon Sep 24 07:51:13 UTC 2018 - building ...
---
[01:34:35] + set +x
[01:34:36] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:34:36] + set +x
[01:34:39] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:34:39] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:34:39] + yes
[01:34:39] + yes
[01:34:39] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:34:39] + rm -rf linux-3.2.84
[01:34:40]  ---> ee2161c255e7
[01:34:40] Removing intermediate container d3280820c46c
[01:34:40] Step 31/38 : COPY scripts/sccache.sh /scripts/
---
[02:59:14] [RUSTC-TIMING] rustc_traits test:false 1.210
[02:59:14]  Documenting rustc_incremental v0.0.0 (file:///checkout/src/librustc_incremental)
[02:59:19]  Documenting rustc_allocator v0.0.0 (file:///checkout/src/librustc_allocator)

Broadcast message from root@travis-job-5b394834-3e18-42c0-a982-ff33423108f7
 (unknown) at 10:07 ...
The system is going down for power off NOW!
[02:59:23] Session terminated, killing shell... Documenting rustc_traits v0.0.0 (file:///checkout/src/librustc_traits)
