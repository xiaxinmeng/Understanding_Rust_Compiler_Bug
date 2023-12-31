plain
100 60.8M  100 60.8M    0     0  6713k      0  0:00:09  0:00:09 --:--:-- 6537k
+ cd gcc-8.5.0
+ sed -i 's|ftp://gcc\.gnu\.org/|http://gcc.gnu.org/|g' ./contrib/download_prerequisites
+ ./contrib/download_prerequisites
2021-08-12 13:34:45 URL:http://gcc.gnu.org/pub/gcc/infrastructure/gmp-6.1.0.tar.bz2 [2383840/2383840] -> "./gmp-6.1.0.tar.bz2" [1]
2021-08-12 13:34:46 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpfr-3.1.4.tar.bz2 [1279284/1279284] -> "./mpfr-3.1.4.tar.bz2" [1]
2021-08-12 13:34:47 URL:http://gcc.gnu.org/pub/gcc/infrastructure/mpc-1.0.3.tar.gz [669925/669925] -> "./mpc-1.0.3.tar.gz" [1]
2021-08-12 13:34:47 URL:http://gcc.gnu.org/pub/gcc/infrastructure/isl-0.18.tar.bz2 [1658291/1658291] -> "./isl-0.18.tar.bz2" [1]
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
100  128M    0  128M    0     0  5477k      0 --:--:--  0:00:23 --:--:-- 6183k
+ mkdir clang-build
+ cd clang-build
+ INC=/rustroot/include:/usr/include
+ hide_output cmake ../llvm -DCMAKE_C_COMPILER=/rustroot/bin/gcc -DCMAKE_CXX_COMPILER=/rustroot/bin/g++ -DCMAKE_BUILD_TYPE=Release -DCMAKE_INSTALL_PREFIX=/rustroot -DCOMPILER_RT_BUILD_SANITIZERS=OFF -DLLVM_TARGETS_TO_BUILD=X86 '-DLLVM_ENABLE_PROJECTS=clang;clang-tools-extra;lld;compiler-rt' -DC_INCLUDE_DIRS=/rustroot/include:/usr/include
Thu Aug 12 14:14:23 UTC 2021 - building ...
Thu Aug 12 14:14:53 UTC 2021 - building ...
++ nproc
+ hide_output make -j16
---
Step 25/38 : ENV CC=clang CXX=clang++
 ---> Running in 6c3ec002ea76
Removing intermediate container 6c3ec002ea76
 ---> 2c1fef1b8195
Step 26/38 : ENV PERF_COMMIT 1e19fc4c6168d2f7596e512f42f358f245d8f09d
Removing intermediate container ba4fefd368d1
 ---> 4ae2b0e184c0
 ---> 4ae2b0e184c0
Step 27/38 : RUN curl -LS -o perf.zip https://github.com/rust-lang/rustc-perf/archive/$PERF_COMMIT.zip
  % Total    % Received % Xferd  Average Speed   Time    Time     Time  Current
                                 Dload  Upload   Total   Spent    Left  Speed

  0     0    0     0    0     0      0      0 --:--:-- --:--:-- --:--:--     0
---
100 31.0M    0 31.0M    0     0  8397k      0 --:--:--  0:00:03 --:--:-- 9510k
100 33.5M    0 33.5M    0     0  8876k      0 --:--:--  0:00:03 --:--:--  9.8M
Removing intermediate container 8822d8a83693
 ---> 4f48b996aa73
Step 28/38 : RUN unzip perf.zip && mv rustc-perf-$PERF_COMMIT rustc-perf
Archive:  perf.zip
1e19fc4c6168d2f7596e512f42f358f245d8f09d
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/.dockerignore  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/.github/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/.github/workflows/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/.github/workflows/ci.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/.gitignore  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/Cargo.lock  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/Cargo.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/Dockerfile  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/README.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/ci/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/ci/check-benchmarks.sh  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/ci/check-profiling.sh  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/ci/check-triage-script.sh  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/
 extracting: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/.gitignore  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/Cargo.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/LICENSE  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/README.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/LICENSE.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/README.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/await-call-tree/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/await-call-tree/Cargo.lock  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/await-call-tree/Cargo.toml  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/await-call-tree/src/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/await-call-tree/src/lib.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/.gitignore  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/.travis.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/0-println.patch  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/ARCHITECTURE.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/CONTRIBUTING.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/Cargo.lock  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/Cargo.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/LICENSE-APACHE  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/LICENSE-MIT  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/LICENSE-THIRD-PARTY  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/README.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/appveyor.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/perf-config.json  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/cli.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/command_prelude.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/bench.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/build.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/check.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/clean.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/doc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/fetch.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/generate_lockfile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/git_checkout.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/init.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/install.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/locate_project.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/login.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/metadata.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/new.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/owner.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/package.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/pkgid.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/publish.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/read_manifest.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/run.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/rustc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/rustdoc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/search.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/test.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/uninstall.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/update.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/verify_project.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/version.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/commands/yank.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/bin/cargo/main.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/build_config.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/build_context/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/build_context/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/build_context/target_info.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/build_plan.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/compilation.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/context/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/context/compilation_files.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/context/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/context/unit_dependencies.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/custom_build.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/fingerprint.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/job.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/job_queue.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/layout.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/compiler/output_depinfo.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/dependency.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/features.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/interning.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/manifest.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/package.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/package_id.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/package_id_spec.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/profiles.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/registry.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/conflict_cache.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/context.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/encode.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/resolve.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/resolver/types.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/shell.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/source/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/source/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/source/source_id.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/summary.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/core/workspace.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/lib.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_clean.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_compile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_doc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_fetch.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_generate_lockfile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_install.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_new.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_output_metadata.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_package.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_pkgid.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_read_manifest.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_run.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/cargo_test.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/lockfile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/registry.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/ops/resolve.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/directory.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/git/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/git/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/git/source.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/git/utils.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/path.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/registry/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/registry/index.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/registry/local.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/registry/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/registry/remote.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/sources/replaced.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/cfg.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/dependency_queue.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/errors.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/flock.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/graph.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/hex.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/important_paths.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/job.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/lev_distance.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/machine_message.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/network.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/paths.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/process_builder.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/profile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/progress.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/read2.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/rustc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/sha256.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/to_semver.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/to_url.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/toml/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/toml/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/toml/targets.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/cargo/util/vcs.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/crates-io/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/crates-io/Cargo.toml  
    linking: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/crates-io/LICENSE-APACHE  -> ../../LICENSE-APACHE 
    linking: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/crates-io/LICENSE-MIT  -> ../../LICENSE-MIT 
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/crates-io/lib.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/README.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/book.toml  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/SUMMARY.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/faq.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/getting-started/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/getting-started/first-steps.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/getting-started/index.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/getting-started/installation.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/build-cache.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/cargo-toml-vs-cargo-lock.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/continuous-integration.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/creating-a-new-project.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/dependencies.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/index.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/project-layout.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/tests.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/why-cargo-exists.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/guide/working-on-an-existing-project.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/images/
 extracting: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/images/Cargo-Logo-Small.png  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/images/auth-level-acl.png  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/images/org-level-acl.png  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/index.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/build-scripts.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/config.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/environment-variables.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/external-tools.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/index.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/manifest.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/pkgid-spec.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/publishing.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/source-replacement.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/specifying-dependencies.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/src/reference/unstable.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/theme/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/doc/theme/favicon.png  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/_cargo  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/cargo.bashcomp.sh  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-bench.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-build.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-check.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-clean.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-doc.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-fetch.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-generate-lockfile.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-init.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-install.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-login.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-metadata.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-new.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-owner.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-package.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-pkgid.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-publish.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-run.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-rustc.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-rustdoc.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-search.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-test.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-uninstall.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-update.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-version.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo-yank.1  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/src/etc/man/cargo.1  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/alt_registry.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/bad_config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/bad_manifest_path.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/bench.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build_auth.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build_lib.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build_plan.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build_script.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/build_script_env.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargo_alias_config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargo_command.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargo_features.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/install.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/mod.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/cross_compile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/git.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/paths.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/publish.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cargotest/support/registry.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cfg.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/check-style.sh  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/check.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/clean.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/concurrent.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/corrupt_git.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cross_compile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/cross_publish.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/custom_target.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/death.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/dep_info.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/directory.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/doc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/features.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/fetch.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/freshness.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/generate_lockfile.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/git.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/hamcrest.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/init.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/install.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/jobserver.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/local_registry.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/lockfile_compat.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/login.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/main.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/metadata.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/net_config.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/new.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/out_dir.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/overrides.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/package.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/patch.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/path.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/plugins.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/proc_macro.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/profile_overrides.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/profile_targets.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/profiles.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/publish.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/read_manifest.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/registry.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rename_deps.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/required_features.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/resolve.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/run.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rustc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rustc_info_cache.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rustdoc.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rustdocflags.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/rustflags.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/search.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/small_fd_limits.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/test.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/tool_paths.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/update.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/verify_project.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/version.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/warn_on_failure.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cargo/tests/testsuite/workspaces.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/.appveyor.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/.clog.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/.gitignore  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/.mention-bot  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/.travis.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/0-println.patch  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/CHANGELOG.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/CONTRIBUTORS.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/Cargo.lock  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/Cargo.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/LICENSE-MIT  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/README.md  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/SPONSORS.md  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/01_default.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/02_simple.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/03_complex.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/04_new_help.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/05_ripgrep.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/benches/06_rustup.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap-perf/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap-perf/clap_perf.dat  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap-perf/clap_perf.png  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap-perf/plot_perf.gp  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap-test.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap_dep_graph.dot  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/clap_dep_graph.png  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/01a_quick_example.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/01b_quick_example.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/01c_quick_example.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/02_apps.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/03_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/04_using_matches.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/05_flag_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/06_positional_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/07_option_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/08_subcommands.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/09_auto_version.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/10_default_values.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/11_only_specific_values.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/12_typed_values.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/13a_enum_values_automatic.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/13b_enum_values_manual.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/14_groups.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/15_custom_validator.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/16_app_settings.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/17_yaml.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/17_yaml.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/18_builder_macro.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/19_auto_authors.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/20_subcommands.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/21_aliases.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/examples/22_stop_parsing_with_--.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/index.html  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/justfile  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/rustfmt.toml  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/help.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/macros.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/meta.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/parser.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/settings.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/usage.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/app/validator.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/any_arg.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/base.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/flag.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/option.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/positional.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/switched.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_builder/valued.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_matcher.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/arg_matches.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/group.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/macros.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/matched_arg.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/settings.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/args/subcommand.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/bash.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/fish.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/macros.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/mod.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/powershell.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/shell.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/completions/zsh.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/errors.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/fmt.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/lib.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/macros.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/map.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/osstringext.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/strext.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/suggestions.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/src/usage_parser.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/app.yml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/app_settings.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/arg_aliases.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/borrowed.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/completions.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/conflicts.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/default_vals.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/delimiters.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/derive_order.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/env.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/example1_tmpl_full.txt  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/example1_tmpl_simple.txt  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/flags.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/global_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/groups.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/help.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/hidden_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/macros.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/multiple_occurrences.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/multiple_values.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/opts.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/positionals.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/posix_compatible.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/possible_values.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/propagate_globals.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/require.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/subcommands.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/template_help.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/tests.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/unique_args.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/utf8.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/version-numbers.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/version.rs  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/clap-rs/tests/yaml.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/0-println.patch  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/1-add-static-arr-item.patch  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/Cargo.lock  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/Cargo.toml  
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/README  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/src/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/coercions/src/main.rs  
   creating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cranelift-codegen/
  inflating: rustc-perf-1e19fc4c6168d2f7596e512f42f358f245d8f09d/collector/benchmarks/cranelift-codegen/.gitignore  
---
   Compiling test v0.0.0 (/checkout/library/test)
    Finished release [optimized + debuginfo] target(s) in 1m 08s
Copying stage0 std from stage0 (x86_64-unknown-linux-gnu -> x86_64-unknown-linux-gnu / x86_64-unknown-linux-gnu)
Building LLVM for x86_64-unknown-linux-gnu
running: "cmake" "/checkout/src/llvm-project/llvm" "-DLLVM_ENABLE_ASSERTIONS=OFF" "-DLLVM_ENABLE_PLUGINS=OFF" "-DLLVM_TARGETS_TO_BUILD=AArch64;ARM;BPF;Hexagon;MSP430;Mips;NVPTX;PowerPC;RISCV;Sparc;SystemZ;WebAssembly;X86" "-DLLVM_EXPERIMENTAL_TARGETS_TO_BUILD=AVR" "-DLLVM_INCLUDE_EXAMPLES=OFF" "-DLLVM_INCLUDE_DOCS=OFF" "-DLLVM_INCLUDE_BENCHMARKS=OFF" "-DLLVM_ENABLE_TERMINFO=OFF" "-DLLVM_ENABLE_LIBEDIT=OFF" "-DLLVM_ENABLE_BINDINGS=OFF" "-DLLVM_ENABLE_Z3_SOLVER=OFF" "-DLLVM_PARALLEL_COMPILE_JOBS=16" "-DLLVM_TARGET_ARCH=x86_64" "-DLLVM_DEFAULT_TARGET_TRIPLE=x86_64-unknown-linux-gnu" "-DLLVM_BUILD_INSTRUMENTED=IR" "-DLLVM_BUILD_RUNTIME=No" "-DLLVM_ENABLE_ZLIB=ON" "-DLLVM_ENABLE_LTO=Thin" "-DLLVM_ENABLE_LLD=ON" "-DLLVM_LINK_LLVM_DYLIB=ON" "-DCMAKE_EXE_LINKER_FLAGS=-Wl,-Bsymbolic -static-libstdc++" "-DLLVM_ENABLE_LIBXML2=OFF" "-DLLVM_VERSION_SUFFIX=-rust-1.56.0-nightly" "-DCMAKE_INSTALL_MESSAGE=LAZY" "-DCMAKE_C_COMPILER_LAUNCHER=sccache" "-DCMAKE_CXX_COMPILER_LAUNCHER=sccache" "-DCMAKE_C_COMPILER=clang" "-DCMAKE_CXX_COMPILER=clang++" "-DCMAKE_ASM_COMPILER=clang" "-DCMAKE_C_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm" "-DCMAKE_CXX_FLAGS=-ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu -fdebug-prefix-map=/checkout=/rustc/llvm -static-libstdc++" "-DCMAKE_AR=/rustroot/bin/llvm-ar" "-DCMAKE_INSTALL_PREFIX=/checkout/obj/build/x86_64-unknown-linux-gnu/llvm" "-DCMAKE_ASM_FLAGS= -ffunction-sections -fdata-sections -fPIC --target=x86_64-unknown-linux-gnu" "-DCMAKE_BUILD_TYPE=Release"
-- The CXX compiler identification is Clang 12.0.1
-- The ASM compiler identification is Clang
-- Found assembler: /rustroot/bin/clang
-- Check for working C compiler: /rustroot/bin/clang
---
Build completed successfully in 0:21:33
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib ../library/core/src/lib.rs
+ RUSTC_BOOTSTRAP=1
+ ./build/x86_64-unknown-linux-gnu/stage2/bin/rustc --edition=2018 --crate-type=lib -Copt-level=3 ../library/core/src/lib.rs
+ cd /tmp/rustc-perf
+ RUST_LOG=collector=debug
+ RUSTC=/checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/rustc
+ RUSTC_BOOTSTRAP=1
+ /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo run -p collector --bin collector -- bench_local /checkout/obj/build/x86_64-unknown-linux-gnu/stage2/bin/rustc Test --builds Check,Debug,Opt --cargo /checkout/obj/build/x86_64-unknown-linux-gnu/stage0/bin/cargo --runs All --include externs,ctfe-stress-4,inflate,cargo
error: could not execute process `rustc -vV` (never executed)
Caused by:
Caused by:
  No such file or directory (os error 2)
