plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:14ea6940
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:00:38] 
[00:00:38] Total download size: 4.9 M
[00:00:38] Downloading Packages:
[00:00:38] --------------------------------------------------------------------------------
[00:00:38] Total                                            24 MB/s | 4.9 MB     00:00     
[00:00:38] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:00:38] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:00:39] Running Transaction Test
[00:00:39] Finished Transaction Test
[00:00:39] Transaction Test Succeeded
[00:00:39] Running Transaction
---
[00:02:14] ./build-openssl.sh: line 23:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:02:14] + hide_output make install
[00:02:14] + set +x
[00:02:34] + cd ..
[00:02:34] + rm -rf openssl-1.0.2k
[00:02:34] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:02:34] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:02:35] Removing intermediate container a4a11dae1f0a
[00:02:35] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:02:35]  ---> d60a3ed5ba68
[00:02:35] Step 15/41 : RUN ./build-curl.sh
[00:02:35] Step 15/41 : RUN ./build-curl.sh
[00:02:35]  ---> Running in e5bd4889a76c
[00:02:35] + source shared.sh
[00:02:35] + VERSION=7.51.0
[00:02:35] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:02:37]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:02:37]                                  Dload  Upload   Total   Spent    Left  Speed
[00:02:38] 
  0 2509k    0  5527    0     0   3622      0  0:11:49  0:00:01  0:11:48  3622
  0 2509k    0  5527    0     0   3622      0  0:11:49  0:00:01  0:11:48  3622
  2 2509k    2 59559    0     0  33560      0  0:01:16  0:00:01  0:01:15  211k
100 2509k  100 2509k    0     0   922k      0  0:00:02  0:00:02 --:--:-- 2095k
[00:02:38] + mkdir curl-build
[00:02:38] + cd curl-build
[00:02:38] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:03:00] + hide_output make -j10
[00:03:00] + set +x
[00:03:12] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:12] + hide_output make install
---
 94 82.1M   94 77.8M    0     0  3659k      0  0:00:22  0:00:21  0:00:01 3264k
 98 82.1M   98 81.1M    0     0  3648k      0  0:00:23  0:00:22  0:00:01 3403k
100 82.1M  100 82.1M    0     0  3650k      0  0:00:23  0:00:23 --:--:-- 3464k
[00:07:01] + cd gcc-4.8.5
[00:07:01] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:07:01] --2018-11-30 15:52:37--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:07:01] Resolving gcc.gnu.org... 209.132.180.131
[00:07:01] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:07:01] HTTP request sent, awaiting response... 200 OK
---
 22 27.0M   22 6214k    0     0  5692k      0  0:00:04  0:00:01  0:00:03 5829k
 51 27.0M   51 13.8M    0     0  6795k      0  0:00:04  0:00:02  0:00:02 6880k
 79 27.0M   79 21.3M    0     0  7073k      0  0:00:03  0:00:03 --:--:-- 7132k
100 27.0M  100 27.0M    0     0  7259k      0  0:00:03  0:00:03 --:--:-- 7309k
[00:41:45] + cd llvm-7.0.0.src
[00:41:45] + mkdir -p tools/clang
[00:41:45] + curl https://releases.llvm.org/7.0.0/cfe-7.0.0.src.tar.xz
[00:41:45] + xz -d
[00:41:45] + tar xf - -C tools/clang --strip-components=1
[00:41:45]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:47] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0 11.9M    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[00:41:47] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100  894k  100  894k    0     0  1999k      0 --:--:-- --:--:-- --:--:-- 2014k
[00:41:47] + mkdir ../clang-build
[00:41:47] + cd ../clang-build
[00:41:47] + INC=/rustroot/include
[00:41:47] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:41:47] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:41:47] + hide_output cmake ../llvm-7.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:17] Fri Nov 30 16:27:53 UTC 2018 - building ...
[00:42:20] + hide_output make -j10
[00:42:20] + set +x
[00:42:50] Fri Nov 30 16:28:25 UTC 2018 - building ...
---
[01:43:01] + set +x
[01:43:02] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:43:02] + set +x
[01:43:06] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:43:06] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:43:06] + yes
[01:43:06] + yes
[01:43:06] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:43:06] + rm -rf linux-3.2.84
[01:43:07]  ---> aa5e813e3050
[01:43:07] Removing intermediate container 72298fcae503
[01:43:07] Step 31/41 : COPY dist-x86_64-linux/build-perl.sh /tmp/
[01:43:07] Step 31/41 : COPY dist-x86_64-linux/build-perl.sh /tmp/
[01:43:07]  ---> 3d3ad2bdfeba
[01:43:07] Step 32/41 : RUN ./build-perl.sh
[01:43:07]  ---> Running in eec6f110053d
[01:43:07] + source shared.sh
[01:43:07] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:43:07]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:43:07]                                  Dload  Upload   Total   Spent    Left  Speed
[01:43:09] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
