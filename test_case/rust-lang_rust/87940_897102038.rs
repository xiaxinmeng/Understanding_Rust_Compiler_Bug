plain
100 60.8M  100 60.8M    0     0  7067k      0  0:00:08  0:00:08 --:--:-- 6496k
+ cd gcc-8.5.0
+ sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
+ ./contrib/download_prerequisites
2021-08-11 17:41:59 URL:http://gcc.gnu.org/pub/gcc/infrastructure/gmp-6.1.0.tar.bz2 [2383840/2383840] -> "./gmp-6.1.0.tar.bz2" [1]
2021-08-11 17:41:59 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-3.1.4.tar.bz2 [1279284/1279284] -> "./mpfr-3.1.4.tar.bz2" [1]
2021-08-11 17:41:59 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpc-1.0.3.tar.gz [669925/669925] -> "./mpc-1.0.3.tar.gz" [1]
2021-08-11 17:41:59 URL:http://gcc.gnu.org/pub/gcc/infrastructure/isl-0.18.tar.bz2 [1658291/1658291] -> "./isl-0.18.tar.bz2" [1]
gmp-6.1.0.tar.bz2: OK
mpfr-3.1.4.tar.bz2: OK
mpc-1.0.3.tar.gz: OK
isl-0.18.tar.bz2: OK
All prerequisites downloaded successfully.
+ cd ../gcc-build
+ hide_output ../gcc-8.5.0/configure --prefix=/rustroot --enable-languages=c,c++ --disable-gnu-unique-object
+ set +x
++ nproc
---
100  128M    0  128M    0     0  5568k      0 --:--:--  0:00:23 --:--:-- 6924k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;clang-tools-extra;lld;compiler-rt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
Wed Aug 11 18:23:36 UTC 2021 - building ...
Wed Aug 11 18:24:06 UTC 2021 - building ...
++ nproc
+ hide_output make -j16
---
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized + debuginfo] target(s) in 1m 09s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_BUILD_INSTRUMENTED=IR" "-DLLVM_BUILD_RUNTIME=No" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.56.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=clang" "-DCMAKE_CXX_COMPILER=clang++" "-DCMAKE_ASM_COMPILER=clang" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm -static-libstdc++" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 12.0.1
-- The ASM compiler identification is Clang
-- Found assembler: /rustroot/bin/clang
-- Check for working C compiler: /rustroot/bin/clang
---
100  540k  100  540k    0     0  16.5M      0 --:--:-- --:--:-- --:--:-- 16.5M
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib /tmp/externs.rs
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc -Copt-level=3 --edition=2018 --crate-type=lib /tmp/externs.rs
+ local PERF=1e19fc4c6168d2f7596e512f42f358f245d8f09d
+ local github_prefix=https://raw.githubusercontent.com/rust-lang/rustc-perf/1e19fc4c6168d2f7596e512f42f358f245d8f09d
+ local name=ctfe-stress-4
+ local edition=2018
---

warning: 6 warnings emitted

+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc -Copt-level=3 --edition=2018 --crate-type=lib /tmp/ctfe-stress-4.rs
 --> /tmp/ctfe-stress-4.rs:2:34
  |
2 | #![feature(const_fn_trait_bound, const_fn_unsize, const_eval_limit)]
  |                                  ^^^^^^^^^^^^^^^
---

warning: 171 warnings emitted

+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc -Copt-level=3 --edition=2015 --crate-type=lib /tmp/inflate.rs
   --> /tmp/inflate.rs:274:18
    |
274 |                 0...15 => self.result.push(code),
    |                  ^^^ help: use `..=` for an inclusive range
---
   Compiling rustfix v0.6.0
   Compiling git2-curl v0.14.1
    Finished release [optimized] target(s) in 4m 03s
+ ./build/x86_64-unknown-linux-gnu/llvm/bin/llvm-profdata merge -o /tmp/rustc-pgo.profdata /tmp/rustc-pgo
+ /rustroot/bin/llvm-profdata merge -o /tmp/llvm-pgo.profdata ./build/x86_64-unknown-linux-gnu/llvm/profiles
error: ./build/x86_64-unknown-linux-gnu/llvm/profiles: No such file or directory
