plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:03a6361e
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:01:23] 
[00:01:23] Total download size: 4.9 M
[00:01:23] Downloading Packages:
[00:01:24] --------------------------------------------------------------------------------
[00:01:24] Total                                           9.7 MB/s | 4.9 MB     00:00     
[00:01:24] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:01:24] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:01:24] Running Transaction Test
[00:01:24] Finished Transaction Test
[00:01:24] Transaction Test Succeeded
[00:01:24] Running Transaction
---
[00:03:01] + hide_output make install
[00:03:01] + set +x
[00:03:20] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:20] + cd ..
[00:03:20] + rm -rf openssl-1.0.2k
[00:03:20] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:03:20] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:03:21] Removing intermediate container 5585f237ffa2
[00:03:21] Step 14/39 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:03:21]  ---> f5f63ec5feb8
[00:03:21] Step 15/39 : RUN ./build-curl.sh
[00:03:21] Step 15/39 : RUN ./build-curl.sh
[00:03:21]  ---> Running in 45dd6d95f9a1
[00:03:21] + source shared.sh
[00:03:21] + VERSION=7.51.0
[00:03:21] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:03:23]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:03:23]                                  Dload  Upload   Total   Spent    Left  Speed
[00:03:24] 
  0 2509k    0 14215    0     0   9063      0  0:04:43  0:00:01  0:04:42  9063
  0 2509k    0 14215    0     0   9063      0  0:04:43  0:00:01  0:04:42  9063
  1 2509k    1 30599    0     0  18003      0  0:02:22  0:00:01  0:02:21  121k
 69 2509k   69 1732k    0     0   655k      0  0:00:03  0:00:02  0:00:01 1595k
100 2509k  100 2509k    0     0   884k      0  0:00:02  0:00:02 --:--:-- 1965k
[00:03:24] + mkdir curl-build
[00:03:24] + cd curl-build
[00:03:24] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:03:45] + hide_output make -j10
[00:03:45] + set +x
[00:03:57] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:57] + hide_output make install
---
 95 82.1M   95 78.4M    0     0  4178k      0  0:00:20  0:00:19  0:00:01 3474k
 98 82.1M   98 81.1M    0     0  4107k      0  0:00:20  0:00:20 --:--:-- 3302k
100 82.1M  100 82.1M    0     0  4111k      0  0:00:20  0:00:20 --:--:-- 3612k
[00:08:06] + cd gcc-4.8.5
[00:08:06] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:06] --2018-09-28 17:43:38--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:06] Resolving gcc.gnu.org... 209.132.180.131
[00:08:06] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:06] HTTP request sent, awaiting response... 200 OK
---
 16 24.1M   16 3989k    0     0  4889k      0  0:00:05 --:--:--  0:00:05 6938k
 53 24.1M   53 12.9M    0     0  7301k      0  0:00:03  0:00:01  0:00:02 8419k
 85 24.1M   85 20.5M    0     0  7487k      0  0:00:03  0:00:02  0:00:01 8188k
100 24.1M  100 24.1M    0     0  7757k      0  0:00:03  0:00:03 --:--:-- 8391k
[00:41:57] + cd llvm-6.0.0.src
[00:41:57] + mkdir -p tools/clang
[00:41:57] + curl https://releases.llvm.org/6.0.0/cfe-6.0.0.src.tar.xz
[00:41:57] + xz -d
[00:41:57] + tar xf - -C tools/clang --strip-components=1
[00:41:57]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:59] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 36 11.4M   36 4272k    0     0  6885k      0  0:00:01 --:--:--  0:00:01 6901k
 36 11.4M   36 4272k    0     0  6885k      0  0:00:01 --:--:--  0:00:01 6901k
 98 11.4M   98 11.2M    0     0  7098k      0  0:00:01  0:00:01 --:--:-- 7105k
100 11.4M  100 11.4M    0     0  7113k      0  0:00:01  0:00:01 --:--:-- 7116k
[00:41:59] + mkdir -p tools/lld
[00:41:59] + curl https://releases.llvm.org/6.0.0/lld-6.0.0.src.tar.xz
[00:41:59] + xz -d
[00:41:59] + tar xf - -C tools/lld --strip-components=1
[00:41:59]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:59] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100  772k  100  772k    0     0  3279k      0 --:--:-- --:--:-- --:--:-- 3313k
100  772k  100  772k    0     0  3279k      0 --:--:-- --:--:-- --:--:-- 3313k
[00:41:59] + mkdir ../clang-build
[00:41:59] + cd ../clang-build
[00:41:59] + INC=/rustroot/include
[00:41:59] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:41:59] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:41:59] + hide_output cmake ../llvm-6.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:29] Fri Sep 28 18:18:01 UTC 2018 - building ...
[00:42:31] + hide_output make -j10
[00:42:31] + set +x
[00:43:01] Fri Sep 28 18:18:32 UTC 2018 - building ...
---
[01:36:57] + set +x
[01:36:58] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:36:58] + set +x
[01:37:02] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:37:02] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:37:02] + yes
[01:37:02] + yes
[01:37:02] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:37:02] + rm -rf linux-3.2.84
[01:37:03]  ---> 532a45795ba6
[01:37:03] Removing intermediate container e4a56076135e
[01:37:03] Step 31/39 : COPY scripts/sccache.sh /scripts/
---

[01:39:29] travis_time:end:07b92c8d:start=1538156189186125707,finish=1538162101070997194,duration=5911884871487
[CI_JOB_NAME=dist-x86_64-linux]
[01:39:29] [CI_JOB_NAME=dist-x86_64-linux]
[01:39:29] /checkout/src/ci/run.sh: line 79: [: too many arguments
[01:39:29] travis_fold:start:configure
travis_time:start:09c51824
configure: processing command line
[01:39:29] configure: 
---
travis_time:end:01ee421e:start=1538163996120131431,finish=1538163996124452145,duration=4320714
travis_fold:end:after_failure.3
travis_fold:start:after_failure.4
travis_time:start:0f2eef78
$ ln -s . checkout && for CORE in obj/cores/core.*; do EXE=$(echo $CORE | sed 's|obj/cores/core\.[0-9]*\.!checkout!\(.*\)|\1|;y|!|/|'); if [ -f "$EXE" ]; then printf travis_fold":start:crashlog\n\033[31;1m%s\033[0m\n" "$CORE"; gdb --batch -q -c "$CORE" "$EXE" -iex 'set auto-load off' -iex 'dir src/' -iex 'set sysroot .' -ex bt -ex q; echo travis_fold":"end:crashlog; fi; done || true
travis_fold:end:after_failure.4
travis_fold:start:after_failure.5
travis_time:start:10c4a538
travis_time:start:10c4a538
$ cat ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers || true
cat: ./obj/build/x86_64-unknown-linux-gnu/native/asan/build/lib/asan/clang_rt.asan-dynamic-i386.vers: No such file or directory
travis_fold:end:after_failure.5
travis_fold:start:after_failure.6
travis_time:start:228f8870
$ dmesg | grep -i kill
