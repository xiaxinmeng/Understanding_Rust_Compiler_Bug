plain
100 59.8M  100 59.8M    0     0  6741k      0  0:00:09  0:00:09 --:--:-- 8454k
+ cd gcc-7.5.0
+ sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
+ ./contrib/download_prerequisites
2022-03-01 21:59:28 URL:http://gcc.gnu.org/pub/gcc/infrastructure/gmp-6.1.0.tar.bz2 [2383840/2383840] -> "./gmp-6.1.0.tar.bz2" [1]
2022-03-01 21:59:29 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-3.1.4.tar.bz2 [1279284/1279284] -> "./mpfr-3.1.4.tar.bz2" [1]
2022-03-01 21:59:30 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpc-1.0.3.tar.gz [669925/669925] -> "./mpc-1.0.3.tar.gz" [1]
2022-03-01 21:59:31 URL:http://gcc.gnu.org/pub/gcc/infrastructure/isl-0.16.1.tar.bz2 [1626446/1626446] -> "./isl-0.16.1.tar.bz2" [1]
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
Tue Mar 1 22:32:16 UTC 2022 - building ...
++ nproc
+ hide_output make -j16
+ set +x
---
shared.sh: line 2:    17 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
+ hide_output make install
+ set +x
shared.sh: line 2:  2456 Terminated              bash -c "while true; do sleep 30; echo \$(date) - building ...; done"
+ ls -lha /rustroot/bin
drwxr-xr-x 1 root root 4.0K Mar  1 22:55 .
drwxr-xr-x 1 root root 4.0K Mar  1 22:31 ..
drwxr-xr-x 1 root root 4.0K Mar  1 22:31 ..
lrwxrwxrwx 1 root root    8 Mar  1 22:26 2to3 -> 2to3-3.9
-rwxr-xr-x 1 root root  100 Mar  1 22:26 2to3-3.9
-rwxr-xr-x 1 root root 4.7M Mar  1 21:59 addr2line
-rwxr-xr-x 1 root root  556 Feb  9 07:15 analyze-build
-rwxr-xr-x 2 root root 4.9M Mar  1 21:59 ar
-rwxr-xr-x 2 root root 6.7M Mar  1 21:59 as
-rwxr-xr-x 1 root root  50M Mar  1 22:54 bugpoint
-rwxr-xr-x 4 root root 4.8M Mar  1 22:25 c++
-rwxr-xr-x 1 root root 4.7M Mar  1 21:59 c++filt
-rwxr-xr-x 1 root root  34M Mar  1 22:55 c-index-test
-rwxr-xr-x 1 root root 5.0K Mar  1 21:58 c_rehash
lrwxrwxrwx 1 root root    3 Mar  1 22:25 cc -> gcc
-rwxr-xr-x 1 root root  13M Mar  1 22:31 ccmake
lrwxrwxrwx 1 root root    8 Mar  1 22:55 clang -> clang-14
lrwxrwxrwx 1 root root    5 Mar  1 22:55 clang++ -> clang
-rwxr-xr-x 1 root root 108M Mar  1 22:54 clang-14
-rwxr-xr-x 1 root root  67M Mar  1 22:53 clang-check
lrwxrwxrwx 1 root root    5 Mar  1 22:55 clang-cl -> clang
lrwxrwxrwx 1 root root    5 Mar  1 22:55 clang-cpp -> clang
-rwxr-xr-x 1 root root  31M Mar  1 22:53 clang-extdef-mapping
-rwxr-xr-x 1 root root 3.0M Mar  1 22:38 clang-format
-rwxr-xr-x 1 root root  44M Mar  1 22:53 clang-linker-wrapper
-rwxr-xr-x 1 root root 545K Mar  1 22:45 clang-nvlink-wrapper
-rwxr-xr-x 1 root root 4.3M Mar  1 22:43 clang-offload-bundler
-rwxr-xr-x 1 root root 2.9M Mar  1 22:46 clang-offload-wrapper
-rwxr-xr-x 1 root root  34M Mar  1 22:49 clang-refactor
-rwxr-xr-x 1 root root  32M Mar  1 22:49 clang-rename
-rwxr-xr-x 1 root root 109M Mar  1 22:53 clang-repl
-rwxr-xr-x 1 root root  80M Mar  1 22:53 clang-scan-deps
-rwxr-xr-x 1 root root  14M Mar  1 22:31 cmake
-rwxr-xr-x 1 root root  14M Mar  1 22:31 cpack
-rwxr-xr-x 1 root root 4.8M Mar  1 22:25 cpp
-rwxr-xr-x 1 root root  15M Mar  1 22:31 ctest
-rwxr-xr-x 1 root root 166K Mar  1 21:58 curl
-rwxr-xr-x 1 root root 5.5K Mar  1 21:58 curl-config
-rwxr-xr-x 1 root root 9.5M Mar  1 22:48 diagtool
-rwxr-xr-x 1 root root  29M Mar  1 22:54 dsymutil
-rwxr-xr-x 1 root root  237 Mar  1 22:26 easy_install-3.9
-rwxr-xr-x 1 root root  78K Mar  1 21:59 elfedit
-rwxr-xr-x 4 root root 4.8M Mar  1 22:25 g++
-rwxr-xr-x 3 root root 4.8M Mar  1 22:25 gcc
-rwxr-xr-x 2 root root 143K Mar  1 22:25 gcc-ar
-rwxr-xr-x 2 root root 143K Mar  1 22:25 gcc-nm
-rwxr-xr-x 2 root root 143K Mar  1 22:25 gcc-ranlib
-rwxr-xr-x 1 root root 4.2M Mar  1 22:25 gcov
-rwxr-xr-x 1 root root 3.1M Mar  1 22:25 gcov-dump
-rwxr-xr-x 1 root root 3.3M Mar  1 22:25 gcov-tool
-rwxr-xr-x 1 root root  23K Feb  9 07:15 git-clang-format
-rwxr-xr-x 1 root root 5.2M Mar  1 21:59 gprof
-rwxr-xr-x 1 root root 9.8K Feb  9 07:15 hmaptool
lrwxrwxrwx 1 root root    7 Mar  1 22:26 idle3 -> idle3.9
-rwxr-xr-x 1 root root   98 Mar  1 22:26 idle3.9
-rwxr-xr-x 1 root root  562 Feb  9 07:15 intercept-build
-rwxr-xr-x 4 root root 6.9M Mar  1 21:59 ld
-rwxr-xr-x 4 root root 6.9M Mar  1 21:59 ld.bfd
lrwxrwxrwx 1 root root    3 Mar  1 22:55 ld.lld -> lld
lrwxrwxrwx 1 root root    3 Mar  1 22:55 ld64.lld -> lld
-rwxr-xr-x 1 root root  44M Mar  1 22:53 llc
-rwxr-xr-x 1 root root  58M Mar  1 22:55 lld
lrwxrwxrwx 1 root root    3 Mar  1 22:55 lld-link -> lld
-rwxr-xr-x 1 root root  41M Mar  1 22:53 lli
lrwxrwxrwx 1 root root   15 Mar  1 22:55 llvm-addr2line -> llvm-symbolizer
-rwxr-xr-x 1 root root 7.2M Mar  1 22:43 llvm-ar
-rwxr-xr-x 1 root root 3.9M Mar  1 22:45 llvm-as
-rwxr-xr-x 1 root root 539K Mar  1 22:41 llvm-bcanalyzer
lrwxrwxrwx 1 root root   12 Mar  1 22:55 llvm-bitcode-strip -> llvm-objcopy
-rwxr-xr-x 1 root root  35M Mar  1 22:55 llvm-bolt
lrwxrwxrwx 1 root root    9 Mar  1 22:55 llvm-bolt-heatmap -> llvm-bolt
lrwxrwxrwx 1 root root    9 Mar  1 22:55 llvm-boltdiff -> llvm-bolt
-rwxr-xr-x 1 root root  29M Mar  1 22:53 llvm-c-test
-rwxr-xr-x 1 root root 3.5M Mar  1 22:45 llvm-cat
-rwxr-xr-x 1 root root  12M Mar  1 22:44 llvm-cfi-verify
-rwxr-xr-x 1 root root 222K Mar  1 22:33 llvm-config
-rwxr-xr-x 1 root root 5.2M Mar  1 22:44 llvm-cov
-rwxr-xr-x 1 root root 466K Mar  1 22:43 llvm-cvtres
-rwxr-xr-x 1 root root 4.2M Mar  1 22:43 llvm-cxxdump
-rwxr-xr-x 1 root root 532K Mar  1 22:37 llvm-cxxfilt
-rwxr-xr-x 1 root root 603K Mar  1 22:41 llvm-cxxmap
-rwxr-xr-x 1 root root 422K Mar  1 22:33 llvm-debuginfod-find
-rwxr-xr-x 1 root root 3.3M Mar  1 22:42 llvm-diff
-rwxr-xr-x 1 root root 2.9M Mar  1 22:41 llvm-dis
lrwxrwxrwx 1 root root    7 Mar  1 22:55 llvm-dlltool -> llvm-ar
-rwxr-xr-x 1 root root 7.4M Mar  1 22:43 llvm-dwarfdump
-rwxr-xr-x 1 root root  28M Mar  1 22:53 llvm-dwp
-rwxr-xr-x 1 root root  30M Mar  1 22:53 llvm-exegesis
-rwxr-xr-x 1 root root 6.4M Mar  1 22:49 llvm-extract
-rwxr-xr-x 1 root root  27M Mar  1 22:53 llvm-gsymutil
-rwxr-xr-x 1 root root 4.4M Mar  1 22:43 llvm-ifs
lrwxrwxrwx 1 root root   12 Mar  1 22:55 llvm-install-name-tool -> llvm-objcopy
-rwxr-xr-x 1 root root  21M Mar  1 22:51 llvm-jitlink
lrwxrwxrwx 1 root root    7 Mar  1 22:55 llvm-lib -> llvm-ar
-rwxr-xr-x 1 root root 4.4M Mar  1 22:43 llvm-libtool-darwin
-rwxr-xr-x 1 root root 4.7M Mar  1 22:49 llvm-link
-rwxr-xr-x 1 root root 4.3M Mar  1 22:53 llvm-lipo
-rwxr-xr-x 1 root root  44M Mar  1 22:53 llvm-lto
-rwxr-xr-x 1 root root  52M Mar  1 22:53 llvm-lto2
-rwxr-xr-x 1 root root 6.7M Mar  1 22:39 llvm-mc
-rwxr-xr-x 1 root root 4.7M Mar  1 22:39 llvm-mca
-rwxr-xr-x 1 root root 6.5M Mar  1 22:39 llvm-ml
-rwxr-xr-x 1 root root 3.7M Mar  1 22:45 llvm-modextract
-rwxr-xr-x 1 root root 250K Mar  1 22:37 llvm-mt
-rwxr-xr-x 1 root root 7.7M Mar  1 22:43 llvm-nm
-rwxr-xr-x 1 root root 5.0M Mar  1 22:43 llvm-objcopy
-rwxr-xr-x 1 root root  11M Mar  1 22:44 llvm-objdump
-rwxr-xr-x 1 root root 770K Mar  1 22:43 llvm-opt-report
lrwxrwxrwx 1 root root   12 Mar  1 22:55 llvm-otool -> llvm-objdump
-rwxr-xr-x 1 root root 7.0M Mar  1 22:45 llvm-pdbutil
-rwxr-xr-x 1 root root 5.8M Mar  1 22:44 llvm-profdata
-rwxr-xr-x 1 root root  12M Mar  1 22:49 llvm-profgen
lrwxrwxrwx 1 root root    7 Mar  1 22:55 llvm-ranlib -> llvm-ar
-rwxr-xr-x 1 root root 711K Mar  1 22:43 llvm-rc
lrwxrwxrwx 1 root root   12 Mar  1 22:55 llvm-readelf -> llvm-readobj
-rwxr-xr-x 1 root root 7.5M Mar  1 22:45 llvm-readobj
-rwxr-xr-x 1 root root  29M Mar  1 22:54 llvm-reduce
-rwxr-xr-x 1 root root 6.8M Mar  1 22:45 llvm-rtdyld
-rwxr-xr-x 1 root root 3.4M Mar  1 22:45 llvm-sim
-rwxr-xr-x 1 root root 4.2M Mar  1 22:43 llvm-size
-rwxr-xr-x 1 root root 4.3M Mar  1 22:46 llvm-split
-rwxr-xr-x 1 root root 2.4M Mar  1 22:45 llvm-stress
-rwxr-xr-x 1 root root 372K Mar  1 22:43 llvm-strings
lrwxrwxrwx 1 root root   12 Mar  1 22:55 llvm-strip -> llvm-objcopy
-rwxr-xr-x 1 root root 6.2M Mar  1 22:44 llvm-symbolizer
-rwxr-xr-x 1 root root 4.3M Mar  1 22:43 llvm-tapi-diff
-rwxr-xr-x 1 root root 3.9M Mar  1 22:33 llvm-tblgen
-rwxr-xr-x 1 root root 4.5M Mar  1 22:45 llvm-tli-checker
-rwxr-xr-x 1 root root 505K Mar  1 22:33 llvm-undname
lrwxrwxrwx 1 root root    7 Mar  1 22:55 llvm-windres -> llvm-rc
-rwxr-xr-x 1 root root 6.7M Mar  1 22:44 llvm-xray
-rwxr-xr-x 1 root root 588K Mar  1 22:35 merge-fdata
-rwxr-xr-x 2 root root 4.7M Mar  1 21:59 nm
-rwxr-xr-x 2 root root 5.4M Mar  1 21:59 objcopy
-rwxr-xr-x 2 root root 6.8M Mar  1 21:59 objdump
-rwxr-xr-x 1 root root 657K Mar  1 21:58 openssl
-rwxr-xr-x 1 root root  52M Mar  1 22:54 opt
lrwxrwxrwx 1 root root    9 Mar  1 22:55 perf2bolt -> llvm-bolt
-rwxr-xr-x 1 root root  228 Mar  1 22:26 pip3
-rwxr-xr-x 1 root root  228 Mar  1 22:26 pip3.9
lrwxrwxrwx 1 root root    8 Mar  1 22:26 pydoc3 -> pydoc3.9
-rwxr-xr-x 1 root root   83 Mar  1 22:26 pydoc3.9
lrwxrwxrwx 1 root root    9 Mar  1 22:26 python3 -> python3.9
lrwxrwxrwx 1 root root   16 Mar  1 22:26 python3-config -> python3.9-config
-rwxr-xr-x 1 root root  18M Mar  1 22:26 python3.9
-rwxr-xr-x 1 root root 3.1K Mar  1 22:26 python3.9-config
-rwxr-xr-x 2 root root 4.9M Mar  1 21:59 ranlib
-rwxr-xr-x 2 root root 1.2M Mar  1 21:59 readelf
-rwxr-xr-x 1 root root  11M Mar  1 22:44 sancov
-rwxr-xr-x 1 root root 6.1M Mar  1 22:44 sanstats
-rwxr-xr-x 1 root root  56K Feb  9 07:15 scan-build
-rwxr-xr-x 1 root root  550 Feb  9 07:15 scan-build-py
-rwxr-xr-x 1 root root 4.6K Feb  9 07:15 scan-view
-rwxr-xr-x 1 root root 4.7M Mar  1 21:59 size
-rwxr-xr-x 1 root root 338K Mar  1 22:33 split-file
-rwxr-xr-x 1 root root 4.7M Mar  1 21:59 strings
-rwxr-xr-x 2 root root 5.4M Mar  1 21:59 strip
-rwxr-xr-x 1 root root 4.3M Mar  1 22:45 verify-uselistorder
lrwxrwxrwx 1 root root    3 Mar  1 22:55 wasm-ld -> lld
-rwxr-xr-x 4 root root 4.8M Mar  1 22:25 x86_64-pc-linux-gnu-c++
-rwxr-xr-x 4 root root 4.8M Mar  1 22:25 x86_64-pc-linux-gnu-g++
-rwxr-xr-x 3 root root 4.8M Mar  1 22:25 x86_64-pc-linux-gnu-gcc
-rwxr-xr-x 3 root root 4.8M Mar  1 22:25 x86_64-pc-linux-gnu-gcc-7.5.0
-rwxr-xr-x 2 root root 143K Mar  1 22:25 x86_64-pc-linux-gnu-gcc-ar
-rwxr-xr-x 2 root root 143K Mar  1 22:25 x86_64-pc-linux-gnu-gcc-nm
-rwxr-xr-x 2 root root 143K Mar  1 22:25 x86_64-pc-linux-gnu-gcc-ranlib
+ cd ../..
+ rm -rf llvm-project
Removing intermediate container 95df938d0be2
 ---> f63bc3543a4a
---
Successfully built 35ec7106ad5b
Successfully tagged rust-ci:latest
Built container sha256:35ec7106ad5bc9508658f923a6e2fe96258baa97227c82b1233062fcd2139482
Uploading finished image to https://ci-caches.rust-lang.org/docker/a60c0a5b65bd51a4040a45d5e2af022a5911ea96437e3785c90d79f85efcc5c49e5018540e37be4bf52c360fc17b0613a062756f25b3cc7752a78ebd41a6c433
upload failed: - to s3://rust-lang-ci-sccache2/docker/a60c0a5b65bd51a4040a45d5e2af022a5911ea96437e3785c90d79f85efcc5c49e5018540e37be4bf52c360fc17b0613a062756f25b3cc7752a78ebd41a6c433 Unable to locate credentials
useradd: warning: the home directory already exists.
Not copying any file from skel directory into it.
[CI_JOB_NAME=dist-x86_64-linux]
---
[2022-03-02T00:07:08Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:07:09Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-02T00:07:09Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpStU7sl#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:07:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:07:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpStU7sl#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpStU7sl/incremental-state"
[2022-03-02T00:07:40Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:07:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpStU7sl#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpStU7sl/incremental-state"
[2022-03-02T00:07:43Z DEBUG collector::execute] applying println to "/tmp/.tmpStU7sl"
[2022-03-02T00:07:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-02T00:07:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-02T00:07:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpStU7sl#cargo:0.29.0" "--profile" "check" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpStU7sl/incremental-state"
[2022-03-02T00:07:51Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:07:52Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-02T00:07:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJVt1Di#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:08:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:08:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:08:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJVt1Di#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpJVt1Di/incremental-state"
[2022-03-02T00:08:59Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:08:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJVt1Di#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpJVt1Di/incremental-state"
[2022-03-02T00:09:06Z DEBUG collector::execute] applying println to "/tmp/.tmpJVt1Di"
[2022-03-02T00:09:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-02T00:09:06Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-02T00:09:06Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJVt1Di#cargo:0.29.0" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpJVt1Di/incremental-state"
[2022-03-02T00:09:21Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:09:22Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-02T00:09:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphWI1WK#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:10:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:10:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:10:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphWI1WK#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphWI1WK/incremental-state"
[2022-03-02T00:10:45Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:10:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphWI1WK#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphWI1WK/incremental-state"
[2022-03-02T00:10:52Z DEBUG collector::execute] applying println to "/tmp/.tmphWI1WK"
[2022-03-02T00:10:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-02T00:10:52Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-02T00:10:52Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphWI1WK#cargo:0.29.0" "--release" "--lib" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphWI1WK/incremental-state"
Preparing ctfe-stress-4
[2022-03-02T00:11:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-03-02T00:11:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-02T00:11:12Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-03-02T00:11:13Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:11:13Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-02T00:11:13Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpADsRfW#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:11:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:11:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpADsRfW#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpADsRfW/incremental-state"
[2022-03-02T00:12:02Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:12:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpADsRfW#ctfe-stress-4:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpADsRfW/incremental-state"
[2022-03-02T00:12:02Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:12:02Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-02T00:12:02Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBxnAbO#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:12:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:12:26Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:12:26Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBxnAbO#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpBxnAbO/incremental-state"
[2022-03-02T00:12:53Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:12:53Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBxnAbO#ctfe-stress-4:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpBxnAbO/incremental-state"
[2022-03-02T00:12:54Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:12:54Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-02T00:12:54Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppPGvLs#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:13:16Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:13:16Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:13:16Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppPGvLs#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmppPGvLs/incremental-state"
[2022-03-02T00:13:41Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:13:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmppPGvLs#ctfe-stress-4:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmppPGvLs/incremental-state"
Preparing externs
[2022-03-02T00:13:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-03-02T00:13:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-02T00:13:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-02T00:13:41Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-02T00:13:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpRhYq6l#externs:0.1.0" "--profile" "check" "--" "--skip-this-rustc"
[2022-03-02T00:13:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyNDa09#externs:0.1.0" "--" "--skip-this-rustc"
[2022-03-02T00:13:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpIkb06t#externs:0.1.0" "--release" "--" "--skip-this-rustc"
Running externs: Check + [Full, IncrFull, IncrUnchanged, IncrPatched]
[2022-03-02T00:13:42Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:13:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-02T00:13:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphZfGOJ#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:13:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:13:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphZfGOJ#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphZfGOJ/incremental-state"
[2022-03-02T00:13:43Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:13:43Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphZfGOJ#externs:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphZfGOJ/incremental-state"
[2022-03-02T00:13:44Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:13:44Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-02T00:13:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphJYijF#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:13:45Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:13:45Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:13:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphJYijF#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphJYijF/incremental-state"
[2022-03-02T00:13:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:13:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphJYijF#externs:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmphJYijF/incremental-state"
[2022-03-02T00:13:48Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:13:48Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-02T00:13:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmqGPBb#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:13:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:13:48Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:13:48Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmqGPBb#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpmqGPBb/incremental-state"
[2022-03-02T00:13:50Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:13:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpmqGPBb#externs:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpmqGPBb/incremental-state"
Preparing inflate
[2022-03-02T00:13:51Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-02T00:13:51Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-02T00:13:51Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
---
[2022-03-02T00:13:59Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:13:59Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-02T00:13:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPzOCOg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:14:03Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:14:03Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPzOCOg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpPzOCOg/incremental-state"
[2022-03-02T00:14:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:14:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPzOCOg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpPzOCOg/incremental-state"
[2022-03-02T00:14:07Z DEBUG collector::execute] applying println to "/tmp/.tmpPzOCOg"
[2022-03-02T00:14:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-02T00:14:07Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-02T00:14:07Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPzOCOg#inflate:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpPzOCOg/incremental-state"
[2022-03-02T00:14:11Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:14:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-02T00:14:11Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-02T00:14:11Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSYVyhY#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:14:17Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:14:17Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSYVyhY#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpSYVyhY/incremental-state"
[2022-03-02T00:14:22Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:14:22Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSYVyhY#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpSYVyhY/incremental-state"
[2022-03-02T00:14:23Z DEBUG collector::execute] applying println to "/tmp/.tmpSYVyhY"
[2022-03-02T00:14:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-02T00:14:23Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrPatched), patch=Some(Patch { index: 0, name: PatchName("println"), path: "0-println.patch" })
[2022-03-02T00:14:23Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpSYVyhY#inflate:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpSYVyhY/incremental-state"
Preparing match-stress-enum
[2022-03-02T00:14:29Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
[2022-03-02T00:14:29Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-02T00:14:29Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
---
[2022-03-02T00:14:29Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:14:29Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=Some(Full), patch=None
[2022-03-02T00:14:29Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkYt8gT#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:14:31Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:14:31Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkYt8gT#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpkYt8gT/incremental-state"
[2022-03-02T00:14:33Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Check, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:14:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpkYt8gT#match-stress-enum:0.1.0" "--profile" "check" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpkYt8gT/incremental-state"
[2022-03-02T00:14:33Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:14:33Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-02T00:14:33Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcYaiDR#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:14:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:14:35Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:14:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcYaiDR#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpcYaiDR/incremental-state"
[2022-03-02T00:14:37Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:14:37Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpcYaiDR#match-stress-enum:0.1.0" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpcYaiDR/incremental-state"
[2022-03-02T00:14:38Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:14:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-02T00:14:38Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-02T00:14:38Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuFCHnM#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:14:40Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:14:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuFCHnM#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpuFCHnM/incremental-state"
[2022-03-02T00:14:42Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:14:42Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpuFCHnM#match-stress-enum:0.1.0" "--release" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpuFCHnM/incremental-state"
Preparing token-stream-stress
[2022-03-02T00:14:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=None, patch=None
[2022-03-02T00:14:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=None, patch=None
[2022-03-02T00:14:42Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Check, run_kind=None, patch=None
---
[2022-03-02T00:14:44Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:14:44Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Debug, run_kind=Some(Full), patch=None
[2022-03-02T00:14:44Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAUfE7T#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:14:45Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:14:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAUfE7T#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpAUfE7T/incremental-state"
[2022-03-02T00:14:45Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Debug, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:14:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpAUfE7T#token-stream-stress:0.0.0" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpAUfE7T/incremental-state"
[2022-03-02T00:14:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-03-02T00:14:45Z INFO  collector::execute] run_rustc with incremental=false, build_kind=Opt, run_kind=Some(Full), patch=None
[2022-03-02T00:14:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbGHuQm#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln"
[2022-03-02T00:14:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:14:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrFull), patch=None
[2022-03-02T00:14:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbGHuQm#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpbGHuQm/incremental-state"
[2022-03-02T00:14:46Z INFO  collector::execute] run_rustc with incremental=true, build_kind=Opt, run_kind=Some(IncrUnchanged), patch=None
[2022-03-02T00:14:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpbGHuQm#token-stream-stress:0.0.0" "--release" "--bin" "token-stream-stress" "--" "--wrap-rustc-with" "eprintln" "-C" "incremental=/tmp/.tmpbGHuQm/incremental-state"
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ rm -r ./build/x86_64-unknown-linux-gnu/llvm ./build/x86_64-unknown-linux-gnu/lld
+ python3 ../x.py build --target=x86_64-unknown-linux-gnu --host=x86_64-unknown-linux-gnu --stage 2 library/std --bolt-profile-generate
please file an issue on rust-lang/rust
this is not a problem for non-concurrent x.py invocations
Building rustbuild
    Finished dev [unoptimized] target(s) in 0.17s
    Finished dev [unoptimized] target(s) in 0.17s
Building stage0 std artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
    Finished release [optimized + debuginfo] target(s) in 0.20s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR;M68k" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_INCLUDE_TESTS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_INSTALL_UTILS=ON" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.61.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=clang" "-DCMAKE_CXX_COMPILER=clang++" "-DCMAKE_ASM_COMPILER=clang" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_SHARED_LINKER_FLAGS= -Wl,-q -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_MODULE_LINKER_FLAGS= -Wl,-q -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_EXE_LINKER_FLAGS= -Wl,-q -Wl,-Bsymbolic -static-libstdc++" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 14.0.0
-- The ASM compiler identification is Clang
-- Found assembler: /rustroot/bin/clang
-- Check for working C compiler: /rustroot/bin/clang
---
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./HandleLLVMOptions.cmake
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./LLVM-Build.cmake
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./AddLLVMDefinitions.cmake
-- Installing: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/cmake/llvm/./FindLibpfm.cmake
LLVM paths: /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVM.so, /checkout/obj/build/x86_64-unknown-linux-gnu/llvm/lib/libLLVM-instrumented.so
cargo:root=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm
BOLT-WARNING: debug info will be stripped from the binary. Use -update-debug-sections to keep it.
BOLT-INFO: shared object or position-independent executable detected
BOLT-INFO: Target architecture: x86_64
BOLT-INFO: BOLT version: <unknown>
BOLT-INFO: first alloc address is 0x0
BOLT-INFO: creating new program header table at address 0x5a00000, offset 0x5a00000
BOLT-INFO: enabling relocation mode
BOLT-INFO: forcing -jump-tables=move for instrumentation
BOLT-INFO: enabling -align-macro-fusion=all since no profile was specified
BOLT-INFO: enabling lite mode
BOLT-WARNING: Failed to analyze 5229 relocations
BOLT-WARNING: Ignored 0 functions due to cold fragments.
BOLT-WARNING: 19 collisions detected while hashing binary objects. Use -v=1 to see the list.
BOLT-INSTRUMENTER: Number of indirect call site descriptors: 68972
BOLT-INSTRUMENTER: Number of indirect call target descriptors: 74099
BOLT-INSTRUMENTER: Number of function descriptors: 74099
BOLT-INSTRUMENTER: Number of branch counters: 1523333
BOLT-INSTRUMENTER: Number of ST leaf node counters: 602638
BOLT-INSTRUMENTER: Number of direct call counters: 4969
BOLT-INSTRUMENTER: Total number of counters: 2130940
BOLT-INSTRUMENTER: Total size of counters: 17047520 bytes (static alloc memory)
BOLT-INSTRUMENTER: Total size of string table emitted: 6995854 bytes in file
BOLT-INSTRUMENTER: Total size of descriptors: 128931880 bytes in file
BOLT-INSTRUMENTER: Profile will be saved to file /tmp/prof.fdata
BOLT-INFO: 0 out of 74859 functions in the binary (0.0%) have non-empty execution profile
BOLT-INFO: the input contains 15962 (dynamic count : 0) opportunities for macro-fusion optimization that are going to be fixed
BOLT-INFO: 4948511 instructions were shortened
BOLT-INFO: removed 10459 empty blocks
BOLT-INFO: merged 1 duplicate CFG edge
BOLT-INFO: removed 216 'repz' prefixes with estimated execution count of 0 times.
BOLT-INFO: UCE removed 65330 blocks and 4489397 bytes of code.
BOLT-INFO: SCTC: patched 0 tail calls (0 forward) tail calls (0 backward) from a total of 0 while removing 0 double jumps and removing 0 basic blocks totalling 0 bytes of code. CTCs total execution count is 0 and the number of times CTCs are taken is 0.
BOLT-INFO: output linked against instrumentation runtime library, lib entry point is 0xede32d0
BOLT status: exit status: 0
BOLT-INFO: clear procedure is 0xede13a0
LLVM exists: true, true
Building stage0 compiler artifacts (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu)
   Compiling rustc_llvm v0.0.0 (/checkout/compiler/rustc_llvm)
   Compiling rustc_codegen_llvm v0.0.0 (/checkout/compiler/rustc_codegen_llvm)
   Compiling rustc_interface v0.0.0 (/checkout/compiler/rustc_interface)
---
    Finished release [optimized] target(s) in 6.02s
Dist build-manifest-nightly-x86_64-unknown-linux-gnu
 finished in 1.324 seconds
Build completed successfully in 0:44:52
+ /rustroot/bin/llvm-bolt ./build/x86_64-unknown-linux-gnu/llvm/lib/libLLVM.so -data=/tmp/prof.fdata -o /tmp/libLLVM.so -reorder-blocks=cache+ -reorder-functions=hfsort -split-functions=2 -split-all-cold -split-eh -dyno-stats
/rustroot/bin/llvm-bolt: '/tmp/prof.fdata': No such file or directory.
BOLT-INFO: shared object or position-independent executable detected
