plain
127.0.0.1 localhost nettuno travis vagrant
127.0.1.1 travis-job-ecd53ccc-fe43-4921-936a-cb08e278b8b3 travis-job-ecd53ccc-fe43-4921-936a-cb08e278b8b3 ip4-loopback trusty64
travis_fold:start:git.checkout
travis_time:start:2b6e7710
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
travis_time:end:02006118:start=1524649400851928284,finish=1524649402029139047,duration=1177210763
travis_fold:end:before_install.3
travis_fold:start:install
travis_time:start:00f4c442
$ case "$TRAVIS_OS_NAME" in linux) travis_retry curl -fo $HOME/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl && chmod +x $HOME/stamp && export PATH=$PATH:$HOME ;; osx) if [[ "$RUST_CHECK_TARGET" == dist ]]; then travis_retry brew update && travis_retry brew install xz; fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -O http://releases.llvm.org/6.0.0/clang+llvm-6.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang++ ;; esac
travis_fold:end:install
travis_fold:start:before_script.1
travis_time:start:0fbfe3b3
$ echo "#### Disk usage before running script:"; df -h; du . | sort -nr | head -n100
---
[00:04:23] ./build-openssl.sh: line 23:   350 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:23] + hide_output make install
[00:04:23] + set +x
[00:04:42] + cd ..
[00:04:42] + rm -rf openssl-1.0.2k
[00:04:42] ./build-openssl.sh: line 25:  4116 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:42] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:43] Removing intermediate container e88a9c4ec0f1
[00:04:43] Step 14/37 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:43]  ---> 30c8e60f33e2
[00:04:43] Step 15/37 : RUN ./build-curl.sh
[00:04:43] Step 15/37 : RUN ./build-curl.sh
[00:04:43]  ---> Running in 2c2364d380a2
[00:04:43] + source shared.sh
[00:04:43] + VERSION=7.51.0
[00:04:43] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:45]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:45]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:46] 
  0 2509k    0 14215    0     0   8884      0  0:04:49  0:00:01  0:04:48  8884
  0 2509k    0 14215    0     0   8884      0  0:04:49  0:00:01  0:04:48  8884
 15 2509k   15  385k    0     0   170k      0  0:00:14  0:00:02  0:00:12  559k
100 2509k  100 2509k    0     0   852k      0  0:00:02  0:00:02 --:--:-- 1855k
[00:04:46] + mkdir curl-build
[00:04:46] + cd curl-build
[00:04:46] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:07] + hide_output make -j10
[00:05:07] + set +x
[00:05:19] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:19] + hide_output make install
---
[00:06:14]  ---> 66e3329e9000
[00:06:14] Step 19/37 : RUN ./build-cmake.sh
[00:06:14]  ---> Running in e8eb44167101
[00:06:14] + source shared.sh
[00:06:14] + curl https://cmake.org/files/v3.6/cmake-3.6.3.tar.gz
[00:06:14]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:06:14]                                  Dload  Upload   Total   Spent    Left  Speed
[00:06:17] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
 95 82.1M   95 78.7M    0     0  3845k      0  0:00:21  0:00:20  0:00:01 3373k
 99 82.1M   99 82.1M    0     0  3830k      0  0:00:21  0:00:21 --:--:-- 3521k
100 82.1M  100 82.1M    0     0  3831k      0  0:00:21  0:00:21 --:--:-- 3628k
[00:09:06] + cd gcc-4.8.5
[00:09:06] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:06] --2018-04-25 09:52:29--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:06] Resolving gcc.gnu.org... 209.132.180.131
[00:09:06] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:06] HTTP request sent, awaiting response... 200 OK
---
[00:09:08]   1850K ...                                                   100%  164K=1.2s
[00:09:08] 
[00:09:08] 2018-04-25 09:52:32 (1.51 MB/s) - `gmp-4.3.2.tar.bz2' saved [1897483/1897483]
[00:09:08] 
[00:09:09] --2018-04-25 09:52:33--  http://gcc.gnu.org/pub/gcc/infrastructure/mpc-0.8.1.tar.gz
[00:09:09] Resolving gcc.gnu.org... 209.132.180.131
[00:09:09] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:09] HTTP request sent, awaiting response... 200 OK
[00:09:09] Length: 544950 (532K) [application/x-gzip]
[00:09:09] Saving to: `mpc-0.8.1.tar.gz'
[00:09:09]      0K .......... .......... .......... .......... ..........  9%  302K 2s
[00:09:09]     50K .......... .......... .......... .......... .......... 18%  895K 1s
[00:09:09]    100K .......... .......... .......... .......... .......... 28%  452K 1s
[00:09:09]    150K .......... .......... .......... .......... .......... 37%  903K 1s
---
[00:42:21]  ---> f08a8fce65b9
[00:42:21] Step 23/37 : RUN ./build-python.sh
[00:42:21]  ---> Running in 64370f2294d5
[00:42:21] + source shared.sh
[00:42:21] + curl https://www.python.org/ftp/python/2.7.12/Python-2.7.12.tgz
[00:42:21]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:42:21]                                  Dload  Upload   Total   Spent    Left  Speed
[00:42:23] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  1 16.1M    1  261k    0     0   743k      0  0:00:22 --:--:--  0:00:22  786k
 58 16.1M   58 9653k    0     0  7146k      0  0:00:02  0:00:01  0:00:01 7247k
100 16.1M  100 16.1M    0     0  8762k      0  0:00:01  0:00:01 --:--:-- 8853k
[00:42:23] + mkdir python-build
[00:42:23] + cd python-build
[00:42:23] + CFLAGS='-I /rustroot/include'
[00:42:23] + LDFLAGS='-L /rustroot/lib -L /rustroot/lib64'
[00:42:23] + hide_output ../Python-2.7.12/configure --prefix=/rustroot
[00:42:36] + hide_output make -j10
[00:42:36] + set +x
[00:43:06] Wed Apr 25 10:26:30 UTC 2018 - building ...
[00:43:36] Wed Apr 25 10:27:00 UTC 2018 - building ...
---
[00:43:50]  ---> 2fed4f161e32
[00:43:50] Step 25/37 : RUN ./build-clang.sh
[00:43:50]  ---> Running in 389c77762f74
[00:43:50] + source shared.sh
[00:43:50] + LLVM=6.0.0
[00:43:50] + mkdir clang
[00:43:50] + cd clang
[00:43:50] + curl https://releases.llvm.org/6.0.0/llvm-6.0.0.src.tar.xz
[00:43:50] + tar xf -
[00:43:50] + xz -d
[00:43:50]                                  Dload  Upload   Total   Spent    Left  Speed
[00:43:53] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 18 24.1M   18 4599k    0     0  7156k      0  0:00:03 --:--:--  0:00:03 7678k
 18 24.1M   18 4599k    0     0  7156k      0  0:00:03 --:--:--  0:00:03 7678k
 57 24.1M   57 13.8M    0     0  8648k      0  0:00:02  0:00:01  0:00:01 8885k
 90 24.1M   90 21.9M    0     0  8491k      0  0:00:02  0:00:02 --:--:-- 8637k
100 24.1M  100 24.1M    0     0  8666k      0  0:00:02  0:00:02 --:--:-- 8803k
[00:43:53] + cd llvm-6.0.0.src
[00:43:53] + mkdir -p tools/clang
[00:43:53] + curl https://releases.llvm.org/6.0.0/cfe-6.0.0.src.tar.xz
[00:43:53] + xz -d
[00:43:53] + tar xf - -C tools/clang --strip-components=1
[00:43:53]                                  Dload  Upload   Total   Spent    Left  Speed
[00:43:54] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 51 11.4M   51 6050k    0     0  7779k      0  0:00:01 --:--:--  0:00:01 7796k
 51 11.4M   51 6050k    0     0  7779k      0  0:00:01 --:--:--  0:00:01 7796k
100 11.4M  100 11.4M    0     0  6929k      0  0:00:01  0:00:01 --:--:-- 6939k
[00:43:54] + mkdir ../clang-build
[00:43:54] + cd ../clang-build
[00:43:54] + INC=/rustroot/include
[00:43:54] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:43:54] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:43:54] + hide_output cmake ../llvm-6.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:44:23] + hide_output make -j10
[00:44:23] + set +x
[00:44:53] Wed Apr 25 10:28:17 UTC 2018 - building ...
[00:45:23] Wed Apr 25 10:28:47 UTC 2018 - building ...
---
[01:38:44] + hide_output make install
[01:38:44] + set +x
[01:38:57] shared.sh: line 11:  1357 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:38:57] + cd ../..
[01:38:57] + rm -rf clang
[01:38:58] ./build-clang.sh: line 64: 14563 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/clang/clang-build)
[01:39:07] Removing intermediate container 389c77762f74
[01:39:07] Removing intermediate container 389c77762f74
[01:39:07] Step 26/37 : ENV CC clang CXX clang++
[01:39:07]  ---> Running in 2ac9ad70193f
[01:39:07] Removing intermediate container 2ac9ad70193f
[01:39:07] Step 27/37 : COPY dist-x86_64-linux/build-git.sh /tmp/
[01:39:07]  ---> 03af8f72f78f
[01:39:07] Step 28/37 : RUN ./build-git.sh
[01:39:07] Step 28/37 : RUN ./build-git.sh
[01:39:07]  ---> Running in 911eea7660cd
[01:39:07] + source shared.sh
[01:39:07] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:39:07]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:39:07]                                  Dload  Upload   Total   Spent    Left  Speed
[01:39:08] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
100 62.6M  100 62.6M    0     0  9619k      0  0:00:06  0:00:06 --:--:-- 10.7M
[01:40:07] + cd linux-3.2.84
[01:40:07] + hide_output make mrproper
[01:40:07] + set +x
[01:40:08] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:40:11] shared.sh: line 11:     9 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:40:11] shared.sh: line 11:     9 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:40:11] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:40:11] + yes
[01:40:11] + yes
[01:40:11] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:40:11] + rm -rf linux-3.2.84
[01:40:12]  ---> 0170d84657aa
[01:40:12] Removing intermediate container 4bdcce122ec5
[01:40:12] Step 31/37 : COPY scripts/sccache.sh /scripts/
---
[01:40:15] Removing intermediate container 4f4223e82560
[01:40:15] Step 33/37 : ENV HOSTS x86_64-unknown-linux-gnu
[01:40:15]  ---> Running in f1689b2772bb
[01:40:16]  ---> b25380806502
[01:40:16] Removing intermediate container f1689b2772bb
[01:40:16] Step 34/37 : ENV RUST_CONFIGURE_ARGS --enable-full-tools       --enable-sanitizers       --enable-profiler       --enable-compiler-docs       --set target.x86_64-unknown-linux-gnu.linker=clang
[01:40:16]  ---> Running in 65484c6581ee
[01:40:16] Removing intermediate container 65484c6581ee
[01:40:16] Step 35/37 : ENV SCRIPT python2.7 ../x.py dist --host $HOSTS --target $HOSTS -v
[01:40:16]  ---> Running in 62046360d85d
[01:40:16]  ---> 414e3c992765
---
[01:42:27] travis_fold:start:configure
travis_time:start:0100a208
configure: processing command line
[01:42:27] configure: 
[01:42:27] configure: target.x86_64-unknown-linux-gnu.linker := clang
[01:42:27] configure: build.submodules     := False
[01:42:27] configure: build.compiler-docs  := True
[01:42:27] configure: llvm.ccache          := sccache
[01:42:27] configure: build.profiler       := True
---
[01:43:17]    Compiling serde_json v1.0.15
[01:43:17]    Compiling toml v0.4.6
[01:43:22]    Compiling serde_derive_internals v0.23.1
[01:43:24]    Compiling serde_derive v1.0.40
[01:43:32] error: linker `cc` not found
[01:43:32]   = note: No such file or directory (os error 2)
[01:43:32] 
[01:43:32] error: aborting due to previous error
[01:43:32] 
[01:43:32] 
[01:43:32] error: Could not compile `serde_derive`.
[01:43:32] To learn more, run the command again with --verbose.
[01:43:32] To learn more, run the command again with --verbose.
[01:43:32] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[01:43:32] Build completed unsuccessfully in 0:01:04
[01:43:32] make: *** [prepare] Error 1
[01:43:32]    Compiling serde_derive v1.0.40
[01:43:32]    Compiling serde_derive v1.0.40
[01:43:39] error: linker `cc` not found
[01:43:39]   = note: No such file or directory (os error 2)
[01:43:39] 
[01:43:39] error: aborting due to previous error
[01:43:39] 
[01:43:39] 
[01:43:39] error: Could not compile `serde_derive`.
[01:43:39] To learn more, run the command again with --verbose.
[01:43:39] To learn more, run the command again with --verbose.
[01:43:39] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[01:43:39] Build completed unsuccessfully in 0:00:07
[01:43:39] make: *** [prepare] Error 1
[01:43:40]    Compiling serde_derive v1.0.40
[01:43:40]    Compiling serde_derive v1.0.40
[01:43:47] error: linker `cc` not found
[01:43:47]   = note: No such file or directory (os error 2)
[01:43:47] 
[01:43:47] error: aborting due to previous error
[01:43:47] 
[01:43:47] 
[01:43:47] error: Could not compile `serde_derive`.
[01:43:47] To learn more, run the command again with --verbose.
[01:43:47] To learn more, run the command again with --verbose.
[01:43:47] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[01:43:47] Build completed unsuccessfully in 0:00:07
[01:43:47] make: *** [prepare] Error 1
[01:43:48]    Compiling serde_derive v1.0.40
[01:43:48]    Compiling serde_derive v1.0.40
[01:43:55] error: linker `cc` not found
[01:43:55]   = note: No such file or directory (os error 2)
[01:43:55] 
[01:43:55] error: aborting due to previous error
[01:43:55] 
[01:43:55] 
[01:43:55] error: Could not compile `serde_derive`.
[01:43:55] To learn more, run the command again with --verbose.
[01:43:55] To learn more, run the command again with --verbose.
[01:43:55] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[01:43:55] Build completed unsuccessfully in 0:00:07
[01:43:55] make: *** [prepare] Error 1
[01:43:55]    Compiling serde_derive v1.0.40
[01:43:55]    Compiling serde_derive v1.0.40
[01:44:03] error: linker `cc` not found
[01:44:03]   = note: No such file or directory (os error 2)
[01:44:03] 
[01:44:03] error: aborting due to previous error
[01:44:03] 
[01:44:03] 
[01:44:03] error: Could not compile `serde_derive`.
[01:44:03] To learn more, run the command again with --verbose.
[01:44:03] To learn more, run the command again with --verbose.
[01:44:03] failed to run: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[01:44:03] Build completed unsuccessfully in 0:00:07
[01:44:03] make: *** [prepare] Error 1
[01:44:03] The command has failed after 5 attempts.
