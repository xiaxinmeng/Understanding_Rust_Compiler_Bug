plain
127.0.0.1 localhost nettuno travis vagrant
127.0.1.1 travis-job-ed01c438-70fa-414e-ab22-16cbb3a31899 travis-job-ed01c438-70fa-414e-ab22-16cbb3a31899 ip4-loopback trusty64
travis_fold:start:git.checkout
travis_time:start:1ba88ac0
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
travis_time:end:0c318001:start=1524667470431467929,finish=1524667471620842250,duration=1189374321
travis_fold:end:before_install.3
travis_fold:start:install
travis_time:start:0423759a
$ case "$TRAVIS_OS_NAME" in linux) travis_retry curl -fo $HOME/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-unknown-linux-musl && chmod +x $HOME/stamp && export PATH=$PATH:$HOME ;; osx) if [[ "$RUST_CHECK_TARGET" == dist ]]; then travis_retry brew update && travis_retry brew install xz; fi && travis_retry curl -fo /usr/local/bin/sccache https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2018-04-02-sccache-x86_64-apple-darwin && chmod +x /usr/local/bin/sccache && travis_retry curl -fo /usr/local/bin/stamp https://s3-us-west-1.amazonaws.com/rust-lang-ci2/rust-ci-mirror/2017-03-17-stamp-x86_64-apple-darwin && chmod +x /usr/local/bin/stamp && travis_retry curl -O http://releases.llvm.org/6.0.0/clang+llvm-6.0.0-x86_64-apple-darwin.tar.xz | tar xJf - && export CC=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang && export CXX=`pwd`/clang+llvm-6.0.0-x86_64-apple-darwin/bin/clang++ ;; esac
travis_fold:end:install
travis_fold:start:before_script.1
travis_time:start:09fe4d42
$ echo "#### Disk usage before running script:"; df -h; du . | sort -nr | head -n100
---
[00:04:17] + hide_output make install
[00:04:17] + set +x
[00:04:36] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:36] + cd ..
[00:04:36] + rm -rf openssl-1.0.2k
[00:04:36] ./build-openssl.sh: line 25:  4117 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:04:36] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:04:36] Removing intermediate container 6f29835d5dcf
[00:04:36] Step 14/38 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:04:36]  ---> ffd3825afc1a
[00:04:36] Step 15/38 : RUN ./build-curl.sh
[00:04:36] Step 15/38 : RUN ./build-curl.sh
[00:04:36]  ---> Running in 6ac334d3ed02
[00:04:37] + source shared.sh
[00:04:37] + VERSION=7.51.0
[00:04:37] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:38]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:38]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:39] 
  0 2509k    0 14215    0     0   9114      0  0:04:41  0:00:01  0:04:40  9114
  0 2509k    0 14215    0     0   9114      0  0:04:41  0:00:01  0:04:40  9114
  1 2509k    1 30599    0     0  18006      0  0:02:22  0:00:01  0:02:21  114k
 76 2509k   76 1909k    0     0   732k      0  0:00:03  0:00:02  0:00:01 1810k
100 2509k  100 2509k    0     0   911k      0  0:00:02  0:00:02 --:--:-- 2092k
[00:04:39] + mkdir curl-build
[00:04:39] + cd curl-build
[00:04:39] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:58] + hide_output make -j10
[00:04:58] + set +x
[00:05:10] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:10] + hide_output make install
---
[00:06:06]  ---> b99d4a7ffa16
[00:06:06] Step 19/38 : RUN ./build-cmake.sh
[00:06:06]  ---> Running in 712f84312e12
[00:06:06] + source shared.sh
[00:06:06] + curl https://cmake.org/files/v3.6/cmake-3.6.3.tar.gz
[00:06:06]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:06:06]                                  Dload  Upload   Total   Spent    Left  Speed
[00:06:18] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
 97 82.1M   97 79.8M    0     0  1563k      0  0:00:53  0:00:52  0:00:01 1224k
 98 82.1M   98 80.9M    0     0  1554k      0  0:00:54  0:00:53  0:00:01 1196k
100 82.1M  100 82.1M    0     0  1550k      0  0:00:54  0:00:54 --:--:-- 1201k
[00:09:43] + cd gcc-4.8.5
[00:09:43] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:43] --2018-04-25 14:54:17--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:44] Resolving gcc.gnu.org... 209.132.180.131
[00:09:44] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:44] HTTP request sent, awaiting response... 200 OK
---
[00:09:46]   1850K ...                                                   100% 5743G=1.3s
[00:09:46] 
[00:09:46] 2018-04-25 14:54:20 (1.42 MB/s) - `gmp-4.3.2.tar.bz2' saved [1897483/1897483]
[00:09:46] 
[00:09:47] --2018-04-25 14:54:20--  http://gcc.gnu.org/pub/gcc/infrastructure/mpc-0.8.1.tar.gz
[00:09:47] Resolving gcc.gnu.org... 209.132.180.131
[00:09:47] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:47] HTTP request sent, awaiting response... 200 OK
[00:09:47] Length: 544950 (532K) [application/x-gzip]
[00:09:47] Saving to: `mpc-0.8.1.tar.gz'
[00:09:47]      0K .......... .......... .......... .......... ..........  9%  281K 2s
[00:09:47]     50K .......... .......... .......... .......... .......... 18%  418K 1s
[00:09:47]    100K .......... .......... .......... .......... .......... 28%  425K 1s
[00:09:47]    150K .......... .......... .......... .......... .......... 37%  801K 1s
---
[00:44:37]  ---> 2143b801751c
[00:44:37] Step 23/38 : RUN ./build-python.sh
[00:44:37]  ---> Running in 806f079be989
[00:44:37] + source shared.sh
[00:44:37] + curl https://www.python.org/ftp/python/2.7.12/Python-2.7.12.tgz
[00:44:37]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:44:37]                                  Dload  Upload   Total   Spent    Left  Speed
[00:44:39] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 89 16.1M   89 14.4M    0     0  13.2M      0  0:00:01  0:00:01 --:--:-- 13.4M
100 16.1M  100 16.1M    0     0  13.9M      0  0:00:01  0:00:01 --:--:-- 14.1M
[00:44:39] + mkdir python-build
[00:44:39] + cd python-build
[00:44:39] + CFLAGS='-I /rustroot/include'
[00:44:39] + LDFLAGS='-L /rustroot/lib -L /rustroot/lib64'
[00:44:39] + hide_output ../Python-2.7.12/configure --prefix=/rustroot
[00:44:52] + hide_output make -j10
[00:44:52] + set +x
[00:45:22] Wed Apr 25 15:29:55 UTC 2018 - building ...
[00:45:52] Wed Apr 25 15:30:25 UTC 2018 - building ...
---
[00:46:07]  ---> cf6886c87249
[00:46:07] Step 25/38 : RUN ./build-clang.sh
[00:46:07]  ---> Running in ec1cd0a9039e
[00:46:07] + source shared.sh
[00:46:07] + LLVM=6.0.0
[00:46:07] + mkdir clang
[00:46:07] + cd clang
[00:46:07] + curl https://releases.llvm.org/6.0.0/llvm-6.0.0.src.tar.xz
[00:46:07] + tar xf -
[00:46:07] + xz -d
[00:46:07]                                  Dload  Upload   Total   Spent    Left  Speed
[00:46:10] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  7 24.1M    7 1840k    0     0  3271k      0  0:00:07 --:--:--  0:00:07 4498k
  7 24.1M    7 1840k    0     0  3271k      0  0:00:07 --:--:--  0:00:07 4498k
 48 24.1M   48 11.5M    0     0  7600k      0  0:00:03  0:00:01  0:00:02 8425k
 81 24.1M   81 19.6M    0     0  7838k      0  0:00:03  0:00:02  0:00:01 8338k
100 24.1M  100 24.1M    0     0  8038k      0  0:00:03  0:00:03 --:--:-- 8459k
[00:46:10] + cd llvm-6.0.0.src
[00:46:10] + mkdir -p tools/clang
[00:46:10] + curl https://releases.llvm.org/6.0.0/cfe-6.0.0.src.tar.xz
[00:46:10] + xz -d
[00:46:10] + tar xf - -C tools/clang --strip-components=1
[00:46:10]                                  Dload  Upload   Total   Spent    Left  Speed
[00:46:12] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  8 11.4M    8  959k    0     0  2022k      0  0:00:05 --:--:--  0:00:05 2028k
  8 11.4M    8  959k    0     0  2022k      0  0:00:05 --:--:--  0:00:05 2028k
 65 11.4M   65 7663k    0     0  5199k      0  0:00:02  0:00:01  0:00:01 5206k
100 11.4M  100 11.4M    0     0  5821k      0  0:00:02  0:00:02 --:--:-- 5827k
[00:46:12] + mkdir ../clang-build
[00:46:12] + cd ../clang-build
[00:46:12] + INC=/rustroot/include
[00:46:12] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:46:12] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:46:12] + hide_output cmake ../llvm-6.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:46:42] Wed Apr 25 15:31:15 UTC 2018 - building ...
[00:46:44] + hide_output make -j10
[00:46:44] + set +x
[00:47:14] Wed Apr 25 15:31:47 UTC 2018 - building ...
---
[01:44:16] + hide_output make install
[01:44:16] + set +x
[01:44:28] shared.sh: line 11:  1360 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:44:28] + cd ../..
[01:44:28] + rm -rf clang
[01:44:30] ./build-clang.sh: line 64: 14584 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/clang/clang-build)
[01:44:47] Removing intermediate container ec1cd0a9039e
[01:44:47] Removing intermediate container ec1cd0a9039e
[01:44:47] Step 26/38 : ENV CC clang CXX clang++
[01:44:50]  ---> Running in f0da53b234b1
[01:44:50]  ---> b4fbb829dc0b
[01:44:50] Removing intermediate container f0da53b234b1
[01:44:50]  ---> 90efdadf739c
[01:44:50] Step 28/38 : RUN ./build-git.sh
[01:44:50]  ---> Running in 26bf20ed1b53
[01:44:51] + source shared.sh
[01:44:51] + source shared.sh
[01:44:51] + tar xzf -
[01:44:51] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:44:51]                                  Dload  Upload   Total   Spent    Left  Speed
[01:44:51] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   178  100   178    0     0    574      0 --:--:-- --:--:-- --:--:--   652
---
100 62.6M  100 62.6M    0     0  9642k      0  0:00:06  0:00:06 --:--:-- 10.2M
[01:45:51] + cd linux-3.2.84
[01:45:51] + hide_output make mrproper
[01:45:51] + set +x
[01:45:53] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:45:56] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:45:56] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:45:56] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:45:56] + yes
[01:45:56] + yes
[01:45:56] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:45:56] + rm -rf linux-3.2.84
[01:45:57]  ---> 5ba96e221768
[01:45:57] Removing intermediate container 2bbdf70cac12
[01:45:57] Step 31/38 : COPY scripts/sccache.sh /scripts/
---
[01:46:00] + chmod +x /usr/local/bin/sccache
[01:46:00]  ---> d3e749f79e8a
[01:46:00] Removing intermediate container 11cd6a55c226
[01:46:00] Step 33/38 : ENV HOSTS x86_64-unknown-linux-gnu
[01:46:00]  ---> Running in 9d0e4e6a2cdb
[01:46:00]  ---> 4230fefaf078
[01:46:00] Removing intermediate container 9d0e4e6a2cdb
[01:46:00] Step 34/38 : ENV RUST_CONFIGURE_ARGS --enable-full-tools       --enable-sanitizers       --enable-profiler       --enable-compiler-docs       --set target.x86_64-unknown-linux-gnu.linker=clang
[01:46:01]  ---> Running in 395eeb97ad46
[01:46:01] Removing intermediate container 395eeb97ad46
[01:46:01] Step 35/38 : ENV SCRIPT python2.7 ../x.py dist --host $HOSTS --target $HOSTS -v
[01:46:01]  ---> Running in b374ad48465b
[01:46:01]  ---> ebaf96cfe2b4
[01:46:01]  ---> ebaf96cfe2b4
[01:46:01] Removing intermediate container b374ad48465b
[01:46:01] Step 36/38 : ENV CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER clang
[01:46:01]  ---> Running in dac61b147491
[01:46:01] Removing intermediate container dac61b147491
[01:46:01] Step 37/38 : ENV DIST_SRC 1
[01:46:01]  ---> Running in c8b47d5e1514
[01:46:01]  ---> ef4622081c51
---
[01:48:14] travis_fold:start:configure
travis_time:start:17d81c1e
configure: processing command line
[01:48:14] configure: 
[01:48:14] configure: target.x86_64-unknown-linux-gnu.linker := clang
[01:48:14] configure: build.submodules     := False
[01:48:14] configure: build.compiler-docs  := True
[01:48:14] configure: llvm.ccache          := sccache
[01:48:14] configure: build.profiler       := True
---
[01:50:18] DirectMap2M:     3078144 kB
[01:50:18] DirectMap1G:    14680064 kB
[01:50:18] + python2.7 ../x.py dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu -v
[01:50:18] travis_fold:end:log-system-info
running: /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo build --manifest-path /checkout/src/bootstrap/Cargo.toml --locked
[01:50:19] running: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu -v
[01:50:21] finding compilers
[01:50:21] finding compilers
[01:50:21] CC_x86_64-unknown-linux-gnu = "clang"
[01:50:21] CFLAGS_x86_64-unknown-linux-gnu = ["-ffunction-sections", "-fdata-sections", "-fPIC", "--target=x86_64-unknown-linux-gnu"]
[01:50:21] AR_x86_64-unknown-linux-gnu = "ar"
[01:50:21] CXX_x86_64-unknown-linux-gnu = "clang++"
[01:50:21] running sanity check
[01:50:21] learning about cargo
[01:50:21] > Docs { stage: 2, host: "x86_64-unknown-linux-gnu" }
[01:50:21]   > UnstableBook { target: "x86_64-unknown-linux-gnu" }
[01:50:21]     > UnstableBookGen { target: "x86_64-unknown-linux-gnu" }
[01:50:21]       > Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:21]         > Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:21]           > Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:21]           < Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             > Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]               > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]                 > StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]                 < StartupObjects { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22] Dirty - /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release
[01:50:22]                 > Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]                 < Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]                 c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]                 > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]                   > Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]                     c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]                   < Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]                   > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd }
[01:50:22]                   < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd }
[01:50:22]                 < StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]               < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]               c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]               c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]               > TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]                 c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]                 > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest }
[01:50:22]                 < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest }
[01:50:22]               < TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]             < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]               c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]               > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc }
[01:50:22]               < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc }
[01:50:22]             < RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]           < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           > CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
[01:50:22]             c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             > Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
[01:50:22]             < Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
[01:50:22]           < CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
[01:50:22]           > CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "emscripten" }
[01:50:22]             c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             > Llvm { target: "x86_64-unknown-linux-gnu", emscripten: true }
[01:50:22]             < Llvm { target: "x86_64-unknown-linux-gnu", emscripten: true }
[01:50:22]           < CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "emscripten" }
[01:50:22]           > Lld { target: "x86_64-unknown-linux-gnu" }
[01:50:22]           < Lld { target: "x86_64-unknown-linux-gnu" }
[01:50:22]           > Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           < Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]           > Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]             c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           < Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]           c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         < Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           > Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]               > StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]               < StartupObjects { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]               c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]               c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]               c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]               c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]               c Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
[01:50:22]               c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]               > StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]                 c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]                 > CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd }
[01:50:22]                 < CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd }
[01:50:22]               < StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]             < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             > TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]               c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]               > CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest }
[01:50:22]               < CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest }
[01:50:22]             < TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]           < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           > RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]             c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]             > CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc }
[01:50:22]             < CleanTools { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc }
[01:50:22]           < RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         > CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
[01:50:22]           c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
[01:50:22]         < CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
[01:50:22]         > CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "emscripten" }
[01:50:22]           c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Llvm { target: "x86_64-unknown-linux-gnu", emscripten: true }
[01:50:22]         < CodegenBackend { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "emscripten" }
[01:50:22]         c Lld { target: "x86_64-unknown-linux-gnu" }
[01:50:22]         > Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         < Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Libdir { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         > Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         < Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       < Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       > Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         > StartupObjects { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         < StartupObjects { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         > StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]           c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]           > CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd }
[01:50:22]             c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           < CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd }
[01:50:22]         < StdLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       > UnstableBookGen { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         > ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: Libstd, is_ext_tool: false, extra_features: [] }
[01:50:22]           c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         < ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "unstable-book-gen", path: "src/tools/unstable-book-gen", mode: Libstd, is_ext_tool: false, extra_features: [] }
[01:50:22]       < UnstableBookGen { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     < UnstableBookGen { target: "x86_64-unknown-linux-gnu" }
[01:50:22]     > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "unstable-book", src: "/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       > Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         > ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: Librustc, is_ext_tool: false, extra_features: [] }
[01:50:22]           c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         < ToolBuild { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", tool: "rustbook", path: "src/tools/rustbook", mode: Librustc, is_ext_tool: false, extra_features: [] }
[01:50:22]       < Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "unstable-book", src: "/checkout/obj/build/x86_64-unknown-linux-gnu/md-doc" }
[01:50:22]   < UnstableBook { target: "x86_64-unknown-linux-gnu" }
[01:50:22]   c UnstableBookGen { target: "x86_64-unknown-linux-gnu" }
[01:50:22]   c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]   > TheBook { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", name: "book" }
[01:50:22]     > Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/first-edition" }
[01:50:22]       > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/first-edition", src: "/checkout/src/doc" }
[01:50:22]         c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/first-edition", src: "/checkout/src/doc" }
[01:50:22]     < Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/first-edition" }
[01:50:22]     > Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/second-edition" }
[01:50:22]       > RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/second-edition", src: "/checkout/src/doc" }
[01:50:22]         c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Rustbook { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       < RustbookSrc { target: "x86_64-unknown-linux-gnu", name: "book/second-edition", src: "/checkout/src/doc" }
[01:50:22]     < Rustbook { target: "x86_64-unknown-linux-gnu", name: "book/second-edition" }
[01:50:22]     > Standalone { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       > Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]         c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         > Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           > Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             > TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]               c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]               > CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest }
[01:50:22]                 c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]               < CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest }
[01:50:22]             < TestLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]           < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]           > RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]             c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]             > CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc }
[01:50:22]               c Assemble { target_compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]             < CleanTools { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc }
[01:50:22]           < RustcLink { compiler: Compiler { stage: 1, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]         < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]         c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       < Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]       c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]       c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     < Standalone { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Assemble { target_compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Sysroot { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" } }
[01:50:22]     c Libdir { compiler: Compiler { stage: 2, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:50:22]     c Rustdoc { host: "x86_64-unknown-linux-gnu" }
---
[01:50:39] warning: ../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingMerge.c:86:48: warning: unused parameter 'ProfileSize' [-Wunused-parameter]
[01:50:39] warning:                                       uint64_t ProfileSize) {
[01:50:39] warning:                                                ^
[01:50:39] warning: 1 warning generated.
[01:50:39] warning: ../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.c:119:3: warning: implicit declaration of function 'flock' is invalid in C99 [-Wimplicit-function-declaration]
[01:50:39] warning:   flock(fd, LOCK_EX);
[01:50:39] warning:   ^
[01:50:39] warning: ../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingUtil.c:144:3: warning: implicit declaration of function 'flock' is invalid in C99 [-Wimplicit-function-declaration]
[01:50:39] warning:   flock(fd, LOCK_UN);
[01:50:39] warning:   ^
[01:50:39] warning: ../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingValue.c:108:60: warning: unused parameter 'Data' [-Wunused-parameter]
[01:50:39] warning: static ValueProfNode *allocateOneNode(__llvm_profile_data *Data, uint32_t Index,
[01:50:39] warning:                                                            ^
[01:50:39] warning: ../libcompiler_builtins/compiler-rt/lib/profile/InstrProfilingValue.c:108:75: warning: unused parameter 'Index' [-Wunused-parameter]
---
[01:50:59]    Compiling alloc_system v0.0.0 (file:///checkout/src/liballoc_system)
[01:50:59]    Compiling panic_abort v0.0.0 (file:///checkout/src/libpanic_abort)
[01:51:04]    Compiling panic_unwind v0.0.0 (file:///checkout/src/libpanic_unwind)
[01:51:23]     Finished release [optimized] target(s) in 59.43 secs
[01:51:23] updating "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/.libstd.stamp" as "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-std/x86_64-unknown-linux-gnu/release/deps/libstd-5dfa2e03c63ddf28.rlib" changed
[01:51:23]                 c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:51:23]                 > StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:51:23] Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[01:51:23]                   > Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:51:23]                     c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:51:23]                   < Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:51:23]                   > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd }
[01:51:23] Dirty - /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools
[01:51:23]                   < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libstd }
[01:51:23]                 < StdLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }

[01:51:23] travis_time:end:stage0-std:start=1524674097134867871,finish=1524674157026996492,duration=59892128621


[01:51:23]               < Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:51:23] Dirty - /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release
[01:51:23]               c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
travis_time:start:stage0-test
Building stage0 test artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:51:23] running: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--manifest-path" "/checkout/src/libtest/Cargo.toml" "--message-format" "json"
[01:51:24]    Compiling getopts v0.2.17
[01:51:24]    Compiling getopts v0.2.17
[01:51:24]    Compiling term v0.0.0 (file:///checkout/src/libterm)
[01:51:28]    Compiling test v0.0.0 (file:///checkout/src/libtest)
[01:51:36]     Finished release [optimized] target(s) in 12.93 secs
[01:51:36] updating "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/.libtest.stamp" as "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-test/x86_64-unknown-linux-gnu/release/deps/libtest-b359e0c28acff7c7.rlib" changed
[01:51:36]               c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:51:36]               > TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:51:36] Copying stage0 test from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[01:51:36]                 c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[01:51:36]                 > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest }
[01:51:36] Dirty - /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools
[01:51:36]                 < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Libtest }
[01:51:36]               < TestLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }

[01:51:36] travis_time:end:stage0-test:start=1524674157028184695,finish=1524674170382060798,duration=13353876103


[01:51:36]             < Test { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:51:36]             c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:51:36]             c Std { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[01:51:36] Dirty - /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release
[01:51:36]             c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
travis_time:start:stage0-rustc
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
[01:51:36] running: "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "build" "--target" "x86_64-unknown-linux-gnu" "-j" "4" "--release" "--locked" "--color" "always" "--features" " jemalloc" "--manifest-path" "/checkout/src/rustc/Cargo.toml" "--message-format" "json"
[01:51:37]    Compiling cfg-if v0.1.2
[01:51:37]    Compiling smallvec v0.6.0
[01:51:37]    Compiling stable_deref_trait v1.0.0
[01:51:37]    Compiling serialize v0.0.0 (file:///checkout/src/libserialize)
---
[02:06:28]    Compiling rustc_borrowck v0.0.0 (file:///checkout/src/librustc_borrowck)
[02:06:48]    Compiling rustc_passes v0.0.0 (file:///checkout/src/librustc_passes)
[02:07:56]    Compiling rustc-main v0.0.0 (file:///checkout/src/rustc)
[02:07:57]     Finished release [optimized] target(s) in 979.87 secs
[02:07:57] updating "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/.librustc.stamp" as "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0-rustc/x86_64-unknown-linux-gnu/release/deps/librustc_driver-e2282828bd71aab0.so" changed
[02:07:57]             c Assemble { target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[02:07:57]             > RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[02:07:57] Copying stage0 rustc from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[02:07:57]               c Libdir { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }
[02:07:57]               > CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc }
[02:07:57] Dirty - /checkout/obj/build/x86_64-unknown-linux-gnu/stage0-tools
[02:07:57]               < CleanTools { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", mode: Librustc }
[02:07:57]             < RustcLink { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target_compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu" }

[02:07:57] travis_time:end:stage0-rustc:start=1524674170383158968,finish=1524675150705626438,duration=980322467470


[02:07:57]           < Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[02:07:57]           > CodegenBackend { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" }, target: "x86_64-unknown-linux-gnu", backend: "llvm" }
[02:07:57]             c Rustc { target: "x86_64-unknown-linux-gnu", compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[02:07:57]             c Sysroot { compiler: Compiler { stage: 0, host: "x86_64-unknown-linux-gnu" } }
[02:07:57]             > Llvm { target: "x86_64-unknown-linux-gnu", emscripten: false }
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-gnu
Building LLVM for x86_64-unknown-linux-gnu
[02:07:57] running: "cmake" "/checkout/src/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
[02:07:57] -- The CXX compiler identification is Clang 6.0.0
[02:07:57] -- The ASM compiler identification is Clang
[02:07:57] -- Found assembler: /usr/local/bin/sccache
[02:07:57] -- Check for working C compiler: /usr/local/bin/sccache
---
[02:12:32] [ 91%] Building CXX object tools/llvm-cxxfilt/CMakeFiles/llvm-cxxfilt.dir/llvm-cxxfilt.cpp.o
[02:12:32] [ 91%] Linking CXX executable ../../bin/llvm-cov
[02:12:33] [ 91%] Built target llvm-cov
[02:12:33] [ 91%] Building CXX object tools/dsymutil/CMakeFiles/llvm-dsymutil.dir/MachODebugMapParser.cpp.o
[02:12:33] /checkout/src/llvm/tools/dsymutil/DwarfLinker.cpp:1762:29: error: use of undeclared identifier 'PATH_MAX'
[02:12:33]               char RealPath[PATH_MAX + 1];
[02:12:33]                             ^
[02:12:33] /checkout/src/llvm/tools/dsymutil/DwarfLinker.cpp:1763:24: error: use of undeclared identifier 'PATH_MAX'
[02:12:33]               RealPath[PATH_MAX] = 0;
[02:12:33] 2 errors generated.
[02:12:33] 2 errors generated.
[02:12:33] gmake[2]: *** [tools/dsymutil/CMakeFiles/llvm-dsymutil.dir/DwarfLinker.cpp.o] Error 1
[02:12:33] gmake[2]: *** Waiting for unfinished jobs....
[02:12:33] [ 92%] Building CXX object tools/llvm-demangle-fuzzer/CMakeFiles/llvm-demangle-fuzzer.dir/DummyDemanglerFuzzer.cpp.o
[02:12:35] [ 92%] Linking CXX executable ../../bin/llvm-cxxfilt
[02:12:35] [ 92%] Built target llvm-cxxfilt
[02:12:35] [ 92%] Building CXX object tools/llvm-demangle-fuzzer/CMakeFiles/llvm-demangle-fuzzer.dir/llvm-demangle-fuzzer.cpp.o
---
[02:12:36] [ 92%] Linking CXX executable ../../bin/llvm-cxxdump
[02:12:37] [ 92%] Built target llvm-cxxdump
[02:12:37] Scanning dependencies of target llvm-dwarfdump
[02:12:37] [ 92%] Building CXX object tools/llvm-dwarfdump/CMakeFiles/llvm-dwarfdump.dir/Statistics.cpp.o
[02:12:38] gmake[1]: *** [tools/dsymutil/CMakeFiles/llvm-dsymutil.dir/all] Error 2
[02:12:38] gmake[1]: *** Waiting for unfinished jobs....
[02:12:40] [ 92%] Building CXX object tools/llvm-diff/CMakeFiles/llvm-diff.dir/DiffConsumer.cpp.o
[02:12:41] [ 92%] Building CXX object tools/llvm-diff/CMakeFiles/llvm-diff.dir/DiffLog.cpp.o
[02:12:42] [ 92%] Linking CXX executable ../../bin/llvm-dis
[02:12:43] [ 92%] Built target llvm-dis
[02:12:43] [ 92%] Built target llvm-dis
[02:12:43] [ 92%] Building CXX object tools/llvm-diff/CMakeFiles/llvm-diff.dir/DifferenceEngine.cpp.o
[02:12:44] [ 93%] Linking CXX executable ../../bin/llvm-dwarfdump
[02:12:44] [ 93%] Built target llvm-dwarfdump
[02:12:47] [ 93%] Linking CXX executable ../../bin/llvm-diff
[02:12:47] [ 93%] Built target llvm-diff
[02:12:47] gmake: *** [all] Error 2
[02:12:47] command did not execute successfully, got: exit code: 2
[02:12:47] 
[02:12:47] 
[02:12:47] build script failed, must exit now', /cargo/registry/src/github.com-1ecc6299db9ec823/cmake-0.1.30/src/lib.rs:643:5
[02:12:47]  finished in 290.284
[02:12:47] Traceback (most recent call last):
[02:12:47] travis_fold:end:llvm


[02:12:47] travis_time:end:llvm:start=1524675150708209042,finish=1524675440992767427,duration=290284558385

[02:12:47]   File "../x.py", line 20, in <module>
[02:12:47]     bootstrap.main()
[02:12:47]   File "/checkout/src/bootstrap/bootstrap.py", line 800, in main
[02:12:47]     bootstrap(help_triggered)
[02:12:47]   File "/checkout/src/bootstrap/bootstrap.py", line 791, in bootstrap
[02:12:47]     run(args, env=env, verbose=build.verbose)
[02:12:47]   File "/checkout/src/bootstrap/bootstrap.py", line 148, in run
[02:12:47]     raise RuntimeError(err)
[02:12:47] RuntimeError: failed to run: /checkout/obj/build/bootstrap/debug/bootstrap dist --host x86_64-unknown-linux-gnu --target x86_64-unknown-linux-gnu -v

The command "stamp sh -x -c "$RUN_SCRIPT"" exited with 1.
travis_time:start:0000e8e4
$ date && (curl -fs --head https://google.com | grep ^Date: | sed 's/Date: //g' || true)
