plain
$ export AWS_SECRET_ACCESS_KEY=[secure]
$ export GCP_CACHE_BUCKET=rust-lang-ci-cache
$ export GCP_CLIENT_SECRET=[secure]
Setting environment variables from .travis.yml
$ export CI_JOB_NAME=$TRAVIS_JOB_NAME
$ export DEPLOY=1
$ bash -c 'echo $BASH_VERSION'
4.3.48(1)-release
travis_fold:start:before_install.1
---
[00:01:19] 
[00:01:19] Total download size: 4.9 M
[00:01:19] Downloading Packages:
[00:01:23] --------------------------------------------------------------------------------
[00:01:23] Total                                           1.5 MB/s | 4.9 MB     00:03     
[00:01:23] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:23] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:23] Running Transaction Test
[00:01:23] Finished Transaction Test
[00:01:23] Transaction Test Succeeded
[00:01:23] Running Transaction
---
[00:04:07] + hide_output make install
[00:04:07] + set +x
[00:04:29] shared.sh: line 1:   352 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:29] + cd ..
[00:04:29] + rm -rf openssl-1.0.2k
[00:04:29] ./build-openssl.sh: line 16:  4115 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:29] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:30]  ---> ae4a69f069b0
[00:04:30] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:30]  ---> 70b92e5f79a6
[00:04:30] Step 15/41 : RUN ./build-curl.sh
[00:04:30] Step 15/41 : RUN ./build-curl.sh
[00:04:30]  ---> Running in a5703aa460e0
[00:04:30] + source shared.sh
[00:04:30] + VERSION=7.51.0
[00:04:30] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:32]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:32]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:33] 
  0 2509k    0  5527    0     0   3663      0  0:11:41  0:00:01  0:11:40  3663
  0 2509k    0  5527    0     0   3663      0  0:11:41  0:00:01  0:11:40  3663
 26 2509k   26  657k    0     0   291k      0  0:00:08  0:00:02  0:00:06  873k
100 2509k  100 2509k    0     0   903k      0  0:00:02  0:00:02 --:--:-- 1976k
[00:04:33] + mkdir curl-build
[00:04:33] + cd curl-build
[00:04:33] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:58] + hide_output make -j10
[00:04:58] + set +x
[00:05:11] shared.sh: line 1:    13 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:11] + hide_output make install
---
 87 67.8M   87 59.0M    0     0  6985k      0  0:00:09  0:00:08  0:00:01 7363k
 98 67.8M   98 66.6M    0     0  7066k      0  0:00:09  0:00:09 --:--:-- 7465k
100 67.8M  100 67.8M    0     0  7093k      0  0:00:09  0:00:09 --:--:-- 8224k
[00:09:03] + cd gcc-5.5.0
[00:09:03] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:03] --2019-03-20 00:47:58--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:03] Resolving gcc.gnu.org... 209.132.180.131
[00:09:04] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:04] HTTP request sent, awaiting response... 200 OK
---
[01:15:45]  ---> 44744e4a4dbb
[01:15:45] Step 25/41 : RUN ./build-clang.sh
[01:15:45]  ---> Running in 097526f8d27b
[01:15:45] + source shared.sh
[01:15:45] + LLVM=llvmorg-8.0.0-rc2
[01:15:45] + cd llvm-project
[01:15:45] + cd llvm-project
[01:15:45] + curl -L https://github.com/llvm/llvm-project/archive/llvmorg-8.0.0-rc2.tar.gz
[01:15:45] + tar xzf - --strip-components=1
[01:15:45]                                  Dload  Upload   Total   Spent    Left  Speed
[01:15:46] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   136    0   136    0     0    678      0 --:--:-- --:--:-- --:--:--   786
---
[01:16:04] + cd clang-build
[01:16:04] + INC=/rustroot/include
[01:16:04] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed
[01:16:04] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:16:04] + hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;lld' -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/5.5.0/include-fixed:/usr/include
[01:16:34] Wed Mar 20 01:55:29 UTC 2019 - building ...
[01:16:43] + hide_output make -j10
[01:16:43] + set +x
[01:17:13] Wed Mar 20 01:56:08 UTC 2019 - building ...
---
[02:42:48]  ---> 82e84f84d82d
[02:42:48] Step 32/41 : RUN ./build-perl.sh
[02:42:48]  ---> Running in 0277a2f6c0dc
[02:42:49] + source shared.sh
[02:42:49] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[02:42:49]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[02:42:49]                                  Dload  Upload   Total   Spent    Left  Speed
[02:42:50] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  4 17.0M    4  814k    0     0  1981k      0  0:00:08 --:--:--  0:00:08 2360k
100 17.0M  100 17.0M    0     0  15.3M      0  0:00:01  0:00:01 --:--:-- 16.3M
[02:42:50] + cd perl-5.28.0
[02:42:50] + CC=gcc
[02:42:50] + CFLAGS='-I /rustroot/include -fgnu89-inline'
[02:42:50] + hide_output ./configure.gnu
[02:42:50] + set +x
[02:43:20] Wed Mar 20 03:22:15 UTC 2019 - building ...
[02:43:30] + hide_output make -j10
