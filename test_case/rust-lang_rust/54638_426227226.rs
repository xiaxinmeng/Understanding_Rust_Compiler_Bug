plain
[00:01:31] 
[00:01:31] Total download size: 4.9 M
[00:01:31] Downloading Packages:
[00:01:40] --------------------------------------------------------------------------------
[00:01:40] Total                                           558 kB/s | 4.9 MB     00:08     
[00:01:40] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:40] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:40] Running Transaction Test
[00:01:40] Finished Transaction Test
[00:01:40] Transaction Test Succeeded
[00:01:40] Running Transaction
---
[00:04:21] + hide_output make install
[00:04:21] + set +x
[00:04:39] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:39] + cd ..
[00:04:39] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:39] + rm -rf openssl-1.0.2k
[00:04:39] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:40] Removing intermediate container d1b7ce73c0db
[00:04:40] Step 14/39 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:40]  ---> bc92a6772e60
[00:04:40] Step 15/39 : RUN ./build-curl.sh
[00:04:40] Step 15/39 : RUN ./build-curl.sh
[00:04:40]  ---> Running in 4d4fc5d043fb
[00:04:40] + source shared.sh
[00:04:40] + VERSION=7.51.0
[00:04:40] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:42]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:42]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:44] 
  0 2509k    0 14215    0     0   8413      0  0:05:05  0:00:01  0:05:04  8413
  0 2509k    0 14215    0     0   8413      0  0:05:05  0:00:01  0:05:04  8413
  1 2509k    1 30599    0     0  16806      0  0:02:32  0:00:01  0:02:31  121k
 54 2509k   54 1373k    0     0   493k      0  0:00:05  0:00:02  0:00:03 1240k
100 2509k  100 2509k    0     0   803k      0  0:00:03  0:00:03 --:--:-- 1740k
[00:04:44] + mkdir curl-build
[00:04:44] + cd curl-build
[00:04:44] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:05] + hide_output make -j10
[00:05:05] + set +x
[00:05:17] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:17] + hide_output make install
---
 91 82.1M   91 75.2M    0     0  4461k      0  0:00:18  0:00:17  0:00:01 4228k
 96 82.1M   96 79.3M    0     0  4444k      0  0:00:18  0:00:18 --:--:-- 4105k
100 82.1M  100 82.1M    0     0  4443k      0  0:00:18  0:00:18 --:--:-- 4129k
[00:08:51] + cd gcc-4.8.5
[00:08:51] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:51] --2018-10-02 07:45:37--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:51] Resolving gcc.gnu.org... 209.132.180.131
[00:08:51] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:51] HTTP request sent, awaiting response... 200 OK
---
 12 24.1M   12 3039k    0     0  2998k      0  0:00:08  0:00:01  0:00:07 3823k
 50 24.1M   50 12.1M    0     0  6174k      0  0:00:04  0:00:02  0:00:02 6927k
 82 24.1M   82 19.8M    0     0  6744k      0  0:00:03  0:00:03 --:--:-- 7271k
100 24.1M  100 24.1M    0     0  7049k      0  0:00:03  0:00:03 --:--:-- 7519k
[00:42:36] + cd llvm-6.0.0.src
[00:42:36] + mkdir -p tools/clang
[00:42:36] + curl https://releases.llvm.org/6.0.0/cfe-6.0.0.src.tar.xz
[00:42:36] + xz -d
[00:42:36] + tar xf - -C tools/clang --strip-components=1
[00:42:36]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:37] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 26 11.4M   26 3039k    0     0  6147k      0  0:00:01 --:--:--  0:00:01 6165k
 26 11.4M   26 3039k    0     0  6147k      0  0:00:01 --:--:--  0:00:01 6165k
 90 11.4M   90 10.3M    0     0  7094k      0  0:00:01  0:00:01 --:--:-- 7098k
100 11.4M  100 11.4M    0     0  7359k      0  0:00:01  0:00:01 --:--:-- 7363k
[00:42:37] + mkdir -p tools/lld
[00:42:37] + curl https://releases.llvm.org/6.0.0/lld-6.0.0.src.tar.xz
[00:42:37] + xz -d
[00:42:37] + tar xf - -C tools/lld --strip-components=1
[00:42:37]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:38] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100  772k  100  772k    0     0  3471k      0 --:--:-- --:--:-- --:--:-- 3509k
100  772k  100  772k    0     0  3471k      0 --:--:-- --:--:-- --:--:-- 3509k
[00:42:38] + mkdir ../clang-build
[00:42:38] + cd ../clang-build
[00:42:38] + INC=/rustroot/include
[00:42:38] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:42:38] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:38] + hide_output cmake ../llvm-6.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:43:08] Tue Oct 2 08:19:54 UTC 2018 - building ...
[00:43:09] + hide_output make -j10
[00:43:09] + set +x
[00:43:39] Tue Oct 2 08:20:25 UTC 2018 - building ...
---
[01:40:02] + set +x
[01:40:03] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:40:03] + set +x
[01:40:08] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:40:08] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:40:08] + yes
[01:40:08] + yes
[01:40:08] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:40:08] + rm -rf linux-3.2.84
[01:40:09]  ---> fd616dee4511
[01:40:09] Removing intermediate container 3b75761feda7
[01:40:09] Step 31/39 : COPY scripts/sccache.sh /scripts/
---
[01:42:43] configure: rust.codegen-backends := ['llvm', 'emscripten']
[01:42:43] configure: rust.lld             := True
[01:42:43] configure: rust.llvm-tools      := True
[01:42:43] configure: build.extended       := True
[01:42:43] configure: dist.missing-tools   := True
[01:42:43] configure: 
[01:42:43] configure: writing `config.toml` in current directory
[01:42:43] configure: 
[01:42:43] configure: run `python /checkout/x.py --help`
---
[02:59:16] [RUSTC-TIMING] serialize test:false 11.203
[02:59:19]    Compiling syn v0.13.11
[02:59:22] [RUSTC-TIMING] rustc_rayon test:false 5.508
[02:59:22]    Compiling rustc-ap-rustc_data_structures v263.0.0
The job exceeded the maximum time limit for jobs, and has been terminated.
