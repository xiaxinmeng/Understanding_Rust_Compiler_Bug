plain
travis_fold:end:services

travis_fold:start:git.checkout
travis_time:start:2d5a5ba8
$ git clone --depth=2 --branch=try https://github.com/rust-lang/rust.git rust-lang/rust
---
[00:00:49] 
[00:00:49] Total download size: 4.9 M
[00:00:49] Downloading Packages:
[00:00:57] --------------------------------------------------------------------------------
[00:00:57] Total                                           579 kB/s | 4.9 MB     00:08     
[00:00:57] warning: rpmts_HdrFromFdno: Header V3 DSA signature: NOKEY, key ID e8562897
[00:00:57] Importing GPG key 0xE8562897 "CentOS-5 Key (CentOS 5 Official Signing Key) <centos-5-key@centos.org>" from /etc/pki/rpm-gpg/RPM-GPG-KEY-CentOS-5
[00:00:57] Running Transaction Test
[00:00:57] Finished Transaction Test
[00:00:57] Transaction Test Succeeded
[00:00:57] Running Transaction
---
[00:03:39] + hide_output make install
[00:03:39] + set +x
[00:03:57] shared.sh: line 11:   351 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:03:57] + cd ..
[00:03:57] + rm -rf openssl-1.0.2k
[00:03:58] ./build-openssl.sh: line 25:  4114 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/openssl-1.0.2k)
[00:03:58] + ln -nsf /etc/pki/tls/cert.pem /rustroot/ssl/
[00:03:58] Removing intermediate container 6273379efc5c
[00:03:58] Step 14/38 : COPY dist-x86_64-linux/build-curl.sh /tmp/
[00:03:58]  ---> 062ae58048cd
[00:03:58] Step 15/38 : RUN ./build-curl.sh
[00:03:58] Step 15/38 : RUN ./build-curl.sh
[00:03:58]  ---> Running in 4c70569f0294
[00:03:59] + source shared.sh
[00:03:59] + VERSION=7.51.0
[00:03:59] + curl http://cool.haxx.se/download/curl-7.51.0.tar.bz2
[00:04:00]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[00:04:00]                                  Dload  Upload   Total   Spent    Left  Speed
[00:04:02] 
  0 2509k    0  1183    0     0    754      0  0:56:48  0:00:01  0:56:47   754
  0 2509k    0  1183    0     0    754      0  0:56:48  0:00:01  0:56:47   754
  9 2509k    9  227k    0     0   107k      0  0:00:23  0:00:02  0:00:21  414k
 64 2509k   64 1606k    0     0   524k      0  0:00:04  0:00:03  0:00:01 1073k
100 2509k  100 2509k    0     0   703k      0  0:00:03  0:00:03 --:--:-- 1255k
[00:04:02] + mkdir curl-build
[00:04:02] + cd curl-build
[00:04:02] + hide_output ../curl-7.51.0/configure --prefix=/rustroot --with-ssl=/rustroot --disable-sspi --disable-gopher --disable-smtp --disable-smb --disable-imap --disable-pop3 --disable-tftp --disable-telnet --disable-manual --disable-dict --disable-rtsp --disable-ldaps --disable-ldap
[00:04:24] + hide_output make -j10
[00:04:24] + set +x
[00:04:36] shared.sh: line 11:    12 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[00:04:36] + hide_output make install
---
 91 82.1M   91 75.2M    0     0  4032k      0  0:00:20  0:00:19  0:00:01 3345k
 95 82.1M   95 78.7M    0     0  4012k      0  0:00:20  0:00:20 --:--:-- 3333k
100 82.1M  100 82.1M    0     0  4004k      0  0:00:21  0:00:21 --:--:-- 3577k
[00:08:16] + cd gcc-4.8.5
[00:08:16] + sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
[00:08:16] --2018-08-10 10:43:10--  http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-2.4.2.tar.bz2
[00:08:16] Resolving gcc.gnu.org... 209.132.180.131
[00:08:16] Connecting to gcc.gnu.org|209.132.180.131|:80... connected.
[00:08:16] HTTP request sent, awaiting response... 200 OK
---
[00:41:38]  ---> 18e18a0dae2a
[00:41:38] Step 25/38 : RUN ./build-clang.sh
[00:41:38]  ---> Running in 1a990b60df57
[00:41:38] + source shared.sh
[00:41:38] + LLVM=6.0.0
[00:41:38] + mkdir clang
[00:41:38] + cd clang
[00:41:38] + curl https://releases.llvm.org/6.0.0/llvm-6.0.0.src.tar.xz
[00:41:38] + xz -d
[00:41:38] + tar xf -
[00:41:38]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:41] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
  3 24.1M    3  960k    0     0  3293k      0  0:00:07 --:--:--  0:00:07 3921k
  3 24.1M    3  960k    0     0  3293k      0  0:00:07 --:--:--  0:00:07 3921k
 41 24.1M   41 10.1M    0     0  8012k      0  0:00:03  0:00:01  0:00:02 8317k
 75 24.1M   75 18.2M    0     0  8166k      0  0:00:03  0:00:02  0:00:01 8336k
100 24.1M  100 24.1M    0     0  8284k      0  0:00:02  0:00:02 --:--:-- 8416k
[00:41:41] + cd llvm-6.0.0.src
[00:41:41] + mkdir -p tools/clang
[00:41:41] + curl https://releases.llvm.org/6.0.0/cfe-6.0.0.src.tar.xz
[00:41:41] + xz -d
[00:41:41] + tar xf - -C tools/clang --strip-components=1
[00:41:41]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:42] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
 11 11.4M   11 1341k    0     0  4510k      0  0:00:02 --:--:--  0:00:02 4531k
 11 11.4M   11 1341k    0     0  4510k      0  0:00:02 --:--:--  0:00:02 4531k
 79 11.4M   79 9277k    0     0  7145k      0  0:00:01  0:00:01 --:--:-- 7152k
100 11.4M  100 11.4M    0     0  7622k      0  0:00:01  0:00:01 --:--:-- 7628k
[00:41:42] + mkdir -p tools/lld
[00:41:42] + curl https://releases.llvm.org/6.0.0/lld-6.0.0.src.tar.xz
[00:41:42] + xz -d
[00:41:42] + tar xf - -C tools/lld --strip-components=1
[00:41:42]                                  Dload  Upload   Total   Spent    Left  Speed
[00:41:43] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
100  772k  100  772k    0     0  3335k      0 --:--:-- --:--:-- --:--:-- 3385k
100  772k  100  772k    0     0  3335k      0 --:--:-- --:--:-- --:--:-- 3385k
[00:41:43] + mkdir ../clang-build
[00:41:43] + cd ../clang-build
[00:41:43] + INC=/rustroot/include
[00:41:43] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed
[00:41:43] + INC=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:41:43] + hide_output cmake ../llvm-6.0.0.src -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DLLVM_TARGETS_TO_BUILD=X86 -DC_INCLUDE_DIRS=/rustroot/include:/rustroot/lib/gcc/x86_64-unknown-linux-gnu/4.8.5/include-fixed:/usr/include
[00:42:12] + hide_output make -j10
[00:42:12] + set +x
[00:42:42] Fri Aug 10 11:17:36 UTC 2018 - building ...
[00:43:12] Fri Aug 10 11:18:06 UTC 2018 - building ...
---
[01:33:48] + hide_output make install
[01:33:48] + set +x
[01:33:59] shared.sh: line 11:  1363 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:33:59] + cd ../..
[01:33:59] + rm -rf clang
[01:34:01] ./build-clang.sh: line 70: 15238 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"  (wd: /tmp/clang/clang-build)
[01:34:22] Removing intermediate container 1a990b60df57
[01:34:22] Step 26/38 : ENV CC clang CXX clang++
[01:34:23]  ---> Running in ac9fb760d824
[01:34:23]  ---> 26887c343dd5
[01:34:23]  ---> 26887c343dd5
[01:34:23] Removing intermediate container ac9fb760d824
[01:34:23] Step 27/38 : COPY dist-x86_64-linux/build-git.sh /tmp/
[01:34:23]  ---> aa923a96c70c
[01:34:23] Step 28/38 : RUN ./build-git.sh
[01:34:23]  ---> Running in db8ebb75c654
[01:34:23] + source shared.sh
[01:34:23] + curl -L https://www.kernel.org/pub/software/scm/git/git-2.10.0.tar.gz
[01:34:23]   % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
[01:34:23]                                  Dload  Upload   Total   Spent    Left  Speed
[01:34:24] 
  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
[01:35:24] + set +x
[01:35:25] + hide_output make INSTALL_HDR_PATH=dest headers_install
[01:35:25] + set +x
[01:35:29] shared.sh: line 11:    10 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
[01:35:29] + find dest/include '(' -name .install -o -name ..install.cmd ')' -delete
[01:35:29] + yes
[01:35:29] + yes
[01:35:29] + cp -fr dest/include/asm dest/include/asm-generic dest/include/drm dest/include/linux dest/include/mtd dest/include/rdma dest/include/scsi dest/include/sound dest/include/video dest/include/xen /usr/include
[01:35:29] + rm -rf linux-3.2.84
[01:35:30]  ---> 5a018c766feb
[01:35:30] Removing intermediate container 027a36af7161
[01:35:30] Step 31/38 : COPY scripts/sccache.sh /scripts/
---
[01:35:33] Step 33/38 : ENV HOSTS x86_64-unknown-linux-gnu
[01:35:33]  ---> Running in 11862d1e8fc7
[01:35:33]  ---> 08b507005827
[01:35:33] Removing intermediate container 11862d1e8fc7
[01:35:33] Step 34/38 : ENV RUST_CONFIGURE_ARGS --enable-full-tools       --enable-sanitizers       --enable-profiler       --enable-compiler-docs       --set target.x86_64-unknown-linux-gnu.linker=clang       --set target.x86_64-unknown-linux-gnu.ar=/rustroot/bin/llvm-ar       --set target.x86_64-unknown-linux-gnu.ranlib=/rustroot/bin/llvm-ranlib       --set llvm.thin-lto=true
[01:35:33]  ---> a70fa0f63dc0
[01:35:33] Removing intermediate container b49d1f2d0949
[01:35:33] Step 35/38 : ENV SCRIPT python2.7 ../x.py dist --host $HOSTS --target $HOSTS
[01:35:33]  ---> Running in d9f82c30ac0a
---
travis_time:start:0fbde54d
configure: processing command line
[01:37:55] configure: 
[01:37:55] configure: target.x86_64-unknown-linux-gnu.linker := clang
[01:37:55] configure: target.x86_64-unknown-linux-gnu.ar := /rustroot/bin/llvm-ar
[01:37:55] configure: target.x86_64-unknown-linux-gnu.ranlib := /rustroot/bin/llvm-ranlib
[01:37:55] configure: llvm.thin-lto        := True
[01:37:55] configure: build.submodules     := False
[01:37:55] configure: build.compiler-docs  := True
[01:37:55] configure: build.profiler       := True
[01:37:55] configure: build.locked-deps    := True
---

[01:53:22] travis_fold:start:llvm
travis_time:start:llvm
Building LLVM for x86_64-unknown-linux-gnu
[01:53:22] running: "cmake" "/checkout/src/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_TARGETS_TO_BUILD=X86;ARM;AArch64;Mips;PowerPC;SystemZ;MSP430;Sparc;NVPTX;Hexagon" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=WebAssembly;RISCV" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_ENABLE_ZLIB=OFF" "-DWITH_POLLY=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=4" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_OCAML_INSTALL_PATH=usr/lib/ocaml" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_C_COMPILER=sccache" "-DCMAKE_C_COMPILER_ARG1=clang" "-DCMAKE_CXX_COMPILER=sccache" "-DCMAKE_CXX_COMPILER_ARG1=clang++" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_BUILD_TYPE=Release"
[01:53:23] -- The CXX compiler identification is Clang 6.0.0
[01:53:23] -- The ASM compiler identification is Clang
[01:53:23] -- Found assembler: /usr/local/bin/sccache
[01:53:23] -- Check for working C compiler: /usr/local/bin/sccache
---
[01:53:36] Call Stack (most recent call first):
[01:53:36]   CMakeLists.txt:616 (include)
[01:53:36] 
[01:53:36] 
[01:53:36] -- Performing Test CXX_SUPPORTS_CUSTOM_LINKER
[01:53:37] -- Performing Test CXX_SUPPORTS_CUSTOM_LINKER - Success
[01:53:37] -- Performing Test C_SUPPORTS_FPIC - Success
[01:53:37] -- Performing Test CXX_SUPPORTS_FPIC
[01:53:37] -- Performing Test CXX_SUPPORTS_FPIC - Success
[01:53:37] -- Building with -fPIC
---
[02:57:35] [ 96%] Building CXX object tools/llvm-objdump/CMakeFiles/llvm-objdump.dir/ELFDump.cpp.o
[02:57:42] [ 96%] Building CXX object tools/llvm-objdump/CMakeFiles/llvm-objdump.dir/MachODump.cpp.o
[02:58:06] [ 96%] Building CXX object tools/llvm-objdump/CMakeFiles/llvm-objdump.dir/WasmDump.cpp.o
[02:58:10] [ 97%] Linking CXX executable ../../bin/llvm-objdump
The job exceeded the maximum time limit for jobs, and has been terminated.
