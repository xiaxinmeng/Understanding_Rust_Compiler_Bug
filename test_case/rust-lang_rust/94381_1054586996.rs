plain
100 59.8M  100 59.8M    0     0  7597k      0  0:00:08  0:00:08 --:--:-- 8373k
+ cd gcc-7.5.0
+ sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
+ ./contrib/download_prerequisites
2022-02-28 16:59:49 URL:http://gcc.gnu.org/pub/gcc/infrastructure/gmp-6.1.0.tar.bz2 [2383840/2383840] -> "./gmp-6.1.0.tar.bz2" [1]
2022-02-28 16:59:49 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-3.1.4.tar.bz2 [1279284/1279284] -> "./mpfr-3.1.4.tar.bz2" [1]
2022-02-28 16:59:49 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpc-1.0.3.tar.gz [669925/669925] -> "./mpc-1.0.3.tar.gz" [1]
2022-02-28 16:59:49 URL:http://gcc.gnu.org/pub/gcc/infrastructure/isl-0.16.1.tar.bz2 [1626446/1626446] -> "./isl-0.16.1.tar.bz2" [1]
gmp-6.1.0.tar.bz2: OK
mpfr-3.1.4.tar.bz2: OK
mpc-1.0.3.tar.gz: OK
isl-0.16.1.tar.bz2: OK
All prerequisites downloaded successfully.
+ cd ../gcc-build
+ hide_output ../gcc-7.5.0/configure --prefix=/rustroot --enable-languages=c,c++ --disable-gnu-unique-object
+ set +x
++ nproc
---
+ INC=/rustroot/include:/usr/include
+ cmake --version
cmake version 3.14.0

CMake suite maintained and supported by Kitware (kitware.com/cmake).
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DCOMPILER_RT_BUILD_XRAY=OFF -DCOMPILER_RT_BUILD_MEMPROF=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt;bolt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
Mon Feb 28 17:29:17 UTC 2022 - building ...
++ nproc
+ hide_output make -j16
+ set +x
---
shared.sh: line 2:    17 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
+ hide_output make install
+ set +x
shared.sh: line 2:  2456 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
+ ls -lha /rustroot/bin
drwxr-xr-x 1 root root 4.0K Feb 28 17:46 .
drwxr-xr-x 1 root root 4.0K Feb 28 17:28 ..
drwxr-xr-x 1 root root 4.0K Feb 28 17:28 ..
lrwxrwxrwx 1 root root    8 Feb 28 17:24 2to3 -> 2to3-3.9
-rwxr-xr-x 1 root root  100 Feb 28 17:24 2to3-3.9
-rwxr-xr-x 1 root root 4.7M Feb 28 16:59 addr2line
-rwxr-xr-x 1 root root  556 Feb  9 07:15 analyze-build
-rwxr-xr-x 2 root root 4.9M Feb 28 16:59 ar
-rwxr-xr-x 2 root root 6.7M Feb 28 16:59 as
-rwxr-xr-x 1 root root  50M Feb 28 17:44 bugpoint
-rwxr-xr-x 4 root root 4.8M Feb 28 17:22 c++
-rwxr-xr-x 1 root root 4.7M Feb 28 16:59 c++filt
-rwxr-xr-x 1 root root  34M Feb 28 17:45 c-index-test
-rwxr-xr-x 1 root root 5.0K Feb 28 16:58 c_rehash
lrwxrwxrwx 1 root root    3 Feb 28 17:23 cc -> gcc
-rwxr-xr-x 1 root root  13M Feb 28 17:28 ccmake
lrwxrwxrwx 1 root root    8 Feb 28 17:45 clang -> clang-14
lrwxrwxrwx 1 root root    5 Feb 28 17:45 clang++ -> clang
-rwxr-xr-x 1 root root 108M Feb 28 17:44 clang-14
-rwxr-xr-x 1 root root  67M Feb 28 17:44 clang-check
lrwxrwxrwx 1 root root    5 Feb 28 17:45 clang-cl -> clang
lrwxrwxrwx 1 root root    5 Feb 28 17:45 clang-cpp -> clang
-rwxr-xr-x 1 root root  31M Feb 28 17:44 clang-extdef-mapping
-rwxr-xr-x 1 root root 3.0M Feb 28 17:33 clang-format
-rwxr-xr-x 1 root root  44M Feb 28 17:44 clang-linker-wrapper
-rwxr-xr-x 1 root root 545K Feb 28 17:38 clang-nvlink-wrapper
-rwxr-xr-x 1 root root 4.3M Feb 28 17:36 clang-offload-bundler
-rwxr-xr-x 1 root root 2.9M Feb 28 17:39 clang-offload-wrapper
-rwxr-xr-x 1 root root  34M Feb 28 17:41 clang-refactor
-rwxr-xr-x 1 root root  32M Feb 28 17:41 clang-rename
-rwxr-xr-x 1 root root 109M Feb 28 17:44 clang-repl
-rwxr-xr-x 1 root root  80M Feb 28 17:44 clang-scan-deps
-rwxr-xr-x 1 root root  14M Feb 28 17:28 cmake
-rwxr-xr-x 1 root root  14M Feb 28 17:28 cpack
-rwxr-xr-x 1 root root 4.8M Feb 28 17:22 cpp
-rwxr-xr-x 1 root root  15M Feb 28 17:28 ctest
-rwxr-xr-x 1 root root 166K Feb 28 16:59 curl
-rwxr-xr-x 1 root root 5.5K Feb 28 16:59 curl-config
-rwxr-xr-x 1 root root 9.5M Feb 28 17:41 diagtool
-rwxr-xr-x 1 root root  29M Feb 28 17:44 dsymutil
-rwxr-xr-x 1 root root  237 Feb 28 17:24 easy_install-3.9
-rwxr-xr-x 1 root root  78K Feb 28 16:59 elfedit
-rwxr-xr-x 4 root root 4.8M Feb 28 17:22 g++
-rwxr-xr-x 3 root root 4.8M Feb 28 17:22 gcc
-rwxr-xr-x 2 root root 143K Feb 28 17:22 gcc-ar
-rwxr-xr-x 2 root root 143K Feb 28 17:22 gcc-nm
-rwxr-xr-x 2 root root 143K Feb 28 17:22 gcc-ranlib
-rwxr-xr-x 1 root root 4.2M Feb 28 17:22 gcov
-rwxr-xr-x 1 root root 3.1M Feb 28 17:22 gcov-dump
-rwxr-xr-x 1 root root 3.3M Feb 28 17:22 gcov-tool
-rwxr-xr-x 1 root root  23K Feb  9 07:15 git-clang-format
-rwxr-xr-x 1 root root 5.2M Feb 28 16:59 gprof
-rwxr-xr-x 1 root root 9.8K Feb  9 07:15 hmaptool
lrwxrwxrwx 1 root root    7 Feb 28 17:24 idle3 -> idle3.9
-rwxr-xr-x 1 root root   98 Feb 28 17:24 idle3.9
-rwxr-xr-x 1 root root  562 Feb  9 07:15 intercept-build
-rwxr-xr-x 4 root root 6.9M Feb 28 16:59 ld
-rwxr-xr-x 4 root root 6.9M Feb 28 16:59 ld.bfd
lrwxrwxrwx 1 root root    3 Feb 28 17:45 ld.lld -> lld
lrwxrwxrwx 1 root root    3 Feb 28 17:45 ld64.lld -> lld
-rwxr-xr-x 1 root root  44M Feb 28 17:44 llc
-rwxr-xr-x 1 root root  58M Feb 28 17:45 lld
lrwxrwxrwx 1 root root    3 Feb 28 17:45 lld-link -> lld
-rwxr-xr-x 1 root root  41M Feb 28 17:44 lli
lrwxrwxrwx 1 root root   15 Feb 28 17:46 llvm-addr2line -> llvm-symbolizer
-rwxr-xr-x 1 root root 7.2M Feb 28 17:37 llvm-ar
-rwxr-xr-x 1 root root 3.9M Feb 28 17:38 llvm-as
-rwxr-xr-x 1 root root 539K Feb 28 17:35 llvm-bcanalyzer
lrwxrwxrwx 1 root root   12 Feb 28 17:45 llvm-bitcode-strip -> llvm-objcopy
-rwxr-xr-x 1 root root  35M Feb 28 17:45 llvm-bolt
lrwxrwxrwx 1 root root    9 Feb 28 17:45 llvm-bolt-heatmap -> llvm-bolt
lrwxrwxrwx 1 root root    9 Feb 28 17:45 llvm-boltdiff -> llvm-bolt
-rwxr-xr-x 1 root root  29M Feb 28 17:44 llvm-c-test
-rwxr-xr-x 1 root root 3.5M Feb 28 17:38 llvm-cat
-rwxr-xr-x 1 root root  12M Feb 28 17:37 llvm-cfi-verify
-rwxr-xr-x 1 root root 222K Feb 28 17:29 llvm-config
-rwxr-xr-x 1 root root 5.2M Feb 28 17:38 llvm-cov
-rwxr-xr-x 1 root root 466K Feb 28 17:36 llvm-cvtres
-rwxr-xr-x 1 root root 4.2M Feb 28 17:37 llvm-cxxdump
-rwxr-xr-x 1 root root 532K Feb 28 17:32 llvm-cxxfilt
-rwxr-xr-x 1 root root 603K Feb 28 17:35 llvm-cxxmap
-rwxr-xr-x 1 root root 422K Feb 28 17:29 llvm-debuginfod-find
-rwxr-xr-x 1 root root 3.3M Feb 28 17:35 llvm-diff
-rwxr-xr-x 1 root root 2.9M Feb 28 17:35 llvm-dis
lrwxrwxrwx 1 root root    7 Feb 28 17:45 llvm-dlltool -> llvm-ar
-rwxr-xr-x 1 root root 7.4M Feb 28 17:37 llvm-dwarfdump
-rwxr-xr-x 1 root root  28M Feb 28 17:44 llvm-dwp
-rwxr-xr-x 1 root root  30M Feb 28 17:44 llvm-exegesis
-rwxr-xr-x 1 root root 6.4M Feb 28 17:41 llvm-extract
-rwxr-xr-x 1 root root  27M Feb 28 17:44 llvm-gsymutil
-rwxr-xr-x 1 root root 4.4M Feb 28 17:37 llvm-ifs
lrwxrwxrwx 1 root root   12 Feb 28 17:45 llvm-install-name-tool -> llvm-objcopy
-rwxr-xr-x 1 root root  21M Feb 28 17:43 llvm-jitlink
lrwxrwxrwx 1 root root    7 Feb 28 17:45 llvm-lib -> llvm-ar
-rwxr-xr-x 1 root root 4.4M Feb 28 17:37 llvm-libtool-darwin
-rwxr-xr-x 1 root root 4.7M Feb 28 17:41 llvm-link
-rwxr-xr-x 1 root root 4.3M Feb 28 17:44 llvm-lipo
-rwxr-xr-x 1 root root  44M Feb 28 17:44 llvm-lto
-rwxr-xr-x 1 root root  52M Feb 28 17:44 llvm-lto2
-rwxr-xr-x 1 root root 6.7M Feb 28 17:34 llvm-mc
-rwxr-xr-x 1 root root 4.7M Feb 28 17:34 llvm-mca
-rwxr-xr-x 1 root root 6.5M Feb 28 17:34 llvm-ml
-rwxr-xr-x 1 root root 3.7M Feb 28 17:38 llvm-modextract
-rwxr-xr-x 1 root root 250K Feb 28 17:32 llvm-mt
-rwxr-xr-x 1 root root 7.7M Feb 28 17:37 llvm-nm
-rwxr-xr-x 1 root root 5.0M Feb 28 17:37 llvm-objcopy
-rwxr-xr-x 1 root root  11M Feb 28 17:37 llvm-objdump
-rwxr-xr-x 1 root root 770K Feb 28 17:37 llvm-opt-report
lrwxrwxrwx 1 root root   12 Feb 28 17:45 llvm-otool -> llvm-objdump
-rwxr-xr-x 1 root root 7.0M Feb 28 17:37 llvm-pdbutil
-rwxr-xr-x 1 root root 5.8M Feb 28 17:38 llvm-profdata
-rwxr-xr-x 1 root root  12M Feb 28 17:42 llvm-profgen
lrwxrwxrwx 1 root root    7 Feb 28 17:45 llvm-ranlib -> llvm-ar
-rwxr-xr-x 1 root root 711K Feb 28 17:37 llvm-rc
lrwxrwxrwx 1 root root   12 Feb 28 17:46 llvm-readelf -> llvm-readobj
-rwxr-xr-x 1 root root 7.5M Feb 28 17:38 llvm-readobj
-rwxr-xr-x 1 root root  29M Feb 28 17:45 llvm-reduce
-rwxr-xr-x 1 root root 6.8M Feb 28 17:38 llvm-rtdyld
-rwxr-xr-x 1 root root 3.4M Feb 28 17:38 llvm-sim
-rwxr-xr-x 1 root root 4.2M Feb 28 17:37 llvm-size
-rwxr-xr-x 1 root root 4.3M Feb 28 17:39 llvm-split
-rwxr-xr-x 1 root root 2.4M Feb 28 17:38 llvm-stress
-rwxr-xr-x 1 root root 372K Feb 28 17:37 llvm-strings
lrwxrwxrwx 1 root root   12 Feb 28 17:45 llvm-strip -> llvm-objcopy
-rwxr-xr-x 1 root root 6.2M Feb 28 17:37 llvm-symbolizer
-rwxr-xr-x 1 root root 4.3M Feb 28 17:37 llvm-tapi-diff
-rwxr-xr-x 1 root root 3.9M Feb 28 17:30 llvm-tblgen
-rwxr-xr-x 1 root root 4.5M Feb 28 17:38 llvm-tli-checker
-rwxr-xr-x 1 root root 505K Feb 28 17:29 llvm-undname
lrwxrwxrwx 1 root root    7 Feb 28 17:46 llvm-windres -> llvm-rc
-rwxr-xr-x 1 root root 6.7M Feb 28 17:37 llvm-xray
-rwxr-xr-x 1 root root 588K Feb 28 17:31 merge-fdata
-rwxr-xr-x 2 root root 4.7M Feb 28 16:59 nm
-rwxr-xr-x 2 root root 5.4M Feb 28 16:59 objcopy
-rwxr-xr-x 2 root root 6.8M Feb 28 16:59 objdump
-rwxr-xr-x 1 root root 657K Feb 28 16:58 openssl
-rwxr-xr-x 1 root root  52M Feb 28 17:44 opt
lrwxrwxrwx 1 root root    9 Feb 28 17:45 perf2bolt -> llvm-bolt
-rwxr-xr-x 1 root root  228 Feb 28 17:24 pip3
-rwxr-xr-x 1 root root  228 Feb 28 17:24 pip3.9
lrwxrwxrwx 1 root root    8 Feb 28 17:24 pydoc3 -> pydoc3.9
-rwxr-xr-x 1 root root   83 Feb 28 17:24 pydoc3.9
lrwxrwxrwx 1 root root    9 Feb 28 17:24 python3 -> python3.9
lrwxrwxrwx 1 root root   16 Feb 28 17:24 python3-config -> python3.9-config
-rwxr-xr-x 1 root root  18M Feb 28 17:24 python3.9
-rwxr-xr-x 1 root root 3.1K Feb 28 17:24 python3.9-config
-rwxr-xr-x 2 root root 4.9M Feb 28 16:59 ranlib
-rwxr-xr-x 2 root root 1.2M Feb 28 16:59 readelf
-rwxr-xr-x 1 root root  11M Feb 28 17:37 sancov
-rwxr-xr-x 1 root root 6.1M Feb 28 17:37 sanstats
-rwxr-xr-x 1 root root  56K Feb  9 07:15 scan-build
-rwxr-xr-x 1 root root  550 Feb  9 07:15 scan-build-py
-rwxr-xr-x 1 root root 4.6K Feb  9 07:15 scan-view
-rwxr-xr-x 1 root root 4.7M Feb 28 16:59 size
-rwxr-xr-x 1 root root 338K Feb 28 17:29 split-file
-rwxr-xr-x 1 root root 4.7M Feb 28 16:59 strings
-rwxr-xr-x 2 root root 5.4M Feb 28 16:59 strip
-rwxr-xr-x 1 root root 4.3M Feb 28 17:38 verify-uselistorder
lrwxrwxrwx 1 root root    3 Feb 28 17:45 wasm-ld -> lld
-rwxr-xr-x 4 root root 4.8M Feb 28 17:22 x86_64-pc-linux-gnu-c++
-rwxr-xr-x 4 root root 4.8M Feb 28 17:22 x86_64-pc-linux-gnu-g++
-rwxr-xr-x 3 root root 4.8M Feb 28 17:22 x86_64-pc-linux-gnu-gcc
-rwxr-xr-x 3 root root 4.8M Feb 28 17:22 x86_64-pc-linux-gnu-gcc-7.5.0
-rwxr-xr-x 2 root root 143K Feb 28 17:22 x86_64-pc-linux-gnu-gcc-ar
-rwxr-xr-x 2 root root 143K Feb 28 17:22 x86_64-pc-linux-gnu-gcc-nm
-rwxr-xr-x 2 root root 143K Feb 28 17:22 x86_64-pc-linux-gnu-gcc-ranlib
+ cd ../..
+ rm -rf llvm-project
Removing intermediate container b3a84f1bd505
 ---> 916808ae9beb
---
Successfully built adf90b0f49f9
Successfully tagged rust-ci:latest
Built container sha256:adf90b0f49f9e4e3f8a05d4505562113d037eeb675d8faba34eb01fdc4dea220
Uploading finished image to https://ci-caches.rust-lang.org/docker/a60c0a5b65bd51a4040a45d5e2af022a5911ea96437e3785c90d79f85efcc5c49e5018540e37be4bf52c360fc17b0613a062756f25b3cc7752a78ebd41a6c433
upload failed: - to s3://rust-lang-ci-sccache2/docker/a60c0a5b65bd51a4040a45d5e2af022a5911ea96437e3785c90d79f85efcc5c49e5018540e37be4bf52c360fc17b0613a062756f25b3cc7752a78ebd41a6c433 Unable to locate credentials
useradd: warning: the home directory already exists.
Not copying any file from skel directory into it.
[CI_JOB_NAME=dist-x86_64-linux]
---
Preparing cargo
[2022-02-28T18:39:55Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-02-28T18:39:55Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-02-28T18:39:55Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-02-28T18:39:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmZS0C5#cargo:0.29.0" "--release" "--lib" "--" "--skip-this-rustc"
[2022-02-28T18:39:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpeII0Zm#cargo:0.29.0" "--profile" "check" "--lib" "--" "--skip-this-rustc"
[2022-02-28T18:39:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpHisYnm#cargo:0.29.0" "--lib" "--" "--skip-this-rustc"
[2022-02-28T18:41:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:41:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-28T18:41:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQyxzdq#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:41:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:41:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:41:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQyxzdq#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpQyxzdq/incremental-state"
[2022-02-28T18:41:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:41:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQyxzdq#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpQyxzdq/incremental-state"
[2022-02-28T18:41:44Z DEBUG collector::execute] applying println to "/tmp/.tmpQyxzdq"
[2022-02-28T18:41:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:41:44Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:41:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQyxzdq#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpQyxzdq/incremental-state"
[2022-02-28T18:41:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:41:52Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-02-28T18:41:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxHUmwi#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:42:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:42:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:42:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxHUmwi#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpxHUmwi/incremental-state"
[2022-02-28T18:42:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:42:49Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxHUmwi#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpxHUmwi/incremental-state"
[2022-02-28T18:42:55Z DEBUG collector::execute] applying println to "/tmp/.tmpxHUmwi"
[2022-02-28T18:42:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:42:55Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:42:55Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpxHUmwi#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpxHUmwi/incremental-state"
[2022-02-28T18:43:09Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:43:09Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-02-28T18:43:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFWDVj5#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:43:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:43:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:43:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFWDVj5#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFWDVj5/incremental-state"
[2022-02-28T18:44:18Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:44:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFWDVj5#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFWDVj5/incremental-state"
[2022-02-28T18:44:24Z DEBUG collector::execute] applying println to "/tmp/.tmpFWDVj5"
[2022-02-28T18:44:24Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:44:24Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:44:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFWDVj5#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFWDVj5/incremental-state"
Preparing ctfe-stress-4
[2022-02-28T18:44:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-02-28T18:44:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-02-28T18:44:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-02-28T18:44:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-02-28T18:44:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphsqUVT#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-02-28T18:44:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpimR1Kc#ctfe-stress-4:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-02-28T18:44:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpG87J7e#ctfe-stress-4:0.1.0" "--" "--skip-this-rustc"
[2022-02-28T18:44:43Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:44:43Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-28T18:44:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCrB3gU#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:45:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:45:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:45:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCrB3gU#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCrB3gU/incremental-state"
[2022-02-28T18:45:34Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:45:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCrB3gU#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCrB3gU/incremental-state"
[2022-02-28T18:45:34Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:45:34Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-02-28T18:45:34Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnYlmhd#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:45:58Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:45:58Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:45:58Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnYlmhd#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpnYlmhd/incremental-state"
[2022-02-28T18:46:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:46:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpnYlmhd#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpnYlmhd/incremental-state"
[2022-02-28T18:46:26Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:46:26Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-02-28T18:46:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpA23UYY#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:46:49Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
---
[2022-02-28T18:47:16Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:47:16Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-28T18:47:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprkrZA9#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:47:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:47:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprkrZA9#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmprkrZA9/incremental-state"
[2022-02-28T18:47:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:47:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprkrZA9#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmprkrZA9/incremental-state"
[2022-02-28T18:47:18Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:47:18Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-02-28T18:47:18Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcpvHrI#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:47:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:47:19Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:47:19Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcpvHrI#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpcpvHrI/incremental-state"
[2022-02-28T18:47:20Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:47:20Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcpvHrI#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpcpvHrI/incremental-state"
[2022-02-28T18:47:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:47:21Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-02-28T18:47:21Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpitb8WA#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:47:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:47:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:47:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpitb8WA#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpitb8WA/incremental-state"
[2022-02-28T18:47:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:47:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpitb8WA#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpitb8WA/incremental-state"
Preparing inflate
[2022-02-28T18:47:23Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-02-28T18:47:23Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-02-28T18:47:23Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-02-28T18:47:23Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-02-28T18:47:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyu0DWG#inflate:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-02-28T18:47:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFy6xdE#inflate:0.1.0" "--" "--skip-this-rustc"
[2022-02-28T18:47:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbqLlUz#inflate:0.1.0" "--release" "--" "--skip-this-rustc"
[2022-02-28T18:47:24Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:47:24Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-28T18:47:24Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-28T18:47:24Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFLoLbE#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:47:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:47:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFLoLbE#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFLoLbE/incremental-state"
[2022-02-28T18:47:29Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:47:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFLoLbE#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFLoLbE/incremental-state"
[2022-02-28T18:47:29Z DEBUG collector::execute] applying println to "/tmp/.tmpFLoLbE"
[2022-02-28T18:47:29Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:47:29Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:47:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFLoLbE#inflate:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFLoLbE/incremental-state"
[2022-02-28T18:47:32Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:47:32Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-02-28T18:47:32Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCuMCrO#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:47:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:47:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:47:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCuMCrO#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCuMCrO/incremental-state"
[2022-02-28T18:47:38Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:47:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCuMCrO#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCuMCrO/incremental-state"
[2022-02-28T18:47:39Z DEBUG collector::execute] applying println to "/tmp/.tmpCuMCrO"
[2022-02-28T18:47:39Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:47:39Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:47:39Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpCuMCrO#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpCuMCrO/incremental-state"
[2022-02-28T18:47:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:47:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-02-28T18:47:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVT2xqU#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:47:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:47:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:47:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVT2xqU#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpVT2xqU/incremental-state"
[2022-02-28T18:47:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:47:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVT2xqU#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpVT2xqU/incremental-state"
[2022-02-28T18:47:54Z DEBUG collector::execute] applying println to "/tmp/.tmpVT2xqU"
[2022-02-28T18:47:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:47:54Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-02-28T18:47:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpVT2xqU#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpVT2xqU/incremental-state"
Preparing match-stress-enum
[2022-02-28T18:47:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-02-28T18:47:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-02-28T18:47:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-02-28T18:48:00Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:48:00Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-28T18:48:00Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpblBuPg#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:48:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:48:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpblBuPg#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpblBuPg/incremental-state"
[2022-02-28T18:48:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:48:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpblBuPg#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpblBuPg/incremental-state"
[2022-02-28T18:48:04Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:48:04Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-02-28T18:48:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFFDqpe#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:48:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:48:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:48:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFFDqpe#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFFDqpe/incremental-state"
[2022-02-28T18:48:08Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:48:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpFFDqpe#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpFFDqpe/incremental-state"
[2022-02-28T18:48:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:48:08Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-02-28T18:48:08Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMxVYGm#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:48:10Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:48:10Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:48:10Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMxVYGm#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMxVYGm/incremental-state"
[2022-02-28T18:48:13Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:48:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpMxVYGm#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpMxVYGm/incremental-state"
Preparing token-stream-stress
[2022-02-28T18:48:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-02-28T18:48:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-02-28T18:48:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-02-28T18:48:14Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:48:14Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-02-28T18:48:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZuyt4A#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:48:14Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-02-28T18:48:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZuyt4A#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpZuyt4A/incremental-state"
[2022-02-28T18:48:14Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-02-28T18:48:14Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZuyt4A#token-stream-stress:0.0.0" "--profile" "check" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpZuyt4A/incremental-state"
[2022-02-28T18:48:15Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-02-28T18:48:15Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-02-28T18:48:15Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPuNt4P#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-02-28T18:48:15Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
