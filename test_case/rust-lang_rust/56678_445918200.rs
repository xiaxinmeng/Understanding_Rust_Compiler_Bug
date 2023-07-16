plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:004206c0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:00:48] 
[00:00:48] Total download size: 4.9 M
[00:00:48] Downloading Packages:
[00:00:51] --------------------------------------------------------------------------------
[00:00:51] Total                                           1.8 MB/s | 4.9 MB     00:02     
[00:00:51] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:00:51] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:00:51] Running Transaction Test
[00:00:51] Finished Transaction Test
[00:00:51] Transaction Test Succeeded
[00:00:51] Running Transaction
---
[00:03:07] + hide_output make install
[00:03:07] + set +x
[00:03:25] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:25] + cd ..
[00:03:25] + rm -rf openssl-1.0.2k
[00:03:25] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:03:25] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:03:26] Removing intermediate container e0a6287fa86f
[00:03:26] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:03:26]  ---> f2f711068964
[00:03:26] Step 15/41 : RUN ./build-curl.sh
[00:03:26] Step 15/41 : RUN ./build-curl.sh
[00:03:26]  ---> Running in 24cce2d607f7
[00:03:26] + source shared.sh
[00:03:26] + VERSION=7.51.0
[00:03:26] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:03:28]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:03:28]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:29] 
  0 2509k    0 14215    0     0   9439      0  0:04:32  0:00:01  0:04:31  9439
  0 2509k    0 14215    0     0   9439      0  0:04:32  0:00:01  0:04:31  9439
  2 2509k    2 59559    0     0  33949      0  0:01:15  0:00:01  0:01:14  177k
100 2509k  100 2509k    0     0   939k      0  0:00:02  0:00:02 --:--:-- 2140k
[00:03:29] + mkdir curl-build
[00:03:29] + cd curl-build
[00:03:29] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:03:49] + hide_output make -j10
[00:03:49] + set +x
[00:04:02] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:02] + hide_output make install
---
 98 82.1M   98 80.9M    0     0  1004k      0  0:01:23  0:01:22  0:00:01 1101k
 99 82.1M   99 82.0M    0     0  1006k      0  0:01:23  0:01:23 --:--:-- 1079k
100 82.1M  100 82.1M    0     0  1006k      0  0:01:23  0:01:23 --:--:-- 1181k
[00:08:56] + cd gcc-4.8.5
[00:08:56] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:56] --2018-12-10 15:30:50--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:56] Resolving gcc.gnu.org... 209.132.180.131
[00:08:56] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:56] HTTP request sent, awaiting response... 200 OK
---
 29 27.0M   29 8128k    0     0  6610k      0  0:00:04  0:00:01  0:00:03 6762k
 61 27.0M   61 16.4M    0     0  7572k      0  0:00:03  0:00:02  0:00:01 7665k
 89 27.0M   89 24.0M    0     0  7636k      0  0:00:03  0:00:03 --:--:-- 7701k
100 27.0M  100 27.0M    0     0  7873k      0  0:00:03  0:00:03 --:--:-- 7937k
[00:42:44] + cd llvm-7.0.0.src
[00:42:44] + mkdir -p tools/clang
[00:42:44] + curl https://releases.llvm.org/7.0.0/cfe-7.0.0.src.tar.xz
[00:42:44] + xz -d
[00:42:44] + tar xf - -C tools/clang --strip-components=1
[00:42:44]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:46] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 28 11.9M   28 3486k    0     0  4992k      0  0:00:02 --:--:--  0:00:02 5001k
---
[00:42:46]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:46] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100  894k  100  894k    0     0  1926k      0 --:--:-- --:--:-- --:--:-- 1935k
[00:42:46] + mkdir ../clang-build
[00:42:46] + cd ../clang-build
[00:42:46] + INC=/rustroot/include
[00:42:46] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:42:46] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:46] + hide_output cmake ../llvm-7.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:43:16] Mon Dec 10 16:05:11 UTC 2018 - building ...
[00:43:18] + hide_output make -j10
[00:43:18] + set +x
[00:43:48] Mon Dec 10 16:05:42 UTC 2018 - building ...
---
[01:41:18] + set +x
[01:41:19] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:41:19] + set +x
[01:41:22] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:41:22] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:41:22] + yes
[01:41:22] + yes
[01:41:22] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:41:22] + rm -rf linux-3.2.84
[01:41:23]  ---> f7766546ed27
[01:41:23] Removing intermediate container beae4c2a93c2
[01:41:23] Step 31/41 : COPY dist-x86_64-linux/build-perl.sh /tmp/
[01:41:23] Step 31/41 : COPY dist-x86_64-linux/build-perl.sh /tmp/
[01:41:23]  ---> d6259b67dfa1
[01:41:23] Step 32/41 : RUN ./build-perl.sh
[01:41:23]  ---> Running in fb6f61c82495
[01:41:24] + source shared.sh
[01:41:24] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:41:24]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:41:24]                                  Dload  Upload   Total   Spent    Left  Speed
[01:41:25] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
