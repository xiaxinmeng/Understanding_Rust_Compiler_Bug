plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:16d2738a
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:05:10] + hide_output make install
[00:05:10] + set +x
[00:05:29] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:05:29] + cd ..
[00:05:29] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:05:29] + rm -rf openssl-1.0.2k
[00:05:29] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:05:30] Removing intermediate container 189c50ee2983
[00:05:30] Step 14/41 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:05:30]  ---> 9caa9a733da9
[00:05:30] Step 15/41 : RUN ./build-curl.sh
[00:05:30] Step 15/41 : RUN ./build-curl.sh
[00:05:30]  ---> Running in 33a966c9af43
[00:05:30] + source shared.sh
[00:05:30] + VERSION=7.51.0
[00:05:30] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:05:32]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:05:32]                                  Dload  Upload   Total   Spent    Left  Speed
[00:05:33] 
  0 2509k    0 14215    0     0   9469      0  0:04:31  0:00:01  0:04:30  9469
  0 2509k    0 14215    0     0   9469      0  0:04:31  0:00:01  0:04:30  9469
  4 2509k    4  114k    0     0  62749      0  0:00:40  0:00:01  0:00:39  271k
100 2509k  100 2509k    0     0   939k      0  0:00:02  0:00:02 --:--:-- 2131k
[00:05:33] + mkdir curl-build
[00:05:33] + cd curl-build
[00:05:33] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:05:54] + hide_output make -j10
[00:05:54] + set +x
[00:06:06] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:06:06] + hide_output make install
---
 94 82.1M   94 77.8M    0     0  3563k      0  0:00:23  0:00:22  0:00:01 3230k
 98 82.1M   98 81.2M    0     0  3559k      0  0:00:23  0:00:23 --:--:-- 3369k
100 82.1M  100 82.1M    0     0  3558k      0  0:00:23  0:00:23 --:--:-- 3405k
[00:09:49] + cd gcc-4.8.5
[00:09:49] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:09:49] --2018-12-23 13:31:25--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:09:49] Resolving gcc.gnu.org... 209.132.180.131
[00:09:49] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:09:49] HTTP request sent, awaiting response... 200 OK
---
[00:43:01]  ---> 1e771b42a142
[00:43:01] Step 25/41 : RUN ./build-clang.sh
[00:43:01]  ---> Running in fc2fb0fd0116
[00:43:02] + source shared.sh
[00:43:02] + LLVM=032b00a5404865765cda7db3039f39d54964d8b0
[00:43:02] + LLD=3e4aa4e8671523321af51449e0569f455ef3ad43
[00:43:02] + CLANG=a6b9739069763243020f4ea6fe586bc135fde1f9
[00:43:02] + mkdir clang
[00:43:02] + cd clang
[00:43:02] + curl -L https://github.com/llvm-mirror/llvm/archive/032b00a5404865765cda7db3039f39d54964d8b0.tar.gz
[00:43:02] + tar xzf - --strip-components=1
[00:43:02]                                  Dload  Upload   Total   Spent    Left  Speed
[00:43:02] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100   158    0   158    0     0    850      0 --:--:-- --:--:-- --:--:--   892
---
100 7072k    0 7072k    0     0  4187k      0 --:--:--  0:00:01 --:--:-- 5821k
100 11.9M    0 11.9M    0     0  4547k      0 --:--:--  0:00:02 --:--:-- 5487k
100 17.1M    0 17.1M    0     0  4968k      0 --:--:--  0:00:03 --:--:-- 5734k
[00:43:14] + mkdir -p tools/lld
[00:43:14] + curl -L https://github.com/llvm-mirror/lld/archive/3e4aa4e8671523321af51449e0569f455ef3ad43.tar.gz
[00:43:14] + tar zxf - --strip-components=1 -C tools/lld
[00:43:14]                                  Dload  Upload   Total   Spent    Left  Speed
[00:43:14] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[00:43:15] + cd clang-build
[00:43:15] + INC=/rustroot/include
[00:43:15] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:43:15] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:43:15] + hide_output cmake .. -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:43:45] Sun Dec 23 14:05:21 UTC 2018 - building ...
[00:43:49] + hide_output make -j10
[00:43:49] + set +x
[00:44:19] Sun Dec 23 14:05:56 UTC 2018 - building ...
---
[01:45:49]  ---> 079a7ed0e0ce
[01:45:49] Step 32/41 : RUN ./build-perl.sh
[01:45:49]  ---> Running in 019f8cf47fab
[01:45:49] + source shared.sh
[01:45:49] + curl https://www.cpan.org/src/5.0/perl-5.28.0.tar.gz
[01:45:49]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:45:49]                                  Dload  Upload   Total   Spent    Left  Speed
[01:45:50] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[02:37:33] In file included from /checkout/src/llvm-emscripten/tools/lli/lli.cpp:28:
[02:37:33] /checkout/src/llvm-emscripten/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetClient.h:74:5: warning: explicitly defaulted move assignment operator is implicitly deleted [-Wdefaulted-function-deleted]
[02:37:33]     operator=(RemoteRTDyldMemoryManager &&) = default;
[02:37:33]     ^
[02:37:33] /checkout/src/llvm-emscripten/include/llvm/ExecutionEngine/Orc/OrcRemoteTargetClient.h:315:28: note: move assignment operator of 'RemoteRTDyldMemoryManager' is implicitly deleted because field 'Client' is of reference type 'llvm::orc::remote::OrcRemoteTargetClient &'
[02:37:33]     OrcRemoteTargetClient &Client;
[02:37:33] 1 warning generated.
[02:37:33] [ 99%] Building CXX object tools/lli/CMakeFiles/lli.dir/OrcLazyJIT.cpp.o
[02:37:33] [ 99%] Building CXX object tools/dsymutil/CMakeFiles/llvm-dsymutil.dir/MachODebugMapParser.cpp.o
[02:37:33] [ 99%] Linking CXX executable ../../bin/lli
