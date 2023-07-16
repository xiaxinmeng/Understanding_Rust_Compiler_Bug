plain
100 59.8M  100 59.8M    0     0  6795k      0  0:00:09  0:00:09 --:--:-- 8936k
+ cd gcc-7.5.0
+ sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
+ ./contrib/download_prerequisites
2022-05-18 07:47:51 URL:http://gcc.gnu.org/pub/gcc/infrastructure/gmp-6.1.0.tar.bz2 [2383840/2383840] -> "./gmp-6.1.0.tar.bz2" [1]
2022-05-18 07:47:51 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-3.1.4.tar.bz2 [1279284/1279284] -> "./mpfr-3.1.4.tar.bz2" [1]
2022-05-18 07:47:51 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpc-1.0.3.tar.gz [669925/669925] -> "./mpc-1.0.3.tar.gz" [1]
2022-05-18 07:47:51 URL:http://gcc.gnu.org/pub/gcc/infrastructure/isl-0.16.1.tar.bz2 [1626446/1626446] -> "./isl-0.16.1.tar.bz2" [1]
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
100  150M    0  150M    0     0  5171k      0 --:--:--  0:00:29 --:--:-- 6409k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DCOMPILER_RT_BUILD_XRAY=OFF -DCOMPILER_RT_BUILD_MEMPROF=OFF -DLLVM_TARGETS_TO_BUILD=X86 -DLLVM_INCLUDE_BENCHMARKS=OFF -DLLVM_INCLUDE_TESTS=OFF -DLLVM_INCLUDE_EXAMPLES=OFF '-DLLVM_ENABLE_PROJECTS=clang;lld;compiler-rt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
Wed May 18 08:23:53 UTC 2022 - building ...
++ nproc
+ hide_output make -j16
+ set +x
---
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/await-call-tree/src/lib.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/bitmaps-3.1.0/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/bitmaps-3.1.0/.cargo_vcs_info.json  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/bitmaps-3.1.0/.github/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/bitmaps-3.1.0/.github/dependabot.yml  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/bitmaps-3.1.0/.github/workflows/ci.yml  
 extracting: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/bitmaps-3.1.0/.gitignore  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/bitmaps-3.1.0/0-println.patch  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/bitmaps-3.1.0/CHANGELOG.md  
---
 extracting: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/rustfmt.toml  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/fixed_width_ints.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/fuchsia/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/fuchsia/aarch64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/fuchsia/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/fuchsia/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/fuchsia/no_align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/fuchsia/x86_64.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/hermit/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/hermit/aarch64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/hermit/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/hermit/x86_64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/macros.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/psp.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/psp.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/sgx.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/solid/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/solid/aarch64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/solid/arm.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/solid/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/align.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b32/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b32/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b32/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b64/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b64/aarch64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b64/aarch64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b64/aarch64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b64/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b64/x86_64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b64/x86_64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/b64/x86_64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/apple/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/dragonfly/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/dragonfly/errno.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/dragonfly/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/aarch64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/arm.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd11/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd11/b64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd11/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd12/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd12/b64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd12/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd12/x86_64.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd13/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd13/b64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd13/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd13/x86_64.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd14/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd14/b64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd14/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/freebsd14/x86_64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/powerpc.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/powerpc64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/riscv64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/x86.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/x86_64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/x86_64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/freebsd/x86_64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/freebsdlike/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/netbsd/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/netbsd/aarch64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/netbsd/arm.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/netbsd/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/netbsd/powerpc.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/netbsd/sparc64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/netbsd/x86.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/netbsd/x86_64.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/openbsd/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/openbsd/aarch64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/openbsd/arm.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/openbsd/mips64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/openbsd/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/openbsd/powerpc.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/openbsd/powerpc64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/openbsd/riscv64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/openbsd/sparc64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/openbsd/x86.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/bsd/netbsdlike/openbsd/x86_64.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/haiku/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/haiku/b32.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/haiku/b64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/haiku/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/haiku/native.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/haiku/x86_64.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/hermit/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/hermit/aarch64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/hermit/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/hermit/x86_64.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b32/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b32/arm.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b32/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b32/x86/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b32/x86/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b32/x86/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b64/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b64/aarch64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b64/aarch64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b64/aarch64/int128.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b64/aarch64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b64/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b64/x86_64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b64/x86_64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/b64/x86_64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/android/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/emscripten/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/emscripten/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/emscripten/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/emscripten/no_align.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/align.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/arch/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/arch/generic/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/arch/generic/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/arch/mips/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/arch/mips/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/arch/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/arch/powerpc/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/arch/powerpc/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/arch/sparc/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/arch/sparc/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/align.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/arm/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/arm/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/arm/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/m68k/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/m68k/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/m68k/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/mips/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/mips/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/mips/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/powerpc.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/riscv32/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/riscv32/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/riscv32/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/sparc/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/sparc/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/sparc/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/x86/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/x86/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b32/x86/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/aarch64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/aarch64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/aarch64/ilp32.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/aarch64/int128.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/aarch64/lp64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/aarch64/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/mips64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/mips64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/mips64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/powerpc64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/powerpc64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/powerpc64/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/riscv64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/riscv64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/riscv64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/s390x.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/sparc64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/sparc64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/sparc64/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/x86_64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/x86_64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/x86_64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/x86_64/not_x32.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/b64/x86_64/x32.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/gnu/no_align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/arm/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/arm/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/arm/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/hexagon.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/mips/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/mips/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/mips/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/powerpc.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/x86/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/x86/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b32/x86/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/aarch64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/aarch64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/aarch64/int128.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/aarch64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/mips64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/powerpc64.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/riscv64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/riscv64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/riscv64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/s390x.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/x86_64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/x86_64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/b64/x86_64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/musl/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/no_align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/non_exhaustive.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/align.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/arm/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/arm/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/arm/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/arm/no_align.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/mips/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/mips/mips32/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/mips/mips32/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/mips/mips32/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/mips/mips32/no_align.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/mips/mips64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/mips/mips64/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/mips/mips64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/mips/mips64/no_align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/mips/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/no_align.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/x86_64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/x86_64/l4re.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/x86_64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/linux/uclibc/x86_64/other.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/linux_like/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/aarch64/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/aarch64/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/align.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/arm/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/arm/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/espidf/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/espidf/mod.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/horizon/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/horizon/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/newlib/no_align.rs  
---
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/solarish/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/solarish/compat.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/solarish/illumos.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/solarish/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/solarish/solaris.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/solarish/x86.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/solarish/x86_64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/unix/solarish/x86_common.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/vxworks/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/vxworks/aarch64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/vxworks/arm.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/vxworks/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/vxworks/powerpc.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/vxworks/powerpc64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/vxworks/x86.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/vxworks/x86_64.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/wasi.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/windows/gnu/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/windows/gnu/align.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/windows/gnu/mod.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/libc-0.2.124/src/windows/mod.rs  
---
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tokio-webpush-simple/native-tls-0.1.5/src/test.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tokio-webpush-simple/perf-config.json  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tokio-webpush-simple/src/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tokio-webpush-simple/src/main.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/Cargo.lock  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/Cargo.toml  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/perf-config.json  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/.cargo_vcs_info.json  
 extracting: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/.clippy.toml  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/.github/
 extracting: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/.github/FUNDING.yml  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/.github/workflows/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/.github/workflows/ci.yml  
 extracting: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/.gitignore  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/Cargo.toml  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/Cargo.toml.orig  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/LICENSE-APACHE  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/LICENSE-MIT  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/README.md  
 extracting: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/perf-config.json  
 extracting: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/rust-toolchain.toml  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/src/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/src/ext.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/src/format.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/src/ident_fragment.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/src/lib.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/src/runtime.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/src/spanned.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/src/to_tokens.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/compiletest.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/test.rs  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/does-not-have-iter-interpolated-dup.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/does-not-have-iter-interpolated-dup.stderr  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/does-not-have-iter-interpolated.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/does-not-have-iter-interpolated.stderr  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/does-not-have-iter-separated.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/does-not-have-iter-separated.stderr  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/does-not-have-iter.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/does-not-have-iter.stderr  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/not-quotable.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/not-quotable.stderr  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/not-repeatable.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/not-repeatable.stderr  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/wrong-type-span.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/quote-1.0.17-modified/tests/ui/wrong-type-span.stderr  
   creating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/src/
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tt-muncher/src/main.rs  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tuple-stress/0-new-row.patch  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tuple-stress/Cargo.lock  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tuple-stress/Cargo.toml  
  inflating: rustc-perf-f66cc8f3e04392b0e2fd811f21fd1ece6ebaded3/collector/benchmarks/tuple-stress/perf-config.json  
---
[2022-05-18T09:16:22Z DEBUG collector] benchmark `hyper-0.14.18`- registered
[2022-05-18T09:16:22Z DEBUG collector] benchmark `regex-1.5.5`- registered
[2022-05-18T09:16:22Z DEBUG collector] benchmark `serde-1.0.136`- registered
[2022-05-18T09:16:22Z DEBUG collector] benchmark `cargo-0.60.0`- registered
Running with 1 job(s)
Executing benchmark cargo-0.60.0 (1/7)
Preparing cargo-0.60.0
[2022-05-18T09:16:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T09:16:23Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
---
Executing benchmark hyper-0.14.18 (3/7)
Preparing hyper-0.14.18
[2022-05-18T09:21:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T09:21:04Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T09:21:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpQgOjxC#hyper:0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-18T09:21:04Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpNjGFqb#hyper:0.14.18" "--features=client,http1,http2,server,stream" "--" "--skip-this-rustc"
[2022-05-18T09:21:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T09:21:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T09:21:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T09:21:41Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpBN8sXB#hyper:0.14.18" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-18T09:21:45Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T09:21:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T09:21:46Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T09:21:46Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpyCQfcR#hyper:0.14.18" "--release" "--features=client,http1,http2,server,stream" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark hyper-0.14.18 (3/7)
Preparing regex-1.5.5
[2022-05-18T09:21:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T09:21:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T09:21:59Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T09:21:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpXRseJs#regex:1.5.5" "--" "--skip-this-rustc"
[2022-05-18T09:21:59Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpJv0ttg#regex:1.5.5" "--release" "--" "--skip-this-rustc"
[2022-05-18T09:22:12Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T09:22:12Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T09:22:12Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpwz3NYn#regex:1.5.5" "--" "--wrap-rustc-with" "Eprintln"
Running regex-1.5.5: Opt + [Full]
---
[2022-05-18T09:23:35Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpU9y3VD#serde:1.0.136" "--" "--skip-this-rustc"
Running serde-1.0.136: Debug + [Full]
[2022-05-18T09:23:36Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T09:23:36Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T09:23:36Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmphtQWBZ#serde:1.0.136" "--" "--wrap-rustc-with" "Eprintln"
[2022-05-18T09:23:40Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T09:23:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T09:23:40Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=Some(Full), patch=None
[2022-05-18T09:23:40Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpZNoOzB#serde:1.0.136" "--release" "--" "--wrap-rustc-with" "Eprintln"
Finished benchmark serde-1.0.136 (6/7)
Preparing syn-1.0.89
[2022-05-18T09:23:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Opt, scenario=None, patch=None
[2022-05-18T09:23:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T09:23:45Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=None, patch=None
[2022-05-18T09:23:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmprBJHkx#syn:1.0.89" "--release" "--" "--skip-this-rustc"
[2022-05-18T09:23:45Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpUaSy48#syn:1.0.89" "--" "--skip-this-rustc"
[2022-05-18T09:23:50Z DEBUG collector::execute] Benchmark iteration 1/1
[2022-05-18T09:23:50Z INFO  collector::execute] run_rustc with incremental=false, profile=Debug, scenario=Some(Full), patch=None
[2022-05-18T09:23:50Z DEBUG collector::execute] "/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo" "rustc" "--manifest-path" "Cargo.toml" "-p" "file:///tmp/.tmpPEX74l#syn:1.0.89" "--" "--wrap-rustc-with" "Eprintln"
Running syn-1.0.89: Opt + [Full]
---
Copying stage2 std from stage1 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
[TIMING] compile::StdLink { compiler: Compiler { stage: 1, host: x86_64-unknown-linux-gnu }, target_compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu }, target: x86_64-unknown-linux-gnu } -- 0.001
[TIMING] compile::Std { target: x86_64-unknown-linux-gnu, compiler: Compiler { stage: 2, host: x86_64-unknown-linux-gnu } } -- 0.000
Build completed successfully in 0:13:41
+ gather_profiles Check,Debug,Opt All externs,ctfe-stress-4,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2021 --crate-type=lib ../library/core/src/lib.rs
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2021 --crate-type=lib -Copt-level=3 ../library/core/src/lib.rs
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2021 --crate-type=lib -Copt-level=3 ../library/core/src/lib.rs
+ cd rustc-perf
+ RUST_LOG=collector=debug
+ RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc
+ RUSTC_BOOTSTRAP=1
+ /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --profiles Check,Debug,Opt --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --scenarios All --include externs,ctfe-stress-4,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0
    Finished dev [unoptimized + debuginfo] target(s) in 0.16s
     Running `target/debug/collector profile_local eprintln /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc --id Test --profiles Check,Debug,Opt --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --scenarios All --include externs,ctfe-stress-4,cargo-0.60.0,token-stream-stress,match-stress,tuple-stress,diesel-1.4.8,bitmaps-3.1.0`
[2022-05-18T09:39:21Z DEBUG collector] benchmark README.md - ignored
[2022-05-18T09:39:21Z DEBUG collector] benchmark `bitmaps-3.1.0`- registered
[2022-05-18T09:39:21Z DEBUG collector] benchmark `token-stream-stress`- registered
[2022-05-18T09:39:21Z DEBUG collector] benchmark `match-stress`- registered
[2022-05-18T09:39:21Z DEBUG collector] benchmark `match-stress`- registered
[2022-05-18T09:39:21Z DEBUG collector] benchmark `diesel-1.4.8`- registered
[2022-05-18T09:39:21Z DEBUG collector] benchmark `externs`- registered
[2022-05-18T09:39:21Z DEBUG collector] benchmark `tuple-stress`- registered
[2022-05-18T09:39:21Z DEBUG collector] benchmark `cargo-0.60.0`- registered
collector error: Warning: one or more unused --include entries: ["ctfe-stress-4"]
